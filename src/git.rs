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
fn run_git_command(command: &str, args: &[&str], verbose: bool, dry_run: bool) -> Result<String> {
    if verbose || dry_run {
        if dry_run {
            println!(
                "{}",
                "[DRY RUN] Command would execute but no changes made".yellow()
            );
            println!("git {} {}", command, args.join(" "));
            println!(); // Add blank line for spacing
            return Ok(String::new());
        } else {
            println!("{} git {} {}", "[RUNNING] ".cyan(), command, args.join(" "));
        }
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
pub fn is_working_directory_clean(verbose: bool, dry_run: bool) -> Result<()> {
    let output = run_git_command("status", &["--porcelain"], verbose, dry_run)?;
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
    _dry_run: bool,
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
pub fn has_staged_changes(verbose: bool, dry_run: bool) -> Result<bool> {
    let status = run_git_status_check("diff", &["--staged", "--quiet"], verbose, dry_run)?;
    // `git diff --quiet` exits with 1 if there are changes, 0 if not.
    Ok(status.code() == Some(1))
}

/// Add a new remote repository to the current Git repository.
/// git remote add origin <your-repository-url>
pub fn add_remote(
    remote_name: &str,
    remote_url: &str,
    verbose: bool,
    dry_run: bool,
) -> Result<String> {
    run_git_command(
        "remote",
        &["add", remote_name, remote_url],
        verbose,
        dry_run,
    )
}

/// Check out the main branch.
pub fn checkout_main(verbose: bool, dry_run: bool, main_branch: &str) -> Result<String> {
    run_git_command("checkout", &[main_branch], verbose, dry_run)
}

/// Pull the latest changes with rebase.
pub fn pull_latest_with_rebase(verbose: bool, dry_run: bool) -> Result<String> {
    // Using --autostash to safely handle local changes before pulling.
    run_git_command("pull", &["--rebase", "--autostash"], verbose, dry_run)
}

/// Fetch the latest changes from the origin remote.
pub fn fetch_origin(verbose: bool, dry_run: bool) -> Result<String> {
    run_git_command("fetch", &["origin"], verbose, dry_run)
}

/// Check if a remote branch exists.
/// This checks if a branch exists on the remote repository (e.g. `origin`).
pub fn remote_branch_exists(branch_name: &str, verbose: bool, dry_run: bool) -> Result<()> {
    let output = run_git_command(
        "ls-remote",
        &["--exit-code", "--heads", "origin", branch_name],
        verbose,
        dry_run,
    );
    match output {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

/// Rebase the current branch onto the main branch.
pub fn rebase_onto_main(main_branch_name: &str, verbose: bool, dry_run: bool) -> Result<String> {
    run_git_command(
        "rebase",
        &["--autostash", &format!("origin/{}", main_branch_name)],
        verbose,
        dry_run,
    )
}

/// Add all changes to the staging area.
pub fn add_all(verbose: bool, dry_run: bool) -> Result<String> {
    run_git_command("add", &["."], verbose, dry_run)
}

// Add all changes except those in specified project directories.
// This uses the `:!<dir>` syntax to exclude directories from being added.
pub fn add_excluding_projects(
    project_dirs: &[String],
    verbose: bool,
    dry_run: bool,
) -> Result<String> {
    let mut args = vec!["."];
    // Use the more explicit and robust `:(exclude)` pathspec syntax.
    let exclude_args: Vec<String> = project_dirs
        .iter()
        .map(|dir| format!(":(exclude){}/", dir))
        .collect();

    let exclude_args_str: Vec<&str> = exclude_args.iter().map(|s| s.as_str()).collect();

    args.extend_from_slice(&exclude_args_str);

    if verbose {
        println!("Excluded dirs: \n{:#?}", args);
    }

    run_git_command("add", &args, verbose, dry_run)
}

/// Commit changes with a message.
pub fn commit(message: &str, verbose: bool, dry_run: bool) -> Result<String> {
    run_git_command("commit", &["-m", message], verbose, dry_run)
}

/// Push changes to the remote repository.
pub fn push(verbose: bool, dry_run: bool) -> Result<String> {
    run_git_command("push", &[], verbose, dry_run)
}

/// Push all tags to the remote repository.
pub fn push_tags(verbose: bool, dry_run: bool) -> Result<String> {
    run_git_command("push", &["--tags"], verbose, dry_run)
}

/// Check if the branch exists locally.
pub fn branch_exists_locally(branch_name: &str, verbose: bool, dry_run: bool) -> Result<()> {
    let output = run_git_command(
        "rev-parse",
        &["--verify", "--quiet", branch_name],
        verbose,
        dry_run,
    )?;
    match output {
        _ if output.is_empty() => Err(GitError::BranchNotFound(branch_name.to_string()).into()),
        _ => Ok(()),
    }
}

/// Find a branch by name
pub fn find_branch(
    name: &str,
    r#type: &str,
    config: &Config,
    verbose: bool,
    dry_run: bool,
) -> Result<String> {
    let prefix = misc::get_branch_prefix_or_error(&config.branch_types, &r#type)?;

    let all_branches = run_git_command("branch", &["--list"], verbose, dry_run)?;
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
pub fn tag_exists(tag_name: &str, verbose: bool, dry_run: bool) -> Result<bool> {
    let output = run_git_command("tag", &["-l", tag_name], verbose, dry_run)?;
    Ok(!output.is_empty())
}

/// Merge the current branch with another branch.
pub fn merge_branch(branch_name: &str, verbose: bool, dry_run: bool) -> Result<String> {
    run_git_command("merge", &["--no-ff", branch_name], verbose, dry_run)
}

/// Delete a local short-lived branch.
pub fn delete_local_branch(branch_name: &str, verbose: bool, dry_run: bool) -> Result<String> {
    run_git_command("branch", &["-d", branch_name], verbose, dry_run)
}

/// Delete a remote branch.
pub fn delete_remote_branch(branch_name: &str, verbose: bool, dry_run: bool) -> Result<String> {
    run_git_command(
        "push",
        &["origin", "--delete", branch_name],
        verbose,
        dry_run,
    )
}

/// Get the current branch name.
pub fn get_current_branch(verbose: bool, dry_run: bool) -> Result<String> {
    run_git_command("branch", &["--show-current"], verbose, dry_run)
}

/// Create a new branch from the current HEAD or a specified point.
pub fn create_branch(
    branch_name: &str,
    from_point: Option<&str>,
    verbose: bool,
    dry_run: bool,
) -> Result<String> {
    let mut args = vec!["-b", branch_name];
    if let Some(point) = from_point {
        args.push(point);
    }
    run_git_command("checkout", &args, verbose, dry_run)
}

/// Get the hash of the current HEAD commit.
pub fn get_head_commit_hash(verbose: bool, dry_run: bool) -> Result<String> {
    run_git_command("rev-parse", &["HEAD"], verbose, dry_run)
}

/// Get the latest tag in the repository.
/// This returns the most recent tag, which is useful for versioning.
pub fn get_latest_tag(verbose: bool, dry_run: bool) -> Result<String> {
    run_git_command("describe", &["--tags", "--abbrev=0"], verbose, dry_run)
}

/// Get the commit history in a specific range.
pub fn get_commit_history(range: &str, verbose: bool, dry_run: bool) -> Result<String> {
    run_git_command("log", &[range, "--pretty=format:%H|%s"], verbose, dry_run)
}

/// Get the remote URL of the repository.
pub fn get_remote_url(verbose: bool, dry_run: bool) -> Result<String> {
    let url = run_git_command("remote", &["get-url", "origin"], verbose, dry_run)?;
    // Remove the .git suffix for cleaner URLs
    Ok(url.trim_end_matches(".git").to_string())
}

/// Create a new tag with a message at a specific commit hash.
pub fn create_tag(
    tag_name: &str,
    message: &str,
    commit_hash: &str,
    verbose: bool,
    dry_run: bool,
) -> Result<String> {
    run_git_command(
        "tag",
        &["-a", tag_name, "-m", message, commit_hash],
        verbose,
        dry_run,
    )
}

/// Push a new branch to the remote repository and set it as upstream.
/// This is useful for new branches that have not been pushed before.
pub fn push_set_upstream(branch_name: &str, verbose: bool, dry_run: bool) -> Result<String> {
    run_git_command(
        "push",
        &["--set-upstream", "origin", branch_name],
        verbose,
        dry_run,
    )
}

pub fn get_status_short(verbose: bool, dry_run: bool) -> Result<String> {
    run_git_command("status", &["--short"], verbose, dry_run)
}

pub fn get_status_full(verbose: bool, dry_run: bool) -> Result<String> {
    run_git_command("status", &[], verbose, dry_run)
}
/// Show the current status of the repository.
pub fn status(verbose: bool, dry_run: bool) -> Result<String> {
    run_git_command("status", &["--short"], verbose, dry_run)
}

pub fn status_for_path(relative_path: &str, verbose: bool, dry_run: bool) -> Result<String> {
    run_git_command(
        "status",
        &["--short", "--", relative_path],
        verbose,
        dry_run,
    )
}

/// Show the current status of the repository, excluding changes in specified project directories.
pub fn status_excluding_projects(
    project_dirs: &[String],
    verbose: bool,
    dry_run: bool,
) -> Result<String> {
    let mut args = vec!["--short", "--"];
    let exclude_args: Vec<String> = project_dirs
        .iter()
        .map(|dir| format!(":(exclude){}/", dir))
        .collect();

    let exclude_args_str: Vec<&str> = exclude_args.iter().map(|s| s.as_str()).collect();

    args.extend_from_slice(&exclude_args_str);

    run_git_command("status", &args, verbose, dry_run)
}

/// Show recent commits in the repository, 15 by default.
pub fn log_graph(verbose: bool, dry_run: bool) -> Result<String> {
    run_git_command(
        "log",
        &["--graph", "--oneline", "-n", "15"],
        verbose,
        dry_run,
    )
}

/// Get the commit count of the current branch ahead of the main branch.
pub fn get_commit_count_ahead(
    branch: &str,
    main_branch: &str,
    verbose: bool,
    dry_run: bool,
) -> Result<String> {
    let range = format!("origin/{}..{}", main_branch, branch);
    run_git_command("rev-list", &["--count", &range], verbose, dry_run)
}

/// Get the log for a specific branch.
pub fn get_branch_log(branch: &str, verbose: bool, dry_run: bool) -> Result<String> {
    let range = format!("origin/main..{}", branch);
    run_git_command("log", &["--oneline", "-n", "10", &range], verbose, dry_run)
}

/// Check if the current dir is a valid Git repository.
pub fn is_git_repository(verbose: bool, dry_run: bool) -> Result<String> {
    run_git_command("rev-parse", &["--is-inside-work-tree"], verbose, dry_run)
}

/// Find the root directory of the Git repository and return its path.
pub fn get_git_root(verbose: bool, dry_run: bool) -> Result<String> {
    run_git_command("rev-parse", &["--show-toplevel"], verbose, dry_run)
}

/// Initialise a new Git repository in the current directory.
pub fn init_git_repository(verbose: bool, dry_run: bool) -> Result<String> {
    run_git_command("init", &[], verbose, dry_run)
}

/// Check for stale branches in the repository.
pub fn get_stale_branches(
    verbose: bool,
    dry_run: bool,
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
        dry_run,
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

/// Get the current git username.
pub fn get_user_name(verbose: bool, dry_run: bool) -> Result<String> {
    run_git_command("config", &["user.name"], verbose, dry_run)
}

/// Get the commit message for a specific commit hash.
pub fn get_commit_message(commit_hash: &str, verbose: bool, dry_run: bool) -> Result<String> {
    run_git_command("log", &["-1", "--format=%s", commit_hash], verbose, dry_run)
}

/// Get the commit log since a given date/time.
/// Returns format: hash|author|subject
pub fn get_log_since(since: &str, verbose: bool, dry_run: bool) -> Result<String> {
    run_git_command(
        "log",
        &["--since", since, "--pretty=format:%H|%an|%s"],
        verbose,
        dry_run,
    )
}

/// Get the list of files changed in a specific commit.
/// Returns a vector of file paths relative to the repository root.
pub fn get_changed_files(commit_hash: &str, verbose: bool, dry_run: bool) -> Result<Vec<String>> {
    let output = run_git_command(
        "diff-tree",
        &["--no-commit-id", "--name-only", "-r", commit_hash],
        verbose,
        dry_run,
    )?;

    Ok(output
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect())
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
        let dry_run = false;
        let result = run_git_command("--version", &[], verbose, dry_run);
        assert!(result.is_ok(), "Expected Ok, got {:?}", result);
        let output = result.unwrap();
        assert!(output.contains("git version"), "Output was: {}", output);
    }

    /// Test the status function
    #[test]
    fn test_status() {
        let verbose = true;
        let dry_run = false;
        let result = status(verbose, dry_run);
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
