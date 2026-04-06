use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use crate::git;

/// Status of the symlink at the destination
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkStatus {
    /// Symlink exists and points to the repo file
    Linked,
    /// Symlink exists but target is missing/wrong
    Broken,
    /// A regular file exists at destination (not a symlink)
    Conflict,
    /// No file or symlink at destination
    Unlinked,
    /// Cannot determine status (missing dest file, etc.)
    Unknown(String),
}

impl LinkStatus {
    pub fn symbol(&self) -> &'static str {
        match self {
            LinkStatus::Linked => "[L]",
            LinkStatus::Broken => "[X]",
            LinkStatus::Conflict => "[C]",
            LinkStatus::Unlinked => "[-]",
            LinkStatus::Unknown(_) => "[?]",
        }
    }
}

/// Git status of the dotfile
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum GitStatus {
    #[default]
    Clean,
    Modified,
    Staged,
}

impl GitStatus {
    pub fn symbol(&self) -> &'static str {
        match self {
            GitStatus::Clean => "[ ]",
            GitStatus::Modified => "[M]",
            GitStatus::Staged => "[+]",
        }
    }
}

/// Represents a managed dotfile
#[derive(Debug, Clone)]
pub struct Dotfile {
    /// Name of the dotfile (directory name in dotfiles/)
    pub name: String,
    /// Path to the dotfile directory in the repo
    pub repo_path: PathBuf,
    /// Path to the actual config file in the repo
    pub source_file: PathBuf,
    /// Raw destination from dest file (with $HOME etc.)
    pub dest_raw: String,
    /// Expanded destination path
    pub dest_expanded: PathBuf,
    /// Current symlink status
    pub link_status: LinkStatus,
    /// Current git status
    pub git_status: GitStatus,
    /// Whether destination requires elevated permissions
    pub needs_sudo: bool,
    /// Whether the source is a directory (not a single file)
    pub is_directory: bool,
}

impl Dotfile {
    /// Check the current link status by examining the destination
    pub fn check_link_status(&self) -> LinkStatus {
        let dest = &self.dest_expanded;

        if !dest.exists() && !dest.is_symlink() {
            return LinkStatus::Unlinked;
        }

        if dest.is_symlink() {
            match fs::read_link(dest) {
                Ok(target) => {
                    // Canonicalize both paths for comparison
                    let target_canonical = if target.is_absolute() {
                        target.canonicalize().ok()
                    } else {
                        dest.parent()
                            .map(|p| p.join(&target))
                            .and_then(|p| p.canonicalize().ok())
                    };

                    let source_canonical = self.source_file.canonicalize().ok();

                    match (target_canonical, source_canonical) {
                        (Some(t), Some(s)) if t == s => LinkStatus::Linked,
                        _ => LinkStatus::Broken,
                    }
                }
                Err(_) => LinkStatus::Broken,
            }
        } else {
            // Regular file exists at destination
            LinkStatus::Conflict
        }
    }

    /// Refresh the link status
    pub fn refresh_status(&mut self) {
        self.link_status = self.check_link_status();
    }

    /// Refresh the git status
    pub fn refresh_git_status(&mut self, repo_root: &Path) {
        // Get the relative path from repo root to the dotfile directory
        if let Ok(relative) = self.repo_path.strip_prefix(repo_root) {
            match git::get_status_for_path(repo_root, relative) {
                Ok(status) => {
                    self.git_status = match status {
                        git::FileGitStatus::Clean => GitStatus::Clean,
                        git::FileGitStatus::Modified => GitStatus::Modified,
                        git::FileGitStatus::Staged => GitStatus::Staged,
                        git::FileGitStatus::Untracked => GitStatus::Modified,
                    };
                }
                Err(_) => {
                    self.git_status = GitStatus::Clean;
                }
            }
        }
    }
}

/// Discovers and loads all dotfiles from the repository
pub struct DotfileManager {
    /// Root of the dotfiles repository
    pub repo_root: PathBuf,
    /// Path to the dotfiles storage directory
    pub dotfiles_dir: PathBuf,
}

impl DotfileManager {
    /// Create a new manager, auto-detecting the repo root
    pub fn new() -> Result<Self> {
        let repo_root = Self::find_repo_root()?;
        let dotfiles_dir = repo_root.join("dotfiles");

        Ok(Self {
            repo_root,
            dotfiles_dir,
        })
    }

    /// Find the repository root by looking for the dotfiles directory
    fn find_repo_root() -> Result<PathBuf> {
        // First, try the executable's directory
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(parent) = exe_path.parent() {
                // Check if we're in target/debug or target/release
                let potential_root = if parent.ends_with("debug") || parent.ends_with("release") {
                    parent.parent().and_then(|p| p.parent())
                } else {
                    Some(parent)
                };

                if let Some(root) = potential_root {
                    if root.join("dotfiles").is_dir() {
                        return Ok(root.to_path_buf());
                    }
                }
            }
        }

        // Fall back to current directory
        let cwd = std::env::current_dir().context("Failed to get current directory")?;
        if cwd.join("dotfiles").is_dir() {
            return Ok(cwd);
        }

        // Try parent directories
        let mut current = cwd.as_path();
        while let Some(parent) = current.parent() {
            if parent.join("dotfiles").is_dir() {
                return Ok(parent.to_path_buf());
            }
            current = parent;
        }

        anyhow::bail!("Could not find dotfiles repository root")
    }

    /// Discover all dotfiles in the repository
    pub fn discover(&self) -> Result<Vec<Dotfile>> {
        let mut dotfiles = Vec::new();

        if !self.dotfiles_dir.is_dir() {
            return Ok(dotfiles);
        }

        let entries =
            fs::read_dir(&self.dotfiles_dir).context("Failed to read dotfiles directory")?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if !path.is_dir() {
                continue;
            }

            match self.load_dotfile(&path) {
                Ok(dotfile) => dotfiles.push(dotfile),
                Err(e) => {
                    // Create a placeholder for invalid dotfiles
                    let name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                        .to_string();

                    dotfiles.push(Dotfile {
                        name,
                        repo_path: path.clone(),
                        source_file: path.clone(),
                        dest_raw: String::new(),
                        dest_expanded: PathBuf::new(),
                        link_status: LinkStatus::Unknown(e.to_string()),
                        git_status: GitStatus::default(),
                        needs_sudo: false,
                        is_directory: false,
                    });
                }
            }
        }

        // Sort: directories first, then by name
        dotfiles.sort_by(|a, b| match (a.is_directory, b.is_directory) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        });

        Ok(dotfiles)
    }

    /// Load a single dotfile from its directory
    fn load_dotfile(&self, dir: &Path) -> Result<Dotfile> {
        let name = dir
            .file_name()
            .and_then(|n| n.to_str())
            .context("Invalid directory name")?
            .to_string();

        // Read the dest file
        let dest_file = dir.join("dest");
        let dest_raw = fs::read_to_string(&dest_file)
            .with_context(|| format!("Failed to read dest file for '{}'", name))?
            .trim()
            .to_string();

        // Expand environment variables
        let dest_expanded = PathBuf::from(
            shellexpand::full(&dest_raw)
                .with_context(|| format!("Failed to expand path for '{}'", name))?
                .into_owned(),
        );

        // Find the source file (the file that should be symlinked)
        let source_file_name = dest_expanded
            .file_name()
            .and_then(|n| n.to_str())
            .context("Destination has no filename")?;

        let source_file = dir.join(source_file_name);

        // Check if source exists (could be a file or directory)
        if !source_file.exists() {
            anyhow::bail!("Source file '{}' not found in '{}'", source_file_name, name);
        }

        // Check if destination requires sudo (not in $HOME)
        let needs_sudo = !dest_raw.contains("$HOME") && !dest_raw.contains("~");

        // Check if source is a directory
        let is_directory = source_file.is_dir();

        let mut dotfile = Dotfile {
            name,
            repo_path: dir.to_path_buf(),
            source_file,
            dest_raw,
            dest_expanded,
            link_status: LinkStatus::Unknown("Not checked".into()),
            git_status: GitStatus::default(),
            needs_sudo,
            is_directory,
        };

        dotfile.refresh_status();
        dotfile.refresh_git_status(&self.repo_root);

        Ok(dotfile)
    }

    /// Get the repo root
    pub fn repo_root(&self) -> &Path {
        &self.repo_root
    }
}
