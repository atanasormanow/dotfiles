use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

/// Git status for a file
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileGitStatus {
    Clean,
    Modified,
    Staged,
    Untracked,
}

/// Run a git command and return stdout
fn git_command(repo_path: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(["-C", repo_path.to_str().unwrap_or(".")])
        .args(args)
        .output()
        .context("Failed to run git command")?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Git command failed: {}", stderr)
    }
}

/// Get the git status of files in a directory
pub fn get_status_for_path(repo_root: &Path, relative_path: &Path) -> Result<FileGitStatus> {
    let path_str = relative_path.to_str().unwrap_or("");

    // Get porcelain status
    let output = git_command(repo_root, &["status", "--porcelain", path_str])?;

    if output.trim().is_empty() {
        return Ok(FileGitStatus::Clean);
    }

    // Parse the status output
    for line in output.lines() {
        if line.len() < 3 {
            continue;
        }

        let index_status = line.chars().next().unwrap_or(' ');
        let worktree_status = line.chars().nth(1).unwrap_or(' ');

        // Check staged changes (index)
        if index_status != ' ' && index_status != '?' {
            return Ok(FileGitStatus::Staged);
        }

        // Check worktree changes
        if worktree_status == 'M' || worktree_status == 'D' {
            return Ok(FileGitStatus::Modified);
        }

        // Untracked files
        if index_status == '?' {
            return Ok(FileGitStatus::Untracked);
        }
    }

    Ok(FileGitStatus::Clean)
}
