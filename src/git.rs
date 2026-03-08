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

/// Pull the latest changes using fast-forward only.
/// Unlike rebase, this preserves existing commit SHAs.
/// Fails if the local branch has diverged from the remote.
pub fn pull_fast_forward_only(verbose: bool, dry_run: bool) -> Result<String> {
    run_git_command("pull", &["--ff-only"], verbose, dry_run)
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

/// Revert a specific commit by its SHA, creating a new revert commit.
pub fn revert_commit(commit_hash: &str, verbose: bool, dry_run: bool) -> Result<String> {
    run_git_command("revert", &["--no-edit", commit_hash], verbose, dry_run)
}

// ── Radar helpers ──────────────────────────────────────────────────────────

/// List remote branches that have NOT been merged into the main branch.
/// Returns branch names without the `origin/` prefix.
pub fn get_active_remote_branches(
    main_branch: &str,
    verbose: bool,
    dry_run: bool,
) -> Result<Vec<String>> {
    let main_ref = format!("origin/{}", main_branch);
    let output = run_git_command(
        "branch",
        &["-r", "--no-merged", &main_ref],
        verbose,
        dry_run,
    )?;
    let branches = output
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.contains("->")) // skip HEAD -> origin/main
        .filter(|l| l.starts_with("origin/"))
        .filter(|l| l.trim_start_matches("origin/") != main_branch)
        .map(|l| l.trim_start_matches("origin/").to_string())
        .collect();
    Ok(branches)
}

/// Get the list of files changed between two refs using three-dot diff (merge-base).
/// Useful for finding what a branch changed relative to its fork point from main.
pub fn get_diff_files_between_refs(
    base: &str,
    head: &str,
    verbose: bool,
    dry_run: bool,
) -> Result<Vec<String>> {
    let range = format!("{}...{}", base, head);
    let output = run_git_command("diff", &["--name-only", &range], verbose, dry_run)?;
    Ok(output
        .lines()
        .filter(|l| !l.is_empty())
        .map(|s| s.to_string())
        .collect())
}

/// A range of lines touched in a diff hunk.
#[derive(Debug, Clone)]
pub struct HunkRange {
    pub start_line: u32,
    pub line_count: u32,
}

impl HunkRange {
    /// Check whether two hunk ranges overlap.
    pub fn overlaps(&self, other: &HunkRange) -> bool {
        let self_end = self.start_line + self.line_count.max(1) - 1;
        let other_end = other.start_line + other.line_count.max(1) - 1;
        self.start_line <= other_end && other.start_line <= self_end
    }
}

/// Parse unified-diff `@@ -a,b +c,d @@` headers into `HunkRange` values.
/// When `side` is `New`, returns the `+c,d` (new file) ranges.
/// When `side` is `Old`, returns the `-a,b` (old file) ranges.
#[derive(Debug, Clone, Copy)]
pub enum DiffSide {
    Old,
    New,
}

fn parse_hunk_headers(diff_output: &str, side: DiffSide) -> Vec<HunkRange> {
    diff_output
        .lines()
        .filter(|l| l.starts_with("@@"))
        .filter_map(|line| {
            // Format: @@ -a,b +c,d @@
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 4 {
                return None;
            }
            let token = match side {
                DiffSide::Old => parts[1].trim_start_matches('-'),
                DiffSide::New => parts[2].trim_start_matches('+'),
            };
            let nums: Vec<&str> = token.split(',').collect();
            let start: u32 = nums[0].parse().ok()?;
            let count: u32 = if nums.len() > 1 {
                nums[1].parse().ok()?
            } else {
                1
            };
            Some(HunkRange {
                start_line: start,
                line_count: count,
            })
        })
        .collect()
}

/// Get line-level diff hunks between two refs for a specific file.
/// Returns the NEW-side hunk ranges (lines added/modified in `head`).
pub fn get_diff_hunks_between_refs(
    base: &str,
    head: &str,
    file: &str,
    verbose: bool,
    dry_run: bool,
) -> Result<Vec<HunkRange>> {
    let range = format!("{}...{}", base, head);
    let output = run_git_command("diff", &["-U0", &range, "--", file], verbose, dry_run)?;
    Ok(parse_hunk_headers(&output, DiffSide::New))
}

/// Get the author name of the most recent commit on a remote branch.
pub fn get_branch_author(branch: &str, verbose: bool, dry_run: bool) -> Result<String> {
    let ref_name = format!("origin/{}", branch);
    run_git_command("log", &["-1", "--format=%an", &ref_name], verbose, dry_run)
}

/// Get the number of commits a remote branch is ahead of origin/main.
pub fn get_remote_branch_commit_count(
    branch: &str,
    main_branch: &str,
    verbose: bool,
    dry_run: bool,
) -> Result<u32> {
    let range = format!("origin/{}..origin/{}", main_branch, branch);
    let output = run_git_command("rev-list", &["--count", &range], verbose, dry_run)?;
    Ok(output.trim().parse().unwrap_or(0))
}

/// Get all locally changed files (both staged and unstaged, plus untracked).
pub fn get_local_changed_files(verbose: bool, dry_run: bool) -> Result<Vec<String>> {
    let mut files = std::collections::HashSet::new();

    // Unstaged modifications
    let unstaged = run_git_command("diff", &["--name-only"], verbose, dry_run)?;
    for f in unstaged.lines().filter(|l| !l.is_empty()) {
        files.insert(f.to_string());
    }

    // Staged modifications
    let staged = run_git_command("diff", &["--name-only", "--staged"], verbose, dry_run)?;
    for f in staged.lines().filter(|l| !l.is_empty()) {
        files.insert(f.to_string());
    }

    Ok(files.into_iter().collect())
}

/// Get line-level diff hunks for local (unstaged + staged) changes in a specific file.
/// Returns NEW-side hunk ranges.
pub fn get_local_diff_hunks(file: &str, verbose: bool, dry_run: bool) -> Result<Vec<HunkRange>> {
    let mut hunks = Vec::new();

    // Unstaged changes
    let unstaged = run_git_command("diff", &["-U0", "--", file], verbose, dry_run)?;
    hunks.extend(parse_hunk_headers(&unstaged, DiffSide::New));

    // Staged changes
    let staged = run_git_command("diff", &["-U0", "--staged", "--", file], verbose, dry_run)?;
    hunks.extend(parse_hunk_headers(&staged, DiffSide::New));

    Ok(hunks)
}

// ── End radar helpers ──────────────────────────────────────────────────────

/// Check if a commit is an ancestor of the given branch (i.e. the commit exists on that branch).
/// Resolves the commit hash and uses the fully-qualified branch ref to avoid ambiguity
/// (e.g. when a tag has the same name as the branch).
pub fn is_ancestor_of(
    commit_hash: &str,
    branch: &str,
    verbose: bool,
    dry_run: bool,
) -> Result<bool> {
    // Resolve to a full hash — short SHAs can be unreliable with merge-base
    let full_hash = run_git_command("rev-parse", &["--verify", commit_hash], verbose, dry_run)?;
    // In dry-run mode rev-parse returns "" so we just assume it's fine
    if full_hash.is_empty() {
        return Ok(true);
    }
    // Use refs/heads/ to unambiguously refer to the local branch,
    // avoiding conflicts with tags or other refs that share the same name.
    let qualified_branch = format!("refs/heads/{}", branch);
    let status = run_git_status_check(
        "merge-base",
        &["--is-ancestor", &full_hash, &qualified_branch],
        verbose,
        dry_run,
    )?;
    Ok(status.code() == Some(0))
}

/// Get the full subject line of a commit.
pub fn get_commit_subject(commit_hash: &str, verbose: bool, dry_run: bool) -> Result<String> {
    run_git_command("log", &["-1", "--format=%s", commit_hash], verbose, dry_run)
}

/// Verify that a commit SHA exists in the repository.
pub fn commit_exists(commit_hash: &str, verbose: bool, dry_run: bool) -> Result<bool> {
    // Use rev-parse --verify which exits non-zero if the ref doesn't exist.
    // run_git_command respects dry-run (returns Ok("")) so we assume it exists in that mode.
    match run_git_command("rev-parse", &["--verify", commit_hash], verbose, dry_run) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

// ── Pre-flight CI status check ─────────────────────────────────────────────

/// The CI status of the latest commit on a branch.
#[derive(Debug, PartialEq)]
pub enum CiStatus {
    /// All checks passed.
    Green,
    /// One or more checks failed.
    Failed,
    /// Checks are still running.
    Pending,
    /// Unable to determine status (gh CLI missing, no CI configured, etc.).
    Unknown(String),
}

/// Check the CI status of the latest commit on the given branch using the `gh` CLI.
///
/// Uses `gh api` to query the combined commit status and check-runs for the
/// branch tip. Falls back gracefully if `gh` is not installed or if the repo
/// has no CI configured.
pub fn check_ci_status(branch: &str, verbose: bool, dry_run: bool) -> CiStatus {
    if dry_run {
        if verbose {
            println!("{}", "[DRY RUN] Would check CI status via gh CLI".yellow());
        }
        return CiStatus::Green;
    }

    // First, check if `gh` CLI is available
    if Command::new("gh")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_err()
    {
        return CiStatus::Unknown("gh CLI is not installed".to_string());
    }

    if verbose {
        println!(
            "{} Checking CI status for branch '{}'...",
            "[PRE-FLIGHT]".cyan(),
            branch
        );
    }

    // Use `gh run list` to query the status of the latest workflow run on the branch.
    // This gives us the overall conclusion of the most recent CI run.
    let output = Command::new("gh")
        .args([
            "run",
            "list",
            "--branch",
            branch,
            "--limit",
            "1",
            "--json",
            "status,conclusion",
            "--jq",
            ".[0] | .status + \"/\" + .conclusion",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    let output = match output {
        Ok(o) => o,
        Err(e) => {
            return CiStatus::Unknown(format!("Failed to run gh CLI: {}", e));
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        // If the command failed because there are no workflow runs, treat as unknown
        return CiStatus::Unknown(format!("gh run list failed: {}", stderr));
    }

    let result = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if verbose {
        println!("{} gh run status: {}", "[PRE-FLIGHT]".cyan(), result);
    }

    if result.is_empty() || result == "/" || result == "null/null" {
        return CiStatus::Unknown("No CI runs found for this branch".to_string());
    }

    // Parse the status/conclusion pair
    let parts: Vec<&str> = result.splitn(2, '/').collect();
    let status = parts.first().unwrap_or(&"");
    let conclusion = parts.get(1).unwrap_or(&"");

    match (*status, *conclusion) {
        ("completed", "success") => CiStatus::Green,
        ("completed", "failure") | ("completed", "timed_out") | ("completed", "cancelled") => {
            CiStatus::Failed
        }
        ("in_progress", _) | ("queued", _) | ("waiting", _) | ("pending", _) => CiStatus::Pending,
        ("completed", "skipped") | ("completed", "neutral") => CiStatus::Green,
        _ => CiStatus::Unknown(format!("Unexpected CI state: {}/{}", status, conclusion)),
    }
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

    /// Test that check_ci_status returns Green in dry-run mode
    #[test]
    fn test_ci_status_dry_run_returns_green() {
        let result = check_ci_status("main", false, true);
        assert_eq!(result, CiStatus::Green);
    }

    /// Test that CiStatus variants have the expected equality behavior
    #[test]
    fn test_ci_status_equality() {
        assert_eq!(CiStatus::Green, CiStatus::Green);
        assert_eq!(CiStatus::Failed, CiStatus::Failed);
        assert_eq!(CiStatus::Pending, CiStatus::Pending);
        assert_ne!(CiStatus::Green, CiStatus::Failed);
        assert_ne!(CiStatus::Green, CiStatus::Pending);
        assert_ne!(CiStatus::Failed, CiStatus::Pending);
    }
}
