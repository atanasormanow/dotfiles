use crate::actions::{self, LinkResult};
use crate::dotfile::{Dotfile, DotfileManager, LinkStatus};
use anyhow::Result;
use std::process::Command;

/// Current view/mode of the application
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum View {
    /// Main list view
    List,
    /// Help screen
    Help,
    /// Confirmation dialog
    Confirm(ConfirmAction),
    /// Input dialog
    Input(InputMode),
    /// Message/error display
    Message(String),
    /// Distribute (batch link) multi-select
    Distribute,
}

/// Actions that require confirmation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfirmAction {
    Unlink(usize),
    Remove(usize),
    Unmanage(usize),
    ForceLink(usize),
    ReplaceAdd { source: String, name: String },
    DistributeConflicts { conflict_count: usize },
}

/// Input modes for the input dialog
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputMode {
    AddDotfileSource,
    AddDotfileName { source: String },
    EditDestination(usize),
    Search,
    Rename(usize),
}

/// Application state
pub struct App {
    /// Whether the app should quit
    pub should_quit: bool,
    /// Current view
    pub view: View,
    /// Dotfile manager
    pub manager: DotfileManager,
    /// List of all dotfiles
    pub dotfiles: Vec<Dotfile>,
    /// Currently selected index in the list
    pub selected: usize,
    /// Current input buffer
    pub input: String,
    /// Status message to display
    pub status_message: Option<String>,
    /// Search filter
    pub filter: String,
    /// Filtered indices (indices into dotfiles vec)
    pub filtered_indices: Vec<usize>,
    /// Selected items for distribute (indices into dotfiles vec)
    pub distribute_selected: Vec<usize>,
    /// Indices of linkable dotfiles shown in distribute view (not already linked)
    pub distribute_indices: Vec<usize>,
    /// Cursor position in distribute view (index into distribute_indices)
    pub distribute_cursor: usize,
    /// Flag to open editor (handled in main loop)
    pub pending_editor: bool,
    /// Current path completion candidates
    pub completion_candidates: Vec<std::path::PathBuf>,
    /// Index of currently selected completion (None = no completion active)
    pub completion_index: Option<usize>,
    /// Input at the time completion was triggered (to detect changes)
    pub completion_base: String,
}

impl App {
    /// Create a new App instance
    pub fn new() -> Result<Self> {
        let manager = DotfileManager::new()?;
        let dotfiles = manager.discover()?;
        let filtered_indices: Vec<usize> = (0..dotfiles.len()).collect();

        Ok(Self {
            should_quit: false,
            view: View::List,
            manager,
            dotfiles,
            selected: 0,
            input: String::new(),
            status_message: None,
            filter: String::new(),
            filtered_indices,
            distribute_selected: Vec::new(),
            distribute_indices: Vec::new(),
            distribute_cursor: 0,
            pending_editor: false,
            completion_candidates: Vec::new(),
            completion_index: None,
            completion_base: String::new(),
        })
    }

    /// Refresh the dotfile list
    pub fn refresh(&mut self) -> Result<()> {
        self.dotfiles = self.manager.discover()?;
        // Git status is refreshed during discover
        self.apply_filter();
        // Ensure selected is in bounds
        if !self.filtered_indices.is_empty() && self.selected >= self.filtered_indices.len() {
            self.selected = self.filtered_indices.len() - 1;
        }
        self.status_message = Some("Refreshed".to_string());
        Ok(())
    }

    /// Refresh with error handling - shows error message dialog on failure
    fn refresh_or_show_error(&mut self) {
        if let Err(e) = self.refresh() {
            self.view = View::Message(format!("Refresh failed: {}", e));
        }
    }

    /// Show a message dialog
    pub fn show_message(&mut self, message: String) {
        self.view = View::Message(message);
    }

    /// Refresh link and git status for a single dotfile by index
    fn refresh_dotfile_status(&mut self, idx: usize) {
        self.dotfiles[idx].refresh_status();
        let repo_root = self.manager.repo_root().to_path_buf();
        self.dotfiles[idx].refresh_git_status(&repo_root);
    }

    /// Apply the current filter to the dotfile list
    fn apply_filter(&mut self) {
        if self.filter.is_empty() {
            self.filtered_indices = (0..self.dotfiles.len()).collect();
        } else {
            let filter_lower = self.filter.to_lowercase();
            self.filtered_indices = self
                .dotfiles
                .iter()
                .enumerate()
                .filter(|(_, d)| d.name.to_lowercase().contains(&filter_lower))
                .map(|(i, _)| i)
                .collect();
        }
    }

    /// Get the currently selected dotfile
    pub fn selected_dotfile(&self) -> Option<&Dotfile> {
        self.filtered_indices
            .get(self.selected)
            .and_then(|&i| self.dotfiles.get(i))
    }

    /// Get the actual index of the selected dotfile
    pub fn selected_index(&self) -> Option<usize> {
        self.filtered_indices.get(self.selected).copied()
    }

    /// Move selection up
    pub fn select_prev(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    /// Move selection down
    pub fn select_next(&mut self) {
        if !self.filtered_indices.is_empty() && self.selected < self.filtered_indices.len() - 1 {
            self.selected += 1;
        }
    }

    /// Move to first item
    pub fn select_first(&mut self) {
        self.selected = 0;
    }

    /// Move to last item
    pub fn select_last(&mut self) {
        if !self.filtered_indices.is_empty() {
            self.selected = self.filtered_indices.len() - 1;
        }
    }

    /// Request to open selected dotfile in $EDITOR
    pub fn open_in_editor(&mut self) {
        if self.selected_dotfile().is_some() {
            self.pending_editor = true;
        }
    }

    /// Execute the editor opening (called from main loop with terminal access)
    pub fn do_open_editor(&mut self) {
        self.pending_editor = false;

        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());

        if let Some(dotfile) = self.selected_dotfile() {
            let path = dotfile.source_file.clone();
            let name = dotfile.name.clone();

            // Use shell to handle $EDITOR that may contain arguments
            let path_str = path.to_string_lossy();
            let shell_cmd = format!("{} \"{}\"", editor, path_str);

            match Command::new("sh").arg("-c").arg(&shell_cmd).status() {
                Ok(status) => {
                    if status.success() {
                        self.status_message = Some(format!("Edited '{}'", name));
                        // Refresh in case file was modified
                        self.refresh_or_show_error();
                    } else {
                        self.status_message = Some("Editor exited with error".to_string());
                    }
                }
                Err(e) => {
                    self.status_message = Some(format!("Failed to open editor: {}", e));
                }
            }
        }
    }

    /// Link the selected dotfile
    pub fn link_selected(&mut self) {
        if let Some(idx) = self.selected_index() {
            let dotfile = &self.dotfiles[idx];

            // Check if force link is needed
            if matches!(dotfile.link_status, LinkStatus::Conflict) {
                self.view = View::Confirm(ConfirmAction::ForceLink(idx));
                return;
            }

            match actions::link_dotfile(dotfile) {
                Ok(LinkResult::Success) => {
                    let name = self.dotfiles[idx].name.clone();
                    self.status_message = Some(format!("Linked '{}'", name));
                    self.refresh_dotfile_status(idx);
                }
                Ok(LinkResult::AlreadyLinked) => {
                    self.status_message = Some(format!("'{}' is already linked", dotfile.name));
                }
                Ok(LinkResult::Conflict) => {
                    self.view = View::Confirm(ConfirmAction::ForceLink(idx));
                }
                Ok(LinkResult::Error(e)) => {
                    self.status_message = Some(format!("Error: {}", e));
                }
                Err(e) => {
                    self.status_message = Some(format!("Error: {}", e));
                }
            }
        }
    }

    /// Unlink the selected dotfile (with confirmation)
    pub fn unlink_selected(&mut self) {
        if let Some(idx) = self.selected_index() {
            let dotfile = &self.dotfiles[idx];
            if matches!(dotfile.link_status, LinkStatus::Linked | LinkStatus::Broken) {
                self.view = View::Confirm(ConfirmAction::Unlink(idx));
            } else {
                self.status_message = Some(format!("'{}' is not linked", dotfile.name));
            }
        }
    }

    /// Execute confirmed unlink
    pub fn confirm_unlink(&mut self, idx: usize) {
        if let Some(dotfile) = self.dotfiles.get(idx) {
            let name = dotfile.name.clone();
            match actions::unlink_dotfile(dotfile) {
                Ok(()) => {
                    self.status_message = Some(format!("Unlinked '{}'", name));
                    self.refresh_dotfile_status(idx);
                }
                Err(e) => {
                    self.status_message = Some(format!("Error: {}", e));
                }
            }
        }
        self.view = View::List;
    }

    /// Execute confirmed force link
    pub fn confirm_force_link(&mut self, idx: usize) {
        if let Some(dotfile) = self.dotfiles.get(idx) {
            let name = dotfile.name.clone();
            match actions::force_link_dotfile(dotfile) {
                Ok(LinkResult::Success) => {
                    self.status_message = Some(format!("Force linked '{}'", name));
                    self.refresh_dotfile_status(idx);
                }
                Ok(_) | Err(_) => {
                    self.status_message = Some("Failed to force link".to_string());
                }
            }
        }
        self.view = View::List;
    }

    /// Start add dotfile flow
    pub fn start_add(&mut self) {
        self.input.clear();
        self.view = View::Input(InputMode::AddDotfileSource);
    }

    /// Start remove dotfile flow (with confirmation)
    pub fn remove_selected(&mut self) {
        if let Some(idx) = self.selected_index() {
            self.view = View::Confirm(ConfirmAction::Remove(idx));
        }
    }

    /// Execute confirmed remove
    pub fn confirm_remove(&mut self, idx: usize) {
        if let Some(dotfile) = self.dotfiles.get(idx) {
            let name = dotfile.name.clone();
            // Don't restore - just remove from repo (user can restore manually if needed)
            match actions::remove_dotfile(dotfile, false) {
                Ok(()) => {
                    self.status_message = Some(format!("Removed '{}'", name));
                    self.refresh_or_show_error();
                }
                Err(e) => {
                    self.status_message = Some(format!("Error: {}", e));
                }
            }
        }
        self.view = View::List;
    }

    /// Start unmanage dotfile flow (with confirmation)
    pub fn unmanage_selected(&mut self) {
        if let Some(idx) = self.selected_index() {
            self.view = View::Confirm(ConfirmAction::Unmanage(idx));
        }
    }

    /// Execute confirmed unmanage
    pub fn confirm_unmanage(&mut self, idx: usize) {
        if let Some(dotfile) = self.dotfiles.get(idx) {
            let name = dotfile.name.clone();
            let dest = dotfile.dest_expanded.display().to_string();
            match actions::unmanage_dotfile(dotfile) {
                Ok(()) => {
                    self.status_message = Some(format!("Unmanaged '{}' -> {}", name, dest));
                    self.refresh_or_show_error();
                }
                Err(e) => {
                    self.status_message = Some(format!("Error: {}", e));
                }
            }
        }
        self.view = View::List;
    }

    /// Execute confirmed replace add (delete old, add new)
    pub fn confirm_replace_add(&mut self, source: String, name: String) {
        // Find and remove the existing dotfile
        if let Some(idx) = self.dotfiles.iter().position(|d| d.name == name) {
            let dotfile = &self.dotfiles[idx];
            if let Err(e) = actions::remove_dotfile(dotfile, false) {
                self.status_message = Some(format!("Error removing old dotfile: {}", e));
                self.view = View::List;
                return;
            }
        }

        // Now add the new one
        self.do_add_dotfile(&source, &name);
        self.view = View::List;
    }

    /// Helper to perform the actual add dotfile operation
    fn do_add_dotfile(&mut self, source: &str, name: &str) {
        let expanded = shellexpand::full(source)
            .map(|s| s.into_owned())
            .unwrap_or_else(|_| source.to_string());
        let path = std::path::Path::new(&expanded);

        match actions::add_dotfile(&self.manager, path, name) {
            Ok(dotfile) => {
                self.status_message = Some(format!("Added '{}'", dotfile.name));
                self.refresh_or_show_error();
            }
            Err(e) => {
                self.status_message = Some(format!("Error: {}", e));
            }
        }
    }

    /// Start edit destination flow
    pub fn start_edit_dest(&mut self) {
        if let Some(idx) = self.selected_index() {
            self.input = self.dotfiles[idx].dest_raw.clone();
            self.view = View::Input(InputMode::EditDestination(idx));
        }
    }

    /// Start search/filter
    pub fn start_search(&mut self) {
        self.input = self.filter.clone();
        self.view = View::Input(InputMode::Search);
    }

    /// Start rename flow
    pub fn start_rename(&mut self) {
        if let Some(idx) = self.selected_index() {
            self.input = self.dotfiles[idx].name.clone();
            self.view = View::Input(InputMode::Rename(idx));
        }
    }

    /// Handle input submission
    pub fn submit_input(&mut self) {
        match self.view.clone() {
            View::Input(InputMode::AddDotfileSource) => {
                let source = self.input.clone();
                self.input.clear();
                self.view = View::Input(InputMode::AddDotfileName { source });
            }
            View::Input(InputMode::AddDotfileName { source }) => {
                let name = self.input.clone();

                // Validate name before proceeding
                if let Err(e) = actions::validate_dotfile_name(&name) {
                    self.status_message = Some(format!("Invalid name: {}", e));
                    return;
                }

                self.input.clear();

                // Check if dotfile with this name already exists
                let target_dir = self.manager.dotfiles_dir.join(&name);
                if target_dir.exists() {
                    // Prompt for replacement
                    self.view = View::Confirm(ConfirmAction::ReplaceAdd { source, name });
                    return;
                }

                self.view = View::List;
                self.do_add_dotfile(&source, &name);
            }
            View::Input(InputMode::EditDestination(idx)) => {
                let new_dest = self.input.clone();
                self.input.clear();
                self.view = View::List;

                if let Some(dotfile) = self.dotfiles.get(idx) {
                    match actions::update_destination(dotfile, &new_dest) {
                        Ok(()) => {
                            self.status_message = Some("Destination updated".to_string());
                            self.refresh_or_show_error();
                        }
                        Err(e) => {
                            self.status_message = Some(format!("Error: {}", e));
                        }
                    }
                }
            }
            View::Input(InputMode::Search) => {
                self.filter = self.input.clone();
                self.input.clear();
                self.apply_filter();
                self.selected = 0;
                self.view = View::List;
            }
            View::Input(InputMode::Rename(idx)) => {
                let new_name = self.input.clone();
                self.input.clear();
                self.view = View::List;

                if let Some(dotfile) = self.dotfiles.get(idx) {
                    let old_name = dotfile.name.clone();
                    match actions::rename_dotfile(dotfile, &new_name, &self.manager.dotfiles_dir) {
                        Ok(()) => {
                            self.status_message =
                                Some(format!("Renamed '{}' to '{}'", old_name, new_name));
                            // Refresh to reload the dotfile list with new name
                            self.refresh_or_show_error();
                            // Try to select the renamed item by finding it
                            if let Some(new_idx) =
                                self.dotfiles.iter().position(|d| d.name == new_name)
                            {
                                // Find position in filtered list
                                if let Some(filtered_pos) =
                                    self.filtered_indices.iter().position(|&i| i == new_idx)
                                {
                                    self.selected = filtered_pos;
                                }
                            }
                        }
                        Err(e) => {
                            self.status_message = Some(format!("Error: {}", e));
                        }
                    }
                }
            }
            _ => {}
        }
    }

    /// Cancel current input/dialog
    pub fn cancel(&mut self) {
        self.input.clear();
        self.clear_completions();
        self.view = View::List;
    }

    /// Clear the search filter
    pub fn clear_filter(&mut self) {
        self.filter.clear();
        self.apply_filter();
        self.selected = 0;
    }

    /// Show help screen
    pub fn show_help(&mut self) {
        self.view = View::Help;
    }

    /// Handle character input
    pub fn input_char(&mut self, c: char) {
        self.input.push(c);
        // Clear completions when user types (will regenerate on next Tab)
        self.clear_completions();
    }

    /// Handle backspace in input
    pub fn input_backspace(&mut self) {
        self.input.pop();
        // Clear completions when user deletes (will regenerate on next Tab)
        self.clear_completions();
    }

    /// Start distribute (sync) mode
    pub fn start_distribute(&mut self) {
        // Show all dotfiles
        self.distribute_indices = (0..self.dotfiles.len()).collect();
        // Pre-select already linked dotfiles
        self.distribute_selected = self
            .dotfiles
            .iter()
            .enumerate()
            .filter(|(_, d)| matches!(d.link_status, LinkStatus::Linked))
            .map(|(i, _)| i)
            .collect();
        self.distribute_cursor = 0;
        self.view = View::Distribute;
    }

    /// Toggle selection of item at cursor in distribute view
    pub fn distribute_toggle(&mut self) {
        if let Some(&dotfile_idx) = self.distribute_indices.get(self.distribute_cursor) {
            if self.distribute_selected.contains(&dotfile_idx) {
                self.distribute_selected.retain(|&i| i != dotfile_idx);
            } else {
                self.distribute_selected.push(dotfile_idx);
            }
        }
    }

    /// Move cursor up in distribute view
    pub fn distribute_prev(&mut self) {
        if self.distribute_cursor > 0 {
            self.distribute_cursor -= 1;
        }
    }

    /// Move cursor down in distribute view
    pub fn distribute_next(&mut self) {
        if self.distribute_cursor < self.distribute_indices.len().saturating_sub(1) {
            self.distribute_cursor += 1;
        }
    }

    /// Execute distribute - link all selected dotfiles
    pub fn distribute_execute(&mut self) {
        // Check if there are any conflicts in the selection
        let conflict_count = self
            .distribute_selected
            .iter()
            .filter(|&&idx| {
                self.dotfiles
                    .get(idx)
                    .map(|d| matches!(d.link_status, LinkStatus::Conflict))
                    .unwrap_or(false)
            })
            .count();

        if conflict_count > 0 {
            // Prompt user to confirm overwriting conflicts
            self.view = View::Confirm(ConfirmAction::DistributeConflicts { conflict_count });
            return;
        }

        // No conflicts, proceed with linking
        self.distribute_execute_inner(false);
    }

    /// Execute distribute after conflict confirmation
    pub fn distribute_execute_with_force(&mut self, force_conflicts: bool) {
        self.distribute_execute_inner(force_conflicts);
    }

    /// Inner distribute execution - syncs link state to match selection
    fn distribute_execute_inner(&mut self, force_conflicts: bool) {
        let mut linked = 0;
        let mut unlinked = 0;
        let mut failed = 0;

        let selected = self.distribute_selected.clone();

        // Link selected items that aren't linked
        for &idx in &selected {
            if let Some(dotfile) = self.dotfiles.get(idx) {
                // Skip already linked
                if matches!(dotfile.link_status, LinkStatus::Linked) {
                    continue;
                }

                // Skip items with unknown status (can't reliably link)
                if matches!(dotfile.link_status, LinkStatus::Unknown(_)) {
                    failed += 1;
                    continue;
                }

                // Handle conflicts
                if matches!(dotfile.link_status, LinkStatus::Conflict) {
                    if force_conflicts {
                        match actions::force_link_dotfile(dotfile) {
                            Ok(LinkResult::Success) => {
                                linked += 1;
                                self.refresh_dotfile_status(idx);
                            }
                            _ => {
                                failed += 1;
                            }
                        }
                    } else {
                        failed += 1;
                    }
                    continue;
                }

                match actions::link_dotfile(dotfile) {
                    Ok(LinkResult::Success) => {
                        linked += 1;
                        self.refresh_dotfile_status(idx);
                    }
                    _ => {
                        failed += 1;
                    }
                }
            }
        }

        // Unlink items that are linked but not selected
        for idx in 0..self.dotfiles.len() {
            if selected.contains(&idx) {
                continue;
            }
            let dotfile = &self.dotfiles[idx];
            if matches!(dotfile.link_status, LinkStatus::Linked) {
                match actions::unlink_dotfile(dotfile) {
                    Ok(()) => {
                        unlinked += 1;
                        self.refresh_dotfile_status(idx);
                    }
                    Err(_) => {
                        failed += 1;
                    }
                }
            }
        }

        // Build status message
        let mut parts = Vec::new();
        if linked > 0 {
            parts.push(format!("Linked {}", linked));
        }
        if unlinked > 0 {
            parts.push(format!("Unlinked {}", unlinked));
        }
        if failed > 0 {
            parts.push(format!("{} failed", failed));
        }
        self.status_message = Some(if parts.is_empty() {
            "No changes".to_string()
        } else {
            parts.join(", ")
        });

        self.distribute_selected.clear();
        self.distribute_indices.clear();
        self.view = View::List;
    }

    /// Cancel distribute mode
    pub fn distribute_cancel(&mut self) {
        self.distribute_selected.clear();
        self.distribute_indices.clear();
        self.view = View::List;
    }

    /// Select all in distribute view
    pub fn distribute_select_all(&mut self) {
        self.distribute_selected = self.distribute_indices.clone();
    }

    /// Deselect all in distribute view
    pub fn distribute_select_none(&mut self) {
        self.distribute_selected.clear();
    }

    /// Trigger path completion (Tab key pressed)
    pub fn trigger_completion(&mut self) {
        // Only for AddDotfileSource and EditDestination modes
        let should_complete = matches!(
            self.view,
            View::Input(InputMode::AddDotfileSource) | View::Input(InputMode::EditDestination(_))
        );

        if !should_complete {
            return;
        }

        // If input changed since last completion, regenerate candidates
        if self.input != self.completion_base {
            self.generate_completions();
            self.completion_base = self.input.clone();
            self.completion_index = if self.completion_candidates.is_empty() {
                None
            } else {
                Some(0)
            };
        } else if !self.completion_candidates.is_empty() {
            // Input hasn't changed - cycle to next candidate
            self.cycle_completion();
        } else {
            // No candidates available
            return;
        }

        // Apply the selected completion
        self.apply_completion();
    }

    /// Generate path completion candidates based on current input
    fn generate_completions(&mut self) {
        use std::path::Path;

        let input = &self.input;

        // Handle empty input - complete from current directory
        if input.is_empty() {
            self.complete_directory(Path::new("."), "");
            return;
        }

        // Expand ~ and environment variables
        let expanded = shellexpand::full(input)
            .map(|s| s.into_owned())
            .unwrap_or_else(|_| input.clone());

        let path = Path::new(&expanded);

        // Determine directory to search and partial filename
        if expanded.ends_with('/') || expanded.ends_with(std::path::MAIN_SEPARATOR) {
            // User typed trailing slash - complete in that directory
            self.complete_directory(path, "");
        } else {
            // Split into directory and partial filename
            let parent = path.parent().unwrap_or(Path::new("."));
            let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            self.complete_directory(parent, filename);
        }
    }

    /// Helper to complete within a specific directory
    fn complete_directory(&mut self, dir: &std::path::Path, partial: &str) {
        use std::fs;
        use std::path::PathBuf;

        self.completion_candidates.clear();

        // Try to read directory
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };

        // Collect matching entries
        let mut candidates: Vec<PathBuf> = entries
            .filter_map(|e| e.ok())
            .filter_map(|entry| {
                let name = entry.file_name().to_string_lossy().to_string();
                // Filter by partial match (case-sensitive prefix)
                if name.starts_with(partial) {
                    Some(entry.path())
                } else {
                    None
                }
            })
            .collect();

        // Sort alphabetically (directories first, then files)
        candidates.sort_by(|a, b| {
            let a_is_dir = a.is_dir();
            let b_is_dir = b.is_dir();
            match (a_is_dir, b_is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.cmp(b),
            }
        });

        // Limit to 50 candidates to prevent performance issues
        candidates.truncate(50);

        self.completion_candidates = candidates;
    }

    /// Cycle to next completion candidate
    fn cycle_completion(&mut self) {
        if let Some(idx) = self.completion_index
            && !self.completion_candidates.is_empty()
        {
            self.completion_index = Some((idx + 1) % self.completion_candidates.len());
        }
    }

    /// Apply selected completion to input buffer
    fn apply_completion(&mut self) {
        if let Some(idx) = self.completion_index
            && let Some(path) = self.completion_candidates.get(idx)
        {
            // Convert back to use $HOME if applicable
            let home = std::env::var("HOME").unwrap_or_default();
            let path_str = path.to_string_lossy();
            let display_path = if !home.is_empty() && path_str.starts_with(&home) {
                path_str.replacen(&home, "$HOME", 1)
            } else {
                path_str.to_string()
            };

            // Add trailing slash for directories
            if path.is_dir() {
                self.input = format!("{}/", display_path);
            } else {
                self.input = display_path;
            }

            // Update completion base to new input
            self.completion_base = self.input.clone();
        }
    }

    /// Clear completion state
    fn clear_completions(&mut self) {
        self.completion_candidates.clear();
        self.completion_index = None;
        self.completion_base.clear();
    }
}
