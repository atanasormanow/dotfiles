use anyhow::{Context, Result};
use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;

use crate::dotfile::{Dotfile, DotfileManager, LinkStatus};

/// Convert a path to a string, replacing $HOME prefix if applicable
fn path_with_home_var(path: &Path) -> String {
    let home = std::env::var("HOME").unwrap_or_default();
    let path_str = path.to_string_lossy();
    if !home.is_empty() && path_str.starts_with(&home) {
        path_str.replacen(&home, "$HOME", 1)
    } else {
        path_str.to_string()
    }
}

/// Ensure parent directory exists, creating it if needed
fn ensure_parent_dir_exists(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent()
        && !parent.exists()
    {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {:?}", parent))?;
    }
    Ok(())
}

/// Result of a link operation
#[derive(Debug)]
pub enum LinkResult {
    Success,
    AlreadyLinked,
    Conflict,
    Error(String),
}

/// Link a dotfile (create symlink at destination)
pub fn link_dotfile(dotfile: &Dotfile) -> Result<LinkResult> {
    // Check current status
    match &dotfile.link_status {
        LinkStatus::Linked => return Ok(LinkResult::AlreadyLinked),
        LinkStatus::Conflict => return Ok(LinkResult::Conflict),
        LinkStatus::Broken => {
            // Remove broken symlink first
            fs::remove_file(&dotfile.dest_expanded).context("Failed to remove broken symlink")?;
        }
        LinkStatus::Unlinked => {}
        LinkStatus::Unknown(e) => {
            return Ok(LinkResult::Error(format!("Unknown status: {}", e)));
        }
    }

    // Create parent directories if needed
    ensure_parent_dir_exists(&dotfile.dest_expanded)?;

    // Create the symlink
    symlink(&dotfile.source_file, &dotfile.dest_expanded).with_context(|| {
        format!(
            "Failed to create symlink: {:?} -> {:?}",
            dotfile.dest_expanded, dotfile.source_file
        )
    })?;

    Ok(LinkResult::Success)
}

/// Unlink a dotfile (remove symlink at destination)
pub fn unlink_dotfile(dotfile: &Dotfile) -> Result<()> {
    match &dotfile.link_status {
        LinkStatus::Linked | LinkStatus::Broken => {
            fs::remove_file(&dotfile.dest_expanded).with_context(|| {
                format!("Failed to remove symlink: {:?}", dotfile.dest_expanded)
            })?;
            Ok(())
        }
        LinkStatus::Unlinked => {
            anyhow::bail!("Dotfile is not linked");
        }
        LinkStatus::Conflict => {
            anyhow::bail!("Cannot unlink: destination is a regular file, not a symlink");
        }
        LinkStatus::Unknown(e) => {
            anyhow::bail!("Cannot unlink: {}", e);
        }
    }
}

/// Force link a dotfile, replacing any existing file
pub fn force_link_dotfile(dotfile: &Dotfile) -> Result<LinkResult> {
    // Remove existing file at destination
    if dotfile.dest_expanded.exists() || dotfile.dest_expanded.is_symlink() {
        if dotfile.dest_expanded.is_dir() && !dotfile.dest_expanded.is_symlink() {
            fs::remove_dir_all(&dotfile.dest_expanded).with_context(|| {
                format!("Failed to remove directory: {:?}", dotfile.dest_expanded)
            })?;
        } else {
            fs::remove_file(&dotfile.dest_expanded)
                .with_context(|| format!("Failed to remove file: {:?}", dotfile.dest_expanded))?;
        }
    }

    // Create parent directories if needed
    ensure_parent_dir_exists(&dotfile.dest_expanded)?;

    // Create the symlink
    symlink(&dotfile.source_file, &dotfile.dest_expanded).with_context(|| {
        format!(
            "Failed to create symlink: {:?} -> {:?}",
            dotfile.dest_expanded, dotfile.source_file
        )
    })?;

    Ok(LinkResult::Success)
}

/// Add a new dotfile to the repository
pub fn add_dotfile(
    manager: &DotfileManager,
    source_path: &Path,
    dotfile_name: &str,
) -> Result<Dotfile> {
    // Validate source exists
    if !source_path.exists() {
        anyhow::bail!("Source file does not exist: {:?}", source_path);
    }

    // Validate dotfile name doesn't exist
    let target_dir = manager.dotfiles_dir.join(dotfile_name);
    if target_dir.exists() {
        anyhow::bail!("Dotfile '{}' already exists", dotfile_name);
    }

    // Get the filename
    let file_name = source_path
        .file_name()
        .and_then(|n| n.to_str())
        .context("Invalid source filename")?;

    // Create the dotfile directory
    fs::create_dir_all(&target_dir)
        .with_context(|| format!("Failed to create directory: {:?}", target_dir))?;

    // Canonicalize BEFORE moving the file (it won't exist at original path after move)
    let source_abs = source_path
        .canonicalize()
        .unwrap_or_else(|_| source_path.to_path_buf());

    // Move the source file to the repo
    let dest_in_repo = target_dir.join(file_name);
    fs::rename(source_path, &dest_in_repo)
        .with_context(|| format!("Failed to move file to repo: {:?}", source_path))?;

    // Create the dest file with the original path (using $HOME if applicable)
    let dest_content = path_with_home_var(&source_abs);

    let dest_file = target_dir.join("dest");
    fs::write(&dest_file, format!("{}\n", dest_content))
        .with_context(|| format!("Failed to write dest file: {:?}", dest_file))?;

    // Create symlink at original location
    symlink(&dest_in_repo, source_path).with_context(|| {
        format!(
            "Failed to create symlink at original location: {:?}",
            source_path
        )
    })?;

    // Check if source is a directory
    let is_directory = dest_in_repo.is_dir();

    // Return the new dotfile
    Ok(Dotfile {
        name: dotfile_name.to_string(),
        repo_path: target_dir,
        source_file: dest_in_repo,
        dest_raw: dest_content,
        dest_expanded: source_abs,
        link_status: LinkStatus::Linked,
        git_status: crate::dotfile::GitStatus::Modified,
        is_directory,
    })
}

/// Update the destination of a dotfile
pub fn update_destination(dotfile: &Dotfile, new_dest: &str) -> Result<()> {
    let dest_file = dotfile.repo_path.join("dest");
    fs::write(&dest_file, format!("{}\n", new_dest))
        .with_context(|| format!("Failed to write dest file: {:?}", dest_file))?;
    Ok(())
}

/// Remove a dotfile from the repository
pub fn remove_dotfile(dotfile: &Dotfile, restore: bool) -> Result<()> {
    // Optionally restore the file to its original location
    if restore {
        if dotfile.dest_expanded.exists() || dotfile.dest_expanded.is_symlink() {
            // Remove existing symlink/file first
            if dotfile.dest_expanded.is_symlink() {
                fs::remove_file(&dotfile.dest_expanded)?;
            } else {
                anyhow::bail!(
                    "Cannot restore: a file already exists at {:?}",
                    dotfile.dest_expanded
                );
            }
        }

        // Create parent directories
        ensure_parent_dir_exists(&dotfile.dest_expanded)?;

        // Copy (not move, in case removal fails)
        if dotfile.source_file.is_dir() {
            copy_dir_recursive(&dotfile.source_file, &dotfile.dest_expanded)?;
        } else {
            fs::copy(&dotfile.source_file, &dotfile.dest_expanded)?;
        }
    } else {
        // Just unlink if currently linked
        if matches!(dotfile.link_status, LinkStatus::Linked | LinkStatus::Broken) {
            let _ = fs::remove_file(&dotfile.dest_expanded);
        }
    }

    // Remove the dotfile directory from repo
    fs::remove_dir_all(&dotfile.repo_path).with_context(|| {
        format!(
            "Failed to remove dotfile directory: {:?}",
            dotfile.repo_path
        )
    })?;

    Ok(())
}

/// Unmanage a dotfile - move it from the repo to its destination
pub fn unmanage_dotfile(dotfile: &Dotfile) -> Result<()> {
    // Check if a regular file already exists at destination (conflict)
    if dotfile.dest_expanded.exists() && !dotfile.dest_expanded.is_symlink() {
        anyhow::bail!(
            "Cannot unmanage: a file already exists at {:?}",
            dotfile.dest_expanded
        );
    }

    // Remove symlink if currently linked
    if dotfile.dest_expanded.is_symlink() {
        fs::remove_file(&dotfile.dest_expanded)
            .with_context(|| format!("Failed to remove symlink: {:?}", dotfile.dest_expanded))?;
    }

    // Create parent directories if needed
    ensure_parent_dir_exists(&dotfile.dest_expanded)?;

    // Move the file/directory from repo to destination
    fs::rename(&dotfile.source_file, &dotfile.dest_expanded).with_context(|| {
        format!(
            "Failed to move {:?} to {:?}",
            dotfile.source_file, dotfile.dest_expanded
        )
    })?;

    // Remove the dotfile directory from repo (now only contains 'dest' file)
    fs::remove_dir_all(&dotfile.repo_path).with_context(|| {
        format!(
            "Failed to remove dotfile directory: {:?}",
            dotfile.repo_path
        )
    })?;

    Ok(())
}

/// Validate a dotfile name (alphanumeric, dash, underscore, dot only)
pub fn validate_dotfile_name(name: &str) -> Result<()> {
    if name.is_empty() {
        anyhow::bail!("Name cannot be empty");
    }

    let is_valid = name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.');

    if !is_valid {
        anyhow::bail!("Name can only contain alphanumeric characters, -, _, and .");
    }

    Ok(())
}

/// Rename a dotfile (rename directory and update symlink if needed)
pub fn rename_dotfile(dotfile: &Dotfile, new_name: &str, dotfiles_dir: &Path) -> Result<()> {
    // Validate the new name
    validate_dotfile_name(new_name)?;

    let old_path = &dotfile.repo_path;
    let new_path = dotfiles_dir.join(new_name);

    // Check if new name already exists
    if new_path.exists() {
        anyhow::bail!("A dotfile named '{}' already exists", new_name);
    }

    // Store link status and destination before rename
    let was_linked = matches!(dotfile.link_status, LinkStatus::Linked);

    // Unlink if currently linked (we'll re-link after rename)
    if was_linked {
        unlink_dotfile(dotfile)?;
    }

    // Rename the directory
    fs::rename(old_path, &new_path).with_context(|| {
        format!(
            "Failed to rename directory: {:?} -> {:?}",
            old_path, new_path
        )
    })?;

    // Re-link if it was linked before
    if was_linked {
        // Get the new source file path (inside renamed directory)
        let file_name = dotfile
            .source_file
            .file_name()
            .context("Invalid source file name")?;
        let new_source_file = new_path.join(file_name);

        // Create symlink pointing to new location
        symlink(&new_source_file, &dotfile.dest_expanded).with_context(|| {
            format!(
                "Failed to create symlink after rename: {:?} -> {:?}",
                dotfile.dest_expanded, new_source_file
            )
        })?;
    }

    Ok(())
}

/// Recursively copy a directory
fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
