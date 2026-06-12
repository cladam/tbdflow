use crate::commands;
use crate::config::Config;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use colored::Colorize;
use std::process::{Command, Stdio};
use thiserror::Error;

/// Execution options threaded through every git operation.
#[derive(Debug, Clone, Copy)]
pub struct RunOpts {
    pub verbose: bool,
    pub dry_run: bool,
}

impl RunOpts {
    pub fn new(verbose: bool, dry_run: bool) -> Self {
        Self { verbose, dry_run }
    }
}

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
fn run_git_command(command: &str, args: &[&str], opts: RunOpts) -> Result<String> {
    if opts.verbose || opts.dry_run {
        if opts.dry_run {
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
pub fn is_working_directory_clean(opts: RunOpts) -> Result<()> {
    let output = run_git_command("status", &["--porcelain"], opts)?;
    if output.is_empty() {
        Ok(())
    } else {
        Err(GitError::DirectoryNotClean(
            "You have unstaged changes. Please commit or stash them first.".to_string(),
        )
        .into())
    }
}

/// Runs a git command, suppressing stdout/stderr. Returns the exit status.
fn run_git_status_check(
    command: &str,
    args: &[&str],
    opts: RunOpts,
) -> Result<std::process::ExitStatus> {
    if opts.verbose {
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
pub fn has_staged_changes(opts: RunOpts) -> Result<bool> {
    let status = run_git_status_check("diff", &["--staged", "--quiet"], opts)?;
    // git diff --quiet exits 1 if there are changes, 0 if clean.
    Ok(status.code() == Some(1))
}

pub fn add_remote(
    remote_name: &str,
    remote_url: &str,
    opts: RunOpts,
) -> Result<String> {
    run_git_command(
        "remote",
        &["add", remote_name, remote_url],
        opts,
    )
}

pub fn checkout_main(opts: RunOpts, main_branch: &str) -> Result<String> {
    run_git_command("checkout", &[main_branch], opts)
}

pub fn pull_latest_with_rebase(opts: RunOpts) -> Result<String> {
    run_git_command("pull", &["--rebase", "--autostash"], opts)
}

/// Fast-forward only — preserves existing commit SHAs.
/// Fails if the local branch has diverged.
pub fn pull_fast_forward_only(opts: RunOpts) -> Result<String> {
    run_git_command("pull", &["--ff-only"], opts)
}

pub fn fetch_origin(opts: RunOpts) -> Result<String> {
    run_git_command("fetch", &["origin"], opts)
}

pub fn remote_branch_exists(branch_name: &str, opts: RunOpts) -> Result<()> {
    let output = run_git_command(
        "ls-remote",
        &["--exit-code", "--heads", "origin", branch_name],
        opts,
    );
    match output {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn rebase_onto_main(main_branch_name: &str, opts: RunOpts) -> Result<String> {
    run_git_command(
        "rebase",
        &["--autostash", &format!("origin/{}", main_branch_name)],
        opts,
    )
}

pub fn add_all(opts: RunOpts) -> Result<String> {
    run_git_command("add", &["."], opts)
}

/// Stages everything except the given project directories using `:(exclude)` pathspec.
pub fn add_excluding_projects(
    project_dirs: &[String],
    opts: RunOpts,
) -> Result<String> {
    let mut args = vec!["."];
    // Use the more explicit and robust `:(exclude)` pathspec syntax.
    let exclude_args: Vec<String> = project_dirs
        .iter()
        .map(|dir| format!(":(exclude){}/", dir))
        .collect();

    let exclude_args_str: Vec<&str> = exclude_args.iter().map(|s| s.as_str()).collect();

    args.extend_from_slice(&exclude_args_str);

    if opts.verbose {
        println!("Excluded dirs: \n{:#?}", args);
    }

    run_git_command("add", &args, opts)
}

pub fn commit(message: &str, opts: RunOpts) -> Result<String> {
    run_git_command("commit", &["-m", message], opts)
}

pub fn push(opts: RunOpts) -> Result<String> {
    run_git_command("push", &[], opts)
}

pub fn push_tags(opts: RunOpts) -> Result<String> {
    run_git_command("push", &["--tags"], opts)
}

pub fn branch_exists_locally(branch_name: &str, opts: RunOpts) -> Result<()> {
    let output = run_git_command(
        "rev-parse",
        &["--verify", "--quiet", branch_name],
        opts,
    )?;
    match output {
        _ if output.is_empty() => Err(GitError::BranchNotFound(branch_name.to_string()).into()),
        _ => Ok(()),
    }
}

/// Fuzzy-matches a branch by type prefix and trailing name.
/// Handles branches with or without issue IDs in the middle.
pub fn find_branch(
    name: &str,
    r#type: &str,
    config: &Config,
    opts: RunOpts,
) -> Result<String> {
    let prefix = commands::get_branch_prefix_or_error(&config.branch_types, r#type)?;

    let all_branches = run_git_command("branch", &["--list"], opts)?;
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
        )),
    }
}

pub fn tag_exists(tag_name: &str, opts: RunOpts) -> Result<bool> {
    let output = run_git_command("tag", &["-l", tag_name], opts)?;
    Ok(!output.is_empty())
}

pub fn merge_branch(branch_name: &str, opts: RunOpts) -> Result<String> {
    run_git_command("merge", &["--no-ff", branch_name], opts)
}

pub fn delete_local_branch(branch_name: &str, opts: RunOpts) -> Result<String> {
    run_git_command("branch", &["-d", branch_name], opts)
}

pub fn delete_remote_branch(branch_name: &str, opts: RunOpts) -> Result<String> {
    run_git_command(
        "push",
        &["origin", "--delete", branch_name],
        opts,
    )
}

pub fn get_current_branch(opts: RunOpts) -> Result<String> {
    run_git_command("branch", &["--show-current"], opts)
}

pub fn create_branch(
    branch_name: &str,
    from_point: Option<&str>,
    opts: RunOpts,
) -> Result<String> {
    let mut args = vec!["-b", branch_name];
    if let Some(point) = from_point {
        args.push(point);
    }
    run_git_command("checkout", &args, opts)
}

pub fn get_head_commit_hash(opts: RunOpts) -> Result<String> {
    run_git_command("rev-parse", &["HEAD"], opts)
}

pub fn get_latest_tag(opts: RunOpts) -> Result<String> {
    run_git_command("describe", &["--tags", "--abbrev=0"], opts)
}

pub fn get_commit_history(range: &str, opts: RunOpts) -> Result<String> {
    run_git_command("log", &[range, "--pretty=format:%H|%s"], opts)
}

pub fn get_remote_url(opts: RunOpts) -> Result<String> {
    let url = run_git_command("remote", &["get-url", "origin"], opts)?;
    Ok(url.trim_end_matches(".git").to_string())
}

pub fn create_tag(
    tag_name: &str,
    message: &str,
    commit_hash: &str,
    opts: RunOpts,
) -> Result<String> {
    run_git_command(
        "tag",
        &["-a", tag_name, "-m", message, commit_hash],
        opts,
    )
}

pub fn push_set_upstream(branch_name: &str, opts: RunOpts) -> Result<String> {
    run_git_command(
        "push",
        &["--set-upstream", "origin", branch_name],
        opts,
    )
}

pub fn get_status_short(opts: RunOpts) -> Result<String> {
    run_git_command("status", &["--short"], opts)
}

pub fn get_status_full(opts: RunOpts) -> Result<String> {
    run_git_command("status", &[], opts)
}

pub fn status_for_path(relative_path: &str, opts: RunOpts) -> Result<String> {
    run_git_command(
        "status",
        &["--short", "--", relative_path],
        opts,
    )
}

/// Status excluding the given project directories (monorepo root use).
pub fn status_excluding_projects(
    project_dirs: &[String],
    opts: RunOpts,
) -> Result<String> {
    let mut args = vec!["--short", "--"];
    let exclude_args: Vec<String> = project_dirs
        .iter()
        .map(|dir| format!(":(exclude){}/", dir))
        .collect();

    let exclude_args_str: Vec<&str> = exclude_args.iter().map(|s| s.as_str()).collect();

    args.extend_from_slice(&exclude_args_str);

    run_git_command("status", &args, opts)
}

/// Monorepo-aware status: scoped to sub-project, root-only, or full.
pub fn get_scoped_status(config: &Config, opts: RunOpts) -> Result<String> {
    let git_root = std::path::PathBuf::from(get_git_root(opts)?);
    let current_dir = std::env::current_dir()?;
    let project_root = crate::config::find_project_root()?;

    if let Some(proj_root) = project_root {
        if current_dir == proj_root {
            status_for_path(".", opts)
        } else {
            let relative_path = proj_root.strip_prefix(&git_root).unwrap_or(&proj_root);
            let path_str = relative_path
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("Project path contains non-UTF-8 characters: {:?}", relative_path))?;
            status_for_path(path_str, opts)
        }
    } else if crate::config::is_monorepo_root(config, &current_dir, &git_root) {
        println!(
            "{}",
            "Monorepo root detected. Showing status for root-level files only.".yellow()
        );
        status_excluding_projects(&config.monorepo.project_dirs, opts)
    } else {
        get_status_short(opts)
    }
}

/// Monorepo-aware staging. At the repo root, excludes project dirs unless `include_projects` is set.
pub fn stage_scoped_changes(
    config: &Config,
    include_projects: bool,
    opts: RunOpts,
) -> Result<()> {
    let git_root = std::path::PathBuf::from(get_git_root(opts)?);
    let current_dir = std::env::current_dir()?;

    if current_dir == git_root
        && config.monorepo.enabled
        && !config.monorepo.project_dirs.is_empty()
    {
        if include_projects {
            println!(
                "{}",
                "Including all project directories in commit.".yellow()
            );
            add_all(opts)?;
        } else {
            println!(
                "{}",
                "Monorepo root detected. Staging root-level files only.".yellow()
            );
            add_excluding_projects(&config.monorepo.project_dirs, opts)?;
        }
    } else {
        add_all(opts)?;
    }

    Ok(())
}

pub fn log_graph(opts: RunOpts) -> Result<String> {
    run_git_command(
        "log",
        &["--graph", "--oneline", "-n", "15"],
        opts,
    )
}

pub fn get_commit_count_ahead(
    branch: &str,
    main_branch: &str,
    opts: RunOpts,
) -> Result<String> {
    let range = format!("origin/{}..{}", main_branch, branch);
    run_git_command("rev-list", &["--count", &range], opts)
}

pub fn get_branch_log(
    branch: &str,
    main_branch: &str,
    opts: RunOpts,
) -> Result<String> {
    let range = format!("origin/{}..{}", main_branch, branch);
    run_git_command("log", &["--oneline", "-n", "10", &range], opts)
}

pub fn is_git_repository(opts: RunOpts) -> Result<String> {
    run_git_command("rev-parse", &["--is-inside-work-tree"], opts)
}

pub fn get_git_root(opts: RunOpts) -> Result<String> {
    run_git_command("rev-parse", &["--show-toplevel"], opts)
}

pub fn init_git_repository(opts: RunOpts) -> Result<String> {
    run_git_command("init", &[], opts)
}

pub fn get_stale_branches(
    opts: RunOpts,
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
        opts,
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

pub fn get_user_name(opts: RunOpts) -> Result<String> {
    run_git_command("config", &["user.name"], opts)
}

pub fn get_commit_message(commit_hash: &str, opts: RunOpts) -> Result<String> {
    run_git_command("log", &["-1", "--format=%s", commit_hash], opts)
}

/// Returns format: `hash|author|subject`
pub fn get_log_since(since: &str, opts: RunOpts) -> Result<String> {
    run_git_command(
        "log",
        &["--since", since, "--pretty=format:%H|%an|%s"],
        opts,
    )
}

pub fn get_latest_commit_time(
    branch: &str,
    opts: RunOpts,
) -> Result<Option<DateTime<Utc>>> {
    let ref_name = format!("origin/{}", branch);
    let output = run_git_command("log", &["-1", "--format=%cI", &ref_name], opts)?;
    if output.is_empty() {
        return Ok(None);
    }
    match DateTime::parse_from_rfc3339(output.trim()) {
        Ok(dt) => Ok(Some(dt.with_timezone(&Utc))),
        Err(_) => Ok(None),
    }
}

pub fn get_file_churn(
    branch: &str,
    hours: u64,
    limit: usize,
    opts: RunOpts,
) -> Result<Vec<(String, usize)>> {
    let since = format!("{} hours ago", hours);
    let ref_name = format!("origin/{}", branch);
    let output = run_git_command(
        "log",
        &[
            &ref_name,
            "--since",
            &since,
            "--name-only",
            "--pretty=format:",
        ],
        opts,
    )?;

    let mut counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for line in output.lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            *counts.entry(trimmed.to_string()).or_insert(0) += 1;
        }
    }

    let mut sorted: Vec<(String, usize)> = counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted.truncate(limit);
    Ok(sorted)
}

pub fn get_changed_files(commit_hash: &str, opts: RunOpts) -> Result<Vec<String>> {
    let output = run_git_command(
        "diff-tree",
        &["--no-commit-id", "--name-only", "-r", commit_hash],
        opts,
    )?;

    Ok(output
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect())
}

pub fn revert_commit(commit_hash: &str, opts: RunOpts) -> Result<String> {
    run_git_command("revert", &["--no-edit", commit_hash], opts)
}

/// Remote branches not yet merged into main, without `origin/` prefix.
pub fn get_active_remote_branches(
    main_branch: &str,
    opts: RunOpts,
) -> Result<Vec<String>> {
    let main_ref = format!("origin/{}", main_branch);
    let output = run_git_command(
        "branch",
        &["-r", "--no-merged", &main_ref],
        opts,
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

/// Three-dot diff: files changed by `head` relative to its fork point from `base`.
pub fn get_diff_files_between_refs(
    base: &str,
    head: &str,
    opts: RunOpts,
) -> Result<Vec<String>> {
    let range = format!("{}...{}", base, head);
    let output = run_git_command("diff", &["--name-only", &range], opts)?;
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

pub fn get_diff_hunks_between_refs(
    base: &str,
    head: &str,
    file: &str,
    opts: RunOpts,
) -> Result<Vec<HunkRange>> {
    let range = format!("{}...{}", base, head);
    let output = run_git_command("diff", &["-U0", &range, "--", file], opts)?;
    Ok(parse_hunk_headers(&output, DiffSide::New))
}

pub fn get_branch_author(branch: &str, opts: RunOpts) -> Result<String> {
    let ref_name = format!("origin/{}", branch);
    run_git_command("log", &["-1", "--format=%an", &ref_name], opts)
}

pub fn get_remote_branch_commit_count(
    branch: &str,
    main_branch: &str,
    opts: RunOpts,
) -> Result<u32> {
    let range = format!("origin/{}..origin/{}", main_branch, branch);
    let output = run_git_command("rev-list", &["--count", &range], opts)?;
    Ok(output.trim().parse().unwrap_or(0))
}

pub fn get_local_changed_files(opts: RunOpts) -> Result<Vec<String>> {
    let mut files = std::collections::HashSet::new();

    // Unstaged modifications
    let unstaged = run_git_command("diff", &["--name-only"], opts)?;
    for f in unstaged.lines().filter(|l| !l.is_empty()) {
        files.insert(f.to_string());
    }

    // Staged modifications
    let staged = run_git_command("diff", &["--name-only", "--staged"], opts)?;
    for f in staged.lines().filter(|l| !l.is_empty()) {
        files.insert(f.to_string());
    }

    Ok(files.into_iter().collect())
}

pub fn get_local_diff_hunks(file: &str, opts: RunOpts) -> Result<Vec<HunkRange>> {
    let mut hunks = Vec::new();

    // Unstaged changes
    let unstaged = run_git_command("diff", &["-U0", "--", file], opts)?;
    hunks.extend(parse_hunk_headers(&unstaged, DiffSide::New));

    // Staged changes
    let staged = run_git_command("diff", &["-U0", "--staged", "--", file], opts)?;
    hunks.extend(parse_hunk_headers(&staged, DiffSide::New));

    Ok(hunks)
}

/// Check if a commit is an ancestor of the given branch (i.e. the commit exists on that branch).
/// Resolves the commit hash and uses the fully-qualified branch ref to avoid ambiguity
/// (e.g. when a tag has the same name as the branch).
pub fn is_ancestor_of(
    commit_hash: &str,
    branch: &str,
    opts: RunOpts,
) -> Result<bool> {
    // Resolve to a full hash — short SHAs can be unreliable with merge-base
    let full_hash = run_git_command("rev-parse", &["--verify", commit_hash], opts)?;
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
        opts,
    )?;
    Ok(status.code() == Some(0))
}

pub fn get_commit_subject(commit_hash: &str, opts: RunOpts) -> Result<String> {
    run_git_command("log", &["-1", "--format=%s", commit_hash], opts)
}

pub fn commit_exists(commit_hash: &str, opts: RunOpts) -> Result<bool> {
    // Use rev-parse --verify which exits non-zero if the ref doesn't exist.
    // run_git_command respects dry-run (returns Ok("")) so we assume it exists in that mode.
    match run_git_command("rev-parse", &["--verify", commit_hash], opts) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

pub fn resolve_commit_hash(short_sha: &str, opts: RunOpts) -> Result<String> {
    run_git_command("rev-parse", &["--verify", short_sha], opts)
        .with_context(|| format!("Could not resolve commit '{}'", short_sha))
}

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
pub fn check_ci_status(branch: &str, opts: RunOpts) -> CiStatus {
    if opts.dry_run {
        if opts.verbose {
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

    if opts.verbose {
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

    if opts.verbose {
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

/// Creates an immutable stash snapshot without touching the stash reflog.
pub fn stash_create(opts: RunOpts) -> Result<Option<String>> {
    let hash = run_git_command("stash", &["create"], opts)?;
    if hash.is_empty() {
        Ok(None)
    } else {
        Ok(Some(hash))
    }
}

pub fn stash_apply(hash: &str, opts: RunOpts) -> Result<String> {
    run_git_command("stash", &["apply", hash], opts)
}

pub fn is_working_directory_dirty(opts: RunOpts) -> Result<bool> {
    let output = run_git_command("status", &["--porcelain"], opts)?;
    Ok(!output.is_empty())
}

pub fn check_git_operation_in_progress(opts: RunOpts) -> Result<Option<String>> {
    let git_dir = run_git_command("rev-parse", &["--git-dir"], opts)?;
    let git_path = std::path::Path::new(&git_dir);

    if git_path.join("rebase-apply").is_dir() || git_path.join("rebase-merge").is_dir() {
        return Ok(Some("A rebase is already in progress.".to_string()));
    }
    if git_path.join("MERGE_HEAD").exists() {
        return Ok(Some("A merge is already in progress.".to_string()));
    }
    if git_path.join("CHERRY_PICK_HEAD").exists() {
        return Ok(Some("A cherry-pick is already in progress.".to_string()));
    }
    if git_path.join("REBASE_HEAD").exists() {
        return Ok(Some("A rebase is already in progress.".to_string()));
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_is_installed() {
        let result = Command::new("git").arg("--version").output();
        assert!(result.is_ok(), "Git is not installed or not in PATH");
        let output = result.unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("git version"),
            "Unexpected output: {}",
            stdout
        );
    }

    #[test]
    fn test_run_git_command_version() {
        let opts = RunOpts::new(true, false);
        let result = run_git_command("--version", &[], opts);
        assert!(result.is_ok(), "Expected Ok, got {:?}", result);
        let output = result.unwrap();
        assert!(output.contains("git version"), "Output was: {}", output);
    }

    #[test]
    fn test_status() {
        let opts = RunOpts::new(true, false);
        let result = get_status_short(opts);
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

    #[test]
    fn test_ci_status_dry_run_returns_green() {
        let result = check_ci_status("main", RunOpts::new(false, true));
        assert_eq!(result, CiStatus::Green);
    }

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
