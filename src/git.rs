// This file is part of tbdflow, a CLI tool for Trunk-Based Development workflows.

use crate::config::Config;
use crate::misc;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use colored::Colorize;
use std::process::{Command, Stdio};
use thiserror::Error;

// --- Custom Error Type ---
// Using `thiserror` to create a structured error type.
#[derive(Error, Debug)]
pub enum GitError {
    #[error("Git command failed: {0}")]
    Git(String),
    #[error("Working directory is not clean: {0}")]
    DirectoryNotClean(String),
    #[error("Invalid branch type: {0}.")]
    InvalidBranchType(String),
    #[error("Branch '{0}' does not exist locally.")]
    BranchNotFound(String),
    #[error("Tag '{0}' already exists.")]
    TagAlreadyExists(String),
    #[error("Cannot complete the main branch. This is a protected branch.")]
    CannotCompleteMainBranch,
    #[error("Not on main branch: {0}")]
    NotOnMainBranch(String),
    #[error("Not a Git repository: {0}")]
    NotAGitRepository(String),
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
fn run_git_command(command: &str, args: &[&str], verbose: bool) -> Result<String> {
    if verbose {
        println!("{} git {} {}", "[RUNNING] ".cyan(), command, args.join(" "));
    }
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
pub fn is_working_directory_clean(verbose: bool) -> Result<()> {
    let output = run_git_command("status", &["--porcelain"], verbose)?;
    if output.is_empty() {
        Ok(())
    } else {
        Err(GitError::DirectoryNotClean(
            "You have unstaged changes. Please commit or stash them first.".to_string(),
        )
        .into())
    }
}

/// Runs a Git command that checks the status of the repository without producing output.
fn run_git_status_check(
    command: &str,
    args: &[&str],
    verbose: bool,
) -> Result<std::process::ExitStatus> {
    if verbose {
        println!(
            "{} git {} {}",
            "[CHECKING] ".dimmed(),
            command,
            args.join(" ")
        );
    }
    Command::new("git")
        .arg(command)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .with_context(|| format!("Failed to execute 'git {}'", command))
}

/// Checks if there are any changes in the staging area.
pub fn has_staged_changes(verbose: bool) -> Result<bool> {
    let status = run_git_status_check("diff", &["--staged", "--quiet"], verbose)?;
    // `git diff --quiet` exits with 1 if there are changes, 0 if not.
    Ok(status.code() == Some(1))
}

/// Add a new remote repository to the current Git repository.
/// git remote add origin <your-repository-url>
pub fn add_remote(remote_name: &str, remote_url: &str, verbose: bool) -> Result<String> {
    run_git_command("remote", &["add", remote_name, remote_url], verbose)
}

/// Check out the main branch.
pub fn checkout_main(verbose: bool, main_branch: &str) -> Result<String> {
    run_git_command("checkout", &[main_branch], verbose)
}

/// Pull the latest changes with rebase.
pub fn pull_latest_with_rebase(verbose: bool) -> Result<String> {
    // Using --autostash to safely handle local changes before pulling.
    run_git_command("pull", &["--rebase", "--autostash"], verbose)
}

/// Fetch the latest changes from the origin remote.
pub fn fetch_origin(verbose: bool) -> Result<String> {
    run_git_command("fetch", &["origin"], verbose)
}

/// Check if a remote branch exists.
/// This checks if a branch exists on the remote repository (e.g. `origin`).
pub fn remote_branch_exists(branch_name: &str, verbose: bool) -> Result<()> {
    let output = run_git_command(
        "ls-remote",
        &["--exit-code", "--heads", "origin", branch_name],
        verbose,
    );
    match output {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

/// Rebase the current branch onto the main branch.
pub fn rebase_onto_main(main_branch_name: &str, verbose: bool) -> Result<String> {
    run_git_command(
        "rebase",
        &["--autostash", &format!("origin/{}", main_branch_name)],
        verbose,
    )
}

/// Add all changes to the staging area.
pub fn add_all(verbose: bool) -> Result<String> {
    run_git_command("add", &["."], verbose)
}

/// Commit changes with a message.
pub fn commit(message: &str, verbose: bool) -> Result<String> {
    run_git_command("commit", &["-m", message], verbose)
}

/// Push changes to the remote repository.
pub fn push(verbose: bool) -> Result<String> {
    run_git_command("push", &[], verbose)
}

/// Push all tags to the remote repository.
pub fn push_tags(verbose: bool) -> Result<String> {
    run_git_command("push", &["--tags"], verbose)
}

/// Check if the branch exists locally.
pub fn branch_exists_locally(branch_name: &str, verbose: bool) -> Result<()> {
    let output = run_git_command("rev-parse", &["--verify", "--quiet", branch_name], verbose);
    match output {
        Ok(_) => Ok(()),
        Err(_) => Err(GitError::BranchNotFound(branch_name.to_string()).into()),
    }
}

/// Find a branch by name
pub fn find_branch(name: &str, r#type: &str, config: &Config, verbose: bool) -> Result<String> {
    let prefix = misc::get_branch_prefix_or_error(&config.branch_types, &r#type)?;

    let all_branches = run_git_command("branch", &["--list"], verbose)?;
    let mut found_branches: Vec<String> = Vec::new();

    for branch in all_branches.lines() {
        let trimmed_branch = branch.trim().trim_start_matches('*').trim();
        let lower_branch = trimmed_branch.to_lowercase();
        let lower_name = name.to_lowercase();
        let lower_prefix = prefix.to_lowercase();

        // Check if the branch starts with the correct prefix and ends with the name.
        // This correctly handles branches with or without issue IDs in the middle.
        if lower_branch.starts_with(&lower_prefix) && lower_branch.ends_with(&lower_name) {
            found_branches.push(trimmed_branch.to_string());
        }
    }

    match found_branches.len() {
        0 => Err(GitError::BranchNotFound(name.to_string()).into()),
        1 => Ok(found_branches.remove(0)),
        _ => Err(anyhow::anyhow!(
            "Multiple branches found matching type '{}' and name '{}':\n{}",
            r#type,
            name,
            found_branches.join("\n")
        )
        .into()),
    }
}

/// Check if the tag exists in the repository.
pub fn tag_exists(tag_name: &str, verbose: bool) -> Result<bool> {
    let output = run_git_command("tag", &["-l", tag_name], verbose)?;
    Ok(!output.is_empty())
}

/// Merge the current branch with another branch.
pub fn merge_branch(branch_name: &str, verbose: bool) -> Result<String> {
    run_git_command("merge", &["--no-ff", branch_name], verbose)
}

/// Delete a local short-lived branch.
pub fn delete_local_branch(branch_name: &str, verbose: bool) -> Result<String> {
    run_git_command("branch", &["-d", branch_name], verbose)
}

/// Delete a remote branch.
pub fn delete_remote_branch(branch_name: &str, verbose: bool) -> Result<String> {
    run_git_command("push", &["origin", "--delete", branch_name], verbose)
}

/// Get the current branch name.
pub fn get_current_branch(verbose: bool) -> Result<String> {
    run_git_command("branch", &["--show-current"], verbose)
}

/// Create a new branch from the current HEAD or a specified point.
pub fn create_branch(branch_name: &str, from_point: Option<&str>, verbose: bool) -> Result<String> {
    let mut args = vec!["-b", branch_name];
    if let Some(point) = from_point {
        args.push(point);
    }
    run_git_command("checkout", &args, verbose)
}

/// Get the hash of the current HEAD commit.
pub fn get_head_commit_hash(verbose: bool) -> Result<String> {
    run_git_command("rev-parse", &["HEAD"], verbose)
}

/// Get the latest tag in the repository.
/// This returns the most recent tag, which is useful for versioning.
pub fn get_latest_tag(verbose: bool) -> Result<String> {
    run_git_command("describe", &["--tags", "--abbrev=0"], verbose)
}

/// Get the commit history in a specific range.
pub fn get_commit_history(range: &str, verbose: bool) -> Result<String> {
    run_git_command("log", &[range, "--pretty=format:%H|%s"], verbose)
}

/// Get the remote URL of the repository.
pub fn get_remote_url(verbose: bool) -> Result<String> {
    let url = run_git_command("remote", &["get-url", "origin"], verbose)?;
    // Remove the .git suffix for cleaner URLs
    Ok(url.trim_end_matches(".git").to_string())
}

/// Create a new tag with a message at a specific commit hash.
pub fn create_tag(
    tag_name: &str,
    message: &str,
    commit_hash: &str,
    verbose: bool,
) -> Result<String> {
    run_git_command(
        "tag",
        &["-a", tag_name, "-m", message, commit_hash],
        verbose,
    )
}

/// Push a new branch to the remote repository and set it as upstream.
/// This is useful for new branches that have not been pushed before.
pub fn push_set_upstream(branch_name: &str, verbose: bool) -> Result<String> {
    run_git_command("push", &["--set-upstream", "origin", branch_name], verbose)
}

/// Show the current status of the repository.
pub fn status(verbose: bool) -> Result<String> {
    run_git_command("status", &["--short"], verbose)
}

/// Show recent commits in the repository, 15 by default.
pub fn log_graph(verbose: bool) -> Result<String> {
    run_git_command("log", &["--graph", "--oneline", "-n", "15"], verbose)
}

/// Check if the current dir is a valid Git repository.
pub fn is_git_repository(verbose: bool) -> Result<String> {
    run_git_command("rev-parse", &["--is-inside-work-tree"], verbose)
}

/// Find the root directory of the Git repository and return its path.
pub fn get_git_root(verbose: bool) -> Result<String> {
    run_git_command("rev-parse", &["--show-toplevel"], verbose)
}

/// Initialise a new Git repository in the current directory.
pub fn init_git_repository(verbose: bool) -> Result<String> {
    run_git_command("init", &[], verbose)
}

/// Check for stale branches in the repository.
pub fn get_stale_branches(
    verbose: bool,
    main_branch: &str,
    stale_days: i64,
) -> Result<Vec<(String, i64)>> {
    let now = Utc::now();
    let day_in_seconds = stale_days * 24 * 60 * 60;

    let output = run_git_command(
        "for-each-ref",
        &[
            "--format",
            "%(refname:short)|%(committerdate:iso8601-strict)",
            "refs/heads/",
        ],
        verbose,
    )?;
    let stale_branches = output
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 2 {
                let branch_name = parts[0].to_string();
                if branch_name == main_branch {
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

/// Unit tests for the Git module.
/// These tests check if Git is installed, if the run_git_command function works correctly,
/// and if the status function returns expected results.
#[cfg(test)]
mod tests {
    use super::*;

    /// Test if Git is installed and available in the system PATH.
    #[test]
    fn test_git_is_installed() {
        let result = std::process::Command::new("git").arg("--version").output();
        assert!(result.is_ok(), "Git is not installed or not in PATH");
        let output = result.unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("git version"),
            "Unexpected output: {}",
            stdout
        );
    }

    /// Test the run_git_command function with a simple command
    #[test]
    fn test_run_git_command_version() {
        let verbose = true;
        let result = run_git_command("--version", &[], verbose);
        assert!(result.is_ok(), "Expected Ok, got {:?}", result);
        let output = result.unwrap();
        assert!(output.contains("git version"), "Output was: {}", output);
    }

    /// Test the status function
    #[test]
    fn test_status() {
        let verbose = true;
        let result = status(verbose);
        assert!(result.is_ok(), "Expected Ok, got {:?}", result);
        let output = result.unwrap();
        // Accept any output (including empty if clean)
        assert!(
            output.is_empty()
                || output.contains("M ")
                || output.contains("A ")
                || output.contains("D "),
            "Unexpected status output: {}",
            output
        );
    }
}
