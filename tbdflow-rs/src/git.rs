// This file is part of tbdflow, a CLI tool for Trunk-Based Development workflows.

use std::process::{Command, Stdio};
use thiserror::Error;
use anyhow::{Context, Result};

// --- Custom Error Type ---
// Using `thiserror` to create a structured error type.
#[derive(Error, Debug)]
pub enum GitError {
    #[error("Git command failed: {0}")]
    Git(String),
    #[error("Working directory is not clean: {0}")]
    DirectoryNotClean(String),
    #[error("Invalid branch type: {0}. Use 'feature', 'release', or 'hotfix'.")]
    InvalidBranchType(String),
}

/// Runs a Git command with the specified subcommand and arguments.
///
/// # Arguments
///
/// * `command` - The Git subcommand to execute (e.g. `"checkout"`, `"status"`).
/// * `args` - A slice of argument strings to pass to the Git command.
///
/// # Returns
///
/// A `Result<String>` containing the command's output if successful, or an error if the command fails.
///
/// # Errors
///
/// If the command fails, it returns a `GitError` with the error message from Git.
///
fn run_git_command(command: &str, args: &[&str]) -> Result<String> {
    println!("[RUNNING] git {} {}", command, args.join(" "));
    let output = Command::new("git")
        .arg(command)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .with_context(|| format!("Failed to execute 'git {}'", command))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(GitError::Git(String::from_utf8_lossy(&output.stderr).trim().to_string()).into())
    }
}

/// Checks if the git working directory is clean.
pub fn is_working_directory_clean() -> Result<()> {
    let output = run_git_command("status", &["--porcelain"])?;
    if output.is_empty() {
        Ok(())
    } else {
        Err(GitError::DirectoryNotClean(
            "You have unstaged changes. Please commit or stash them first.".to_string()
        ).into())
        //Err("You have unstaged changes. Please commit or stash them first.".to_string())
    }
}

// -- Public Git workflow functions --
// These functions provide a high-level interface to common Git operations.

/// Check out the main branch.
pub fn checkout_main() -> Result<String> {
    run_git_command("checkout", &["main"])
}

/// Pull the latest changes with rebase.
pub fn pull_latest_with_rebase() -> Result<String> {
    // Using --autostash to safely handle local changes before pulling.
    run_git_command("pull", &["--rebase", "--autostash"])
}

/// Add all changes to the staging area.
pub fn add_all() -> Result<String> {
    run_git_command("add", &["."])
}

/// Commit changes with a message.
pub fn commit(message: &str) -> Result<String> {
    run_git_command("commit", &["-m", message])
}

/// Push changes to the remote repository.
pub fn push() -> Result<String> {
    run_git_command("push", &[])
}

/// Merge the current branch with another branch.
pub fn merge_branch(branch_name: &str) -> Result<String> {
    run_git_command("merge", &["--no-ff", branch_name])
}

/// Delete a local short-lived branch.
pub fn delete_local_branch(branch_name: &str) -> Result<String> {
    run_git_command("branch", &["-d", branch_name])
}

/// Delete a remote branch.
pub fn delete_remote_branch(branch_name: &str) -> Result<String> {
    run_git_command("push", &["origin", "--delete", branch_name])
}

/// Get the current branch name.
pub fn get_current_branch() -> Result<String> {
    run_git_command("rev-parse", &["--abbrev-ref", "HEAD"])
}

/// Create a new branch from the current HEAD or a specified point.
pub fn create_branch(branch_name: &str, from_point: Option<&str>) -> Result<String> {
    let mut args = vec!["-b", branch_name];
    if let Some(point) = from_point {
        args.push(point);
    }
    run_git_command("checkout", &args)
}

pub fn push_set_upstream(branch_name: &str) -> Result<String> {
    run_git_command("push", &["--set-upstream", "origin", branch_name])
}

/// Show the current status of the repository.
pub fn status() -> Result<String> {
    run_git_command("status", &["--short"])
}

/// Show recent commits in the repository, 15 by default.
pub fn log_graph() -> Result<String> {
    run_git_command("log", &["--graph", "--oneline", "-n", "15"])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_is_installed() {
        let result = std::process::Command::new("git")
            .arg("--version")
            .output();
        assert!(result.is_ok(), "Git is not installed or not in PATH");
        let output = result.unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("git version"), "Unexpected output: {}", stdout);
    }

    /// Test the run_git_command function with a simple command
    #[test]
    fn test_run_git_command_version() {
        let result = run_git_command("--version", &[]);
        assert!(result.is_ok(), "Expected Ok, got {:?}", result);
        let output = result.unwrap();
        assert!(output.contains("git version"), "Output was: {}", output);
    }

    /// Test the status function
    #[test]
    fn test_status() {
        let result = status();
        assert!(result.is_ok(), "Expected Ok, got {:?}", result);
        let output = result.unwrap();
        // Accept any output (including empty if clean)
        assert!(
            output.is_empty() || output.contains("M ") || output.contains("A ") || output.contains("D "),
            "Unexpected status output: {}",
            output
        );
    }
}