// This file is part of tbdflow, a CLI tool for Trunk-Based Development workflows.

use std::process::{Command, Stdio};
use thiserror::Error;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use colored::Colorize;
use crate::git;

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

// Helper function to perform and display the stale branch check
pub fn check_and_warn_for_stale_branches() -> Result<(), anyhow::Error> {
    let stale_branches = git::get_stale_branches()?;
    if !stale_branches.is_empty() {
        println!("\n{}", "Warning: The following branches may be stale:".bold().yellow());
        for (branch, days) in stale_branches {
            println!("{}", format!("  - {} (last commit {} days ago)", branch, days).yellow());
        }
    }
    Ok(())
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

pub fn push_tags() -> Result<String> {
    run_git_command("push", &["--tags"])
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

pub fn get_head_commit_hash() -> Result<String> {
    run_git_command("rev-parse", &["HEAD"])
}

pub fn create_tag(tag_name: &str, message: &str, commit_hash: &str) -> Result<String> {
    run_git_command("tag", &["-a", tag_name, "-m", message, commit_hash])
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

/// Check for stale branches in the repository.
pub fn get_stale_branches() -> Result<Vec<(String, i64)>> {
    let now = Utc::now();
    let day_in_seconds = 24 * 60 * 60;

    let output = run_git_command("for-each-ref", &["--format", "%(refname:short)|%(committerdate:iso8601-strict)", "refs/heads/"])?;
    let stale_branches = output
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 2 {
                let branch_name = parts[0].to_string();
                if branch_name == "main" {
                    return None; // Skip the main branch
                }
                if let Ok(date) = DateTime::parse_from_rfc3339(parts[1]) {
                    let duration = now.signed_duration_since(date);
                    if duration.num_seconds() > day_in_seconds {
                        return Some(Ok((branch_name, duration.num_days())));
                    }
                }
            }
            None
        })
        .collect::<Result<Vec<_>, anyhow::Error>>()?;

    Ok(stale_branches)
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