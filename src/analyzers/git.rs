use std::process::Command;

use anyhow::Result;

use crate::inventory::{GitInfo, Inventory};

pub fn analyze(inventory: &mut Inventory) -> Result<()> {
    // Skip if this is not a Git repository.
    if !inventory.root.join(".git").exists() {
        return Ok(());
    }

    // Current branch
    let branch = Command::new("git")
        .arg("-C")
        .arg(&inventory.root)
        .args(["branch", "--show-current"])
        .output()?;

    let branch = String::from_utf8_lossy(&branch.stdout).trim().to_string();

    // Repository status
    let status = Command::new("git")
        .arg("-C")
        .arg(&inventory.root)
        .args(["status", "--porcelain"])
        .output()?;

    let clean = status.stdout.is_empty();

    // Commit count
    let commits = Command::new("git")
        .arg("-C")
        .arg(&inventory.root)
        .args(["rev-list", "--count", "HEAD"])
        .output()?;

    let commits = String::from_utf8_lossy(&commits.stdout)
        .trim()
        .parse::<usize>()
        .unwrap_or(0);

    // Remote URL
    let remote = Command::new("git")
        .arg("-C")
        .arg(&inventory.root)
        .args(["remote", "get-url", "origin"])
        .output()?;

    let remote = if remote.status.success() {
        Some(String::from_utf8_lossy(&remote.stdout).trim().to_string())
    } else {
        None
    };

    inventory.git = Some(GitInfo {
        branch,
        clean,
        commits,
        contributors: 0,
        remote,
    });

    Ok(())
}
