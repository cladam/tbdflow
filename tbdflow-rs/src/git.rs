// This file is part of tbdflow, a CLI tool for Trunk-Based Development workflows.

use super::*; // Make parent's items available
use std::io::{self, Write};
use std::process::{Command, Stdio};

// A type alias for our result type for cleaner code.
type CommandResult = Result<String, String>;

/// Runs a Git command with the specified subcommand and arguments.
///
/// # Arguments
///
/// * `command` - The Git subcommand to execute (e.g., `"checkout"`, `"status"`).
/// * `args` - A slice of argument strings to pass to the Git command.
///
/// # Returns
///
/// * `Ok(String)` containing the trimmed standard output if the command succeeds.
/// * `Err(String)` containing the trimmed standard error if the command fails.
///
fn run_git_command(command: &str, args: &[&str]) -> CommandResult {
    println!("[RUNNING] git {} {}", command, args.join(" "));
    let output = Command::new("git")
        .arg(command)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

// -- Public Git workflow functions --
// These functions provide a high-level interface to common Git operations.

/// Check out the main branch.
pub fn checkout_main() -> CommandResult {
    run_git_command("checkout", &["main"])
}

/// Pull the latest changes with rebase.
pub fn pull_latest_with_rebase() -> CommandResult {
    run_git_command("pull", &["--rebase"])
}

/// Add all changes to the staging area.
pub fn add_all() -> CommandResult {
    run_git_command("add", &["."])
}

/// Commit changes with a message.
pub fn commit(message: &str) -> CommandResult {
    run_git_command("commit", &["-m", message])
}

/// Push changes to the remote repository.
pub fn push() -> CommandResult {
    run_git_command("push", &[])
}

/// Merge the current branch with another branch.
pub fn merge_branch(branch_name: &str) -> CommandResult {
    run_git_command("merge", &["--no-ff", branch_name])
}

/// Delete a local short-lived branch.
pub fn delete_local_branch(branch_name: &str) -> CommandResult {
    run_git_command("branch", &["-d", branch_name])
}

/// Delete a remote branch.
pub fn delete_remote_branch(branch_name: &str) -> CommandResult {
    run_git_command("push", &["origin", "--delete", branch_name])
}

/// Get the current branch name.
pub fn get_current_branch() -> CommandResult {
    run_git_command("rev-parse", &["--abbrev-ref", "HEAD"])
}

/// Create a new branch from the current HEAD or a specified point.
pub fn create_branch(branch_name: &str, from_point: Option<&str>) -> CommandResult {
    let mut args = vec!["-b", branch_name];
    if let Some(point) = from_point {
        args.push(point);
    }
    run_git_command("checkout", &args)
}

/// Show the current status of the repository.
pub fn status() -> CommandResult {
    run_git_command("status", &[])
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the run_git_command function with a simple command
    #[test]
    fn test_run_git_command_version() {
        let result = run_git_command("--version", &[]);
        assert!(result.is_ok(), "Expected Ok, got {:?}", result);
        let output = result.unwrap();
        assert!(output.contains("git version"), "Output was: {}", output);
    }
}