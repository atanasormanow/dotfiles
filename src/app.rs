use crate::actions::{self, LinkResult};
use crate::dotfile::{Dotfile, DotfileManager, LinkStatus};
use anyhow::Result;

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
}

/// Actions that require confirmation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfirmAction {
    Unlink(usize),
    Remove(usize),
    ForceLink(usize),
}

/// Input modes for the input dialog
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputMode {
    AddDotfileSource,
    AddDotfileName { source: String },
    EditDestination(usize),
    Search,
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
                    self.dotfiles[idx].refresh_status();
                    let repo_root = self.manager.repo_root().to_path_buf();
                    self.dotfiles[idx].refresh_git_status(&repo_root);
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
                    self.dotfiles[idx].refresh_status();
                    let repo_root = self.manager.repo_root().to_path_buf();
                    self.dotfiles[idx].refresh_git_status(&repo_root);
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
            match actions::force_link_dotfile(dotfile, true) {
                Ok(LinkResult::Success) => {
                    self.status_message = Some(format!("Force linked '{}' (backup created)", name));
                    self.dotfiles[idx].refresh_status();
                    let repo_root = self.manager.repo_root().to_path_buf();
                    self.dotfiles[idx].refresh_git_status(&repo_root);
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
                    let _ = self.refresh();
                }
                Err(e) => {
                    self.status_message = Some(format!("Error: {}", e));
                }
            }
        }
        self.view = View::List;
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
                self.input.clear();
                self.view = View::List;

                // Expand the source path
                let expanded = shellexpand::full(&source)
                    .map(|s| s.into_owned())
                    .unwrap_or(source.clone());
                let path = std::path::Path::new(&expanded);

                match actions::add_dotfile(&self.manager, path, &name) {
                    Ok(dotfile) => {
                        self.status_message = Some(format!("Added '{}'", dotfile.name));
                        let _ = self.refresh();
                    }
                    Err(e) => {
                        self.status_message = Some(format!("Error: {}", e));
                    }
                }
            }
            View::Input(InputMode::EditDestination(idx)) => {
                let new_dest = self.input.clone();
                self.input.clear();
                self.view = View::List;

                if let Some(dotfile) = self.dotfiles.get(idx) {
                    match actions::update_destination(dotfile, &new_dest) {
                        Ok(()) => {
                            self.status_message = Some("Destination updated".to_string());
                            let _ = self.refresh();
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
            _ => {}
        }
    }

    /// Cancel current input/dialog
    pub fn cancel(&mut self) {
        self.input.clear();
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
    }

    /// Handle backspace in input
    pub fn input_backspace(&mut self) {
        self.input.pop();
    }
}
