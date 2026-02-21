// This file is part of tbdflow, a CLI tool for Trunk-Based Development workflows.
// It provides non-blocking post-commit review functionality.

use crate::config::{Config, ReviewLabelsConfig, ReviewStrategy};
use crate::git;
use anyhow::{Context, Result};
use colored::Colorize;
use glob::Pattern;
use std::process::Command;

/// Returns the first 7 characters of a commit hash for display purposes.
fn short_hash(hash: &str) -> &str {
    &hash[..7.min(hash.len())]
}

/// Checks if any review rules match the files changed in a commit.
/// Returns true if at least one rule pattern matches, meaning a review should be auto-triggered.
pub fn should_auto_trigger_review(
    config: &Config,
    commit_hash: &str,
    verbose: bool,
    dry_run: bool,
) -> Result<bool> {
    if !config.review.enabled || config.review.rules.is_empty() {
        return Ok(false);
    }

    let touched_files = git::get_changed_files(commit_hash, verbose, dry_run)?;

    for rule in &config.review.rules {
        if let Ok(pattern) = Pattern::new(&rule.pattern) {
            if touched_files.iter().any(|f| pattern.matches(f)) {
                if verbose {
                    println!(
                        "{} Auto-trigger: files match rule pattern '{}'",
                        "[REVIEW]".magenta(),
                        rule.pattern
                    );
                }
                return Ok(true);
            }
        }
    }

    Ok(false)
}

/// Triggers a non-blocking review for a commit.
/// This is called automatically after committing to main (if enabled),
/// or manually via `tbdflow review --trigger`.
///
/// # Arguments
/// * `config` - The tbdflow configuration
/// * `reviewers_override` - Optional list of reviewers to use instead of config defaults
/// * `commit_hash` - The full commit hash
/// * `message` - The commit message
/// * `author` - The commit author
/// * `verbose` - Enable verbose output
/// * `dry_run` - Simulate without making changes
pub fn trigger_review(
    config: &Config,
    reviewers_override: Option<&[String]>,
    commit_hash: &str,
    message: &str,
    author: &str,
    verbose: bool,
    dry_run: bool,
) -> Result<()> {
    if !config.review.enabled {
        if verbose {
            println!("{}", "Review system is disabled in config.".dimmed());
        }
        return Ok(());
    }

    // 1. Identify which rules apply based on touched files
    let touched_files = git::get_changed_files(commit_hash, verbose, dry_run)?;
    let mut applicable_reviewers: Vec<String> = Vec::new();
    let mut is_targeted = false;

    for rule in &config.review.rules {
        if let Ok(pattern) = Pattern::new(&rule.pattern) {
            let matched = touched_files.iter().any(|f| pattern.matches(f));
            if matched {
                if verbose {
                    println!(
                        "{} File match for rule: {}",
                        "[RULE]".magenta(),
                        rule.pattern.dimmed()
                    );
                }
                is_targeted = true;
                if let Some(rule_reviewers) = &rule.reviewers {
                    applicable_reviewers.extend(rule_reviewers.clone());
                }
            }
        }
    }

    // 2. Aggregate reviewers
    let mut final_reviewers = if let Some(ovr) = reviewers_override {
        ovr.to_vec()
    } else if !applicable_reviewers.is_empty() {
        applicable_reviewers
    } else {
        config.review.default_reviewers.clone()
    };

    final_reviewers.sort();
    final_reviewers.dedup();

    // 3. Trigger the review
    println!("{}", "--- Triggering Non-blocking Review ---".blue());
    if is_targeted {
        println!("{} Review triggered by targeted file rules.", "🎯".yellow());
    }

    let short = short_hash(commit_hash);
    println!(
        "{} {} ({})",
        "Review requested for:".green(),
        message.bold(),
        short.dimmed()
    );
    println!("   Author: {}", author);
    if !final_reviewers.is_empty() {
        println!("   Reviewers: {}", final_reviewers.join(", "));
    }

    if dry_run {
        println!("{}", "[DRY RUN] Would create review request".yellow());
        return Ok(());
    }

    // Strategy-specific handling using type-safe enum
    match &config.review.strategy {
        ReviewStrategy::GithubIssue => {
            create_github_issue(
                &config.review.labels,
                &final_reviewers,
                commit_hash,
                message,
                author,
                verbose,
            )?;
        }
        ReviewStrategy::GithubWorkflow => {
            trigger_github_workflow(
                config,
                commit_hash,
                message,
                author,
                &final_reviewers,
                verbose,
            )?;
        }
        ReviewStrategy::LogOnly => {
            println!(
                "{}",
                "Review logged (no external system integration)".dimmed()
            );
        }
    }

    Ok(())
}

/// Triggers a GitHub Actions workflow for server-side review management.
/// This enables the "Trunktopus" pattern where Actions handle issue creation,
/// commit status updates, and multi-reviewer orchestration.
fn trigger_github_workflow(
    config: &Config,
    commit_hash: &str,
    message: &str,
    author: &str,
    reviewers: &[String],
    verbose: bool,
) -> Result<()> {
    if !is_gh_cli_available() {
        println!(
            "{}",
            "Warning: GitHub CLI (gh) not found. Install it to trigger workflows.".yellow()
        );
        println!(
            "{}",
            "Install: https://cli.github.com/ or 'brew install gh'".dimmed()
        );
        return Ok(());
    }

    let workflow_name = config
        .review
        .workflow
        .as_deref()
        .unwrap_or("nbr-review.yml");

    let short = short_hash(commit_hash);

    if verbose {
        println!(
            "{} Triggering workflow '{}' for commit {}",
            "[INFO]".cyan(),
            workflow_name,
            short
        );
    }

    // Build workflow inputs as JSON
    let reviewers_json = reviewers.join(",");

    let output = Command::new("gh")
        .args([
            "workflow",
            "run",
            workflow_name,
            "-f",
            &format!("commit_sha={}", commit_hash),
            "-f",
            &format!("commit_message={}", message),
            "-f",
            &format!("author={}", author),
            "-f",
            &format!("reviewers={}", reviewers_json),
        ])
        .output()
        .context("Failed to trigger GitHub workflow")?;

    if output.status.success() {
        println!(
            "{}",
            format!(
                "Workflow '{}' triggered for commit {}",
                workflow_name, short
            )
            .green()
        );
        println!(
            "{}",
            "   Server-side review management is now active.".dimmed()
        );
        println!(
            "{}",
            "   Check GitHub Actions for issue creation and status updates.".dimmed()
        );
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("could not find any workflows") {
            println!(
                "{}",
                format!(
                    "Warning: Workflow '{}' not found in repository.",
                    workflow_name
                )
                .yellow()
            );
            println!(
                "{}",
                "   Create the workflow file at .github/workflows/ to enable server-side reviews."
                    .dimmed()
            );
            println!(
                "{}",
                "   Falling back to client-side issue creation...".dimmed()
            );
            // Fallback to client-side issue creation
            create_github_issue(
                &config.review.labels,
                reviewers,
                commit_hash,
                message,
                author,
                verbose,
            )?;
        } else {
            println!(
                "{}",
                format!("Warning: Failed to trigger workflow: {}", stderr.trim()).yellow()
            );
        }
    }

    Ok(())
}

/// Creates a GitHub issue for post-commit review using the `gh` CLI.
fn create_github_issue(
    labels: &ReviewLabelsConfig,
    reviewers: &[String],
    commit_hash: &str,
    message: &str,
    author: &str,
    verbose: bool,
) -> Result<()> {
    let short = short_hash(commit_hash);

    // Check if gh CLI is available
    if !is_gh_cli_available() {
        println!(
            "{}",
            "Warning: GitHub CLI (gh) not found. Install it to enable GitHub issue creation."
                .yellow()
        );
        println!(
            "{}",
            "Install: https://cli.github.com/ or 'brew install gh'".dimmed()
        );
        return Ok(());
    }

    // Ensure all review labels exist (create if missing)
    ensure_review_labels_exist(labels, verbose);

    // Get the repository URL for commit links
    let repo_url = git::get_remote_url(verbose, false).unwrap_or_default();
    let commit_url = if repo_url.is_empty() {
        format!("`{}`", commit_hash)
    } else {
        format!("[`{}`]({}/commit/{})", short, repo_url, commit_hash)
    };

    let title = format!("[Review] {} ({})", message, short);
    let body = format!(
        "## Non-blocking Review Request\n\n\
        **Commit:** {}\n\
        **Author:** {}\n\
        **Message:** {}\n\n\
        ---\n\n\
        > In Trunk-Based Development, this code is already in the trunk.\n\
        > Your goal is **Course Correction** and **Knowledge Sharing**, not gatekeeping.\n\n\
        ### What to Look For\n\n\
        | Focus | Question |\n\
        |-------|----------|\n\
        | **Design & Intent** | Does the implementation align with our architectural patterns? |\n\
        | **Logic & Edge Cases** | Are there logical flaws or unhappy paths that tests might miss? |\n\
        | **Readability** | Are names descriptive? (Code as Documentation) |\n\
        | **Simplification** | Can this be done with less code or lower complexity? |\n\n\
        ### How to Comment\n\n\
        - **Questions > Commands**: _\"Could we use the existing helper here?\"_ instead of _\"Change this.\"_\n\
        - **Praise**: If you see something clever or clean, say so! NBR boosts team morale.\n\
        - **Nitpicking**: Label minor style issues as `(nit)` so the author knows they're optional.\n\n\
        ### Concerns\n\n\
        _No concerns raised yet._\n\n\
        ---\n\n\
        To approve via CLI:\n\
        ```\n\
        tbdflow review --approve {}\n\
        ```\n\n\
        To raise a concern:\n\
        ```\n\
        tbdflow review --concern {} -m \"Your concern here\"\n\
        ```",
        commit_url, author, message, short, short
    );

    let mut args = vec!["issue", "create", "--title", &title, "--body", &body];

    // Add the pending label
    if label_exists(&labels.pending) {
        args.push("--label");
        args.push(&labels.pending);
    }

    // Add assignees if configured
    let assignees: Vec<&str> = reviewers.iter().map(String::as_str).collect();
    let assignees_str = assignees.join(",");
    if !assignees.is_empty() {
        args.push("--assignee");
        args.push(&assignees_str);
    }

    if verbose {
        println!("{} gh {}", "[RUNNING]".cyan(), args.join(" "));
    }

    let output = Command::new("gh")
        .args(&args)
        .output()
        .context("Failed to execute 'gh' CLI")?;

    if output.status.success() {
        let issue_url = String::from_utf8_lossy(&output.stdout).trim().to_string();
        println!("{} {}", "Review issue created:".green(), issue_url);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!(
            "{}",
            format!("Warning: Failed to create GitHub issue: {}", stderr).yellow()
        );
    }

    Ok(())
}

/// Checks if a specific label exists in the repository.
fn label_exists(label_name: &str) -> bool {
    Command::new("gh")
        .args(["label", "list", "--search", label_name, "--json", "name"])
        .output()
        .map(|o| {
            o.status.success()
                && String::from_utf8_lossy(&o.stdout)
                    .contains(&format!("\"name\":\"{}\"", label_name))
        })
        .unwrap_or(false)
}

/// Ensures a label exists, creating it if necessary.
fn ensure_label_exists(label_name: &str, description: &str, color: &str, verbose: bool) {
    if label_exists(label_name) {
        return;
    }

    if verbose {
        println!("{} Creating '{}' label...", "[INFO]".cyan(), label_name);
    }

    let result = Command::new("gh")
        .args([
            "label",
            "create",
            label_name,
            "--description",
            description,
            "--color",
            color,
        ])
        .output();

    match result {
        Ok(output) if output.status.success() => {
            if verbose {
                println!("{} Created '{}' label", "[INFO]".cyan(), label_name);
            }
        }
        _ => {
            // Silently continue - label creation may fail due to permissions
            // The issue will still be created, just without the label
        }
    }
}

/// Ensures all review labels exist (pending, concern, accepted, dismissed).
fn ensure_review_labels_exist(labels: &ReviewLabelsConfig, verbose: bool) {
    ensure_label_exists(
        &labels.pending,
        "Review pending - awaiting attention",
        "FBCA04", // Yellow
        verbose,
    );
    ensure_label_exists(
        &labels.concern,
        "Review concern raised - needs attention",
        "D93F0B", // Red-orange
        verbose,
    );
    ensure_label_exists(
        &labels.accepted,
        "Review accepted/approved",
        "0E8A16", // Green
        verbose,
    );
    ensure_label_exists(
        &labels.dismissed,
        "Review dismissed - won't fix",
        "6A737D", // Gray
        verbose,
    );
}

/// Checks if the GitHub CLI is available.
fn is_gh_cli_available() -> bool {
    Command::new("gh")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Handles the `review --trigger` command for the current HEAD commit.
pub fn handle_review_trigger(
    config: &Config,
    reviewers_override: Option<Vec<String>>,
    verbose: bool,
    dry_run: bool,
) -> Result<()> {
    if !config.review.enabled {
        println!(
            "{}",
            "Review system is not enabled. Add the following to your .tbdflow.yml:".yellow()
        );
        println!("\n  review:");
        println!("    enabled: true");
        println!("    strategy: github-issue");
        println!("    default_reviewers:");
        println!("      - teammate-username\n");
        return Ok(());
    }

    let commit_hash = git::get_head_commit_hash(verbose, dry_run)?;
    let message = git::get_commit_message(&commit_hash, verbose, dry_run)?;
    let author = git::get_user_name(verbose, dry_run)?;

    trigger_review(
        config,
        reviewers_override.as_deref(),
        &commit_hash,
        &message,
        &author,
        verbose,
        dry_run,
    )
}

/// Generates a digest of commits since a given time for review.
pub fn handle_review_digest(
    config: &Config,
    since: &str,
    verbose: bool,
    dry_run: bool,
) -> Result<()> {
    println!(
        "{}",
        format!("--- Trunk Evolution Digest (Since {}) ---", since).blue()
    );

    let log = git::get_log_since(since, verbose, dry_run)?;

    if log.is_empty() {
        println!(
            "{}",
            "No new commits found in the specified time range.".yellow()
        );
        return Ok(());
    }

    println!("\n{}", "COMMITS FOR REVIEW".cyan().bold());
    println!("{}", "─".repeat(50).cyan());

    for line in log.lines() {
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.splitn(3, '|').collect();
        if parts.len() >= 2 {
            let hash = short_hash(parts[0]);
            let author = parts.get(1).unwrap_or(&"unknown");
            let message = parts.get(2).unwrap_or(&"");
            println!(
                "  {} {} {}",
                hash.yellow(),
                format!("({})", author).dimmed(),
                message
            );
        }
    }

    println!("{}", "─".repeat(50).cyan());

    if !config.review.default_reviewers.is_empty() {
        println!(
            "\n{}",
            format!(
                "Default reviewers: {}",
                config.review.default_reviewers.join(", ")
            )
            .dimmed()
        );
    }

    println!("\n{}", "Next steps:".bold());
    println!("   • Review commits above and discuss with the team");
    println!("   • Run 'tbdflow review --approve <hash>' to mark as reviewed");
    println!("   • Run 'tbdflow review --trigger' to create review issues\n");

    Ok(())
}

/// Marks a commit as approved (closes the associated review issue if using GitHub).
pub fn handle_review_approve(
    config: &Config,
    commit_hash: &str,
    verbose: bool,
    dry_run: bool,
) -> Result<()> {
    let short = short_hash(commit_hash);

    println!("{}", format!("--- Approving Commit {} ---", short).blue());

    if dry_run {
        println!("{}", "[DRY RUN] Would mark commit as approved".yellow());
        return Ok(());
    }

    match &config.review.strategy {
        ReviewStrategy::GithubIssue => {
            close_github_review_issue(&config.review.labels, short, verbose)?;
        }
        ReviewStrategy::GithubWorkflow => {
            // For workflow strategy, close the issue which will trigger
            // the server-side Action to update commit status
            close_github_review_issue(&config.review.labels, short, verbose)?;
            println!(
                "{}",
                "   Server-side workflow will update commit status.".dimmed()
            );
        }
        ReviewStrategy::LogOnly => {
            println!("{}", format!("Commit {} marked as approved", short).green());
        }
    }

    Ok(())
}

/// Raises a concern on a commit review (keeps issue open, adds concern label, notifies author).
pub fn handle_review_concern(
    config: &Config,
    commit_hash: &str,
    message: &str,
    verbose: bool,
    dry_run: bool,
) -> Result<()> {
    let short = short_hash(commit_hash);

    println!(
        "{}",
        format!("--- Raising Concern on Commit {} ---", short).blue()
    );

    if dry_run {
        println!("{}", "[DRY RUN] Would raise concern on commit".yellow());
        return Ok(());
    }

    match &config.review.strategy {
        ReviewStrategy::GithubIssue | ReviewStrategy::GithubWorkflow => {
            raise_github_concern(config, commit_hash, message, verbose)?;
        }
        ReviewStrategy::LogOnly => {
            println!("{}", format!("CONCERN on {}: {}", short, message).yellow());
        }
    }

    Ok(())
}

/// Dismisses a review (closes issue with dismissed label).
pub fn handle_review_dismiss(
    config: &Config,
    commit_hash: &str,
    message: &str,
    verbose: bool,
    dry_run: bool,
) -> Result<()> {
    let short = short_hash(commit_hash);

    println!(
        "{}",
        format!("--- Dismissing Review for Commit {} ---", short).blue()
    );

    if dry_run {
        println!("{}", "[DRY RUN] Would dismiss review".yellow());
        return Ok(());
    }

    match &config.review.strategy {
        ReviewStrategy::GithubIssue | ReviewStrategy::GithubWorkflow => {
            dismiss_github_review_issue(&config.review.labels, short, message, verbose)?;
        }
        ReviewStrategy::LogOnly => {
            println!(
                "{}",
                format!("Review for {} dismissed: {}", short, message).dimmed()
            );
        }
    }

    Ok(())
}

/// Raises a concern on a GitHub review issue.
fn raise_github_concern(
    config: &Config,
    commit_hash: &str,
    message: &str,
    verbose: bool,
) -> Result<()> {
    let short = short_hash(commit_hash);
    let labels = &config.review.labels;

    if !is_gh_cli_available() {
        println!(
            "{}",
            "Warning: GitHub CLI (gh) not found. Cannot raise concern.".yellow()
        );
        return Ok(());
    }

    // Search for the review issue
    let search_query = format!("[Review] in:title {} in:title is:open", short);

    if verbose {
        println!("{} Searching for review issue...", "[INFO]".cyan());
    }

    let output = Command::new("gh")
        .args([
            "issue",
            "list",
            "--search",
            &search_query,
            "--json",
            "number,body",
            "--limit",
            "1",
        ])
        .output()
        .context("Failed to search for GitHub issues")?;

    if !output.status.success() {
        println!(
            "{}",
            format!("Warning: Could not find review issue for {}", short).yellow()
        );
        return Ok(());
    }

    let json_output = String::from_utf8_lossy(&output.stdout);

    if let Some(issue_num) = extract_issue_number(&json_output) {
        let issue_num_str = issue_num.to_string();

        // Update labels: remove pending, add concern
        if verbose {
            println!(
                "{} Updating labels on issue #{}",
                "[INFO]".cyan(),
                issue_num
            );
        }

        let _ = Command::new("gh")
            .args([
                "issue",
                "edit",
                &issue_num_str,
                "--remove-label",
                &labels.pending,
            ])
            .output();

        let _ = Command::new("gh")
            .args([
                "issue",
                "edit",
                &issue_num_str,
                "--add-label",
                &labels.concern,
            ])
            .output();

        // Add a comment with the concern
        let comment = format!("**Concern Raised**\n\n{}", message);

        let _ = Command::new("gh")
            .args(["issue", "comment", &issue_num_str, "--body", &comment])
            .output();

        // Append checklist item to the issue body
        append_concern_checklist_item(&issue_num_str, message, verbose)?;

        // Set commit status based on config
        set_commit_status(config, commit_hash, message, verbose)?;

        println!(
            "{}",
            format!(
                "Concern raised on issue #{} for commit {} (label: {})",
                issue_num, short, labels.concern
            )
            .yellow()
        );
    } else {
        println!(
            "{}",
            format!("Warning: No open review issue found for commit {}", short).yellow()
        );
        println!("   Run 'tbdflow review --trigger' first to create the review issue.");
    }

    Ok(())
}

/// Appends a concern as a checklist item to the issue body.
fn append_concern_checklist_item(
    issue_num: &str,
    concern_message: &str,
    verbose: bool,
) -> Result<()> {
    // Get current issue body
    let output = Command::new("gh")
        .args(["issue", "view", issue_num, "--json", "body"])
        .output()
        .context("Failed to get issue body")?;

    if !output.status.success() {
        return Ok(());
    }

    let json_output = String::from_utf8_lossy(&output.stdout);

    // Extract the body content
    let current_body = extract_body_from_json(&json_output).unwrap_or_default();

    // Replace the "No concerns raised yet" placeholder or append to concerns section
    let new_body = if current_body.contains("_No concerns raised yet._") {
        current_body.replace(
            "_No concerns raised yet._",
            &format!("- [ ] {}", concern_message),
        )
    } else if current_body.contains("### Concerns") {
        // Find the concerns section and append the new item
        let concerns_marker = "### Concerns\n\n";
        if let Some(pos) = current_body.find(concerns_marker) {
            let insert_pos = pos + concerns_marker.len();
            let (before, after) = current_body.split_at(insert_pos);
            format!("{}- [ ] {}\n{}", before, concern_message, after)
        } else {
            current_body
        }
    } else {
        current_body
    };

    if verbose {
        println!(
            "{} Updating issue body with concern checklist item",
            "[INFO]".cyan()
        );
    }

    let _ = Command::new("gh")
        .args(["issue", "edit", issue_num, "--body", &new_body])
        .output();

    Ok(())
}

/// Extracts body content from GitHub CLI JSON output.
fn extract_body_from_json(json: &str) -> Option<String> {
    // Looking for "body":"..." pattern
    if let Some(start) = json.find("\"body\":\"") {
        let rest = &json[start + 8..];
        // Find the closing quote, handling escaped quotes
        let mut end = 0;
        let mut escaped = false;
        for (i, c) in rest.chars().enumerate() {
            if escaped {
                escaped = false;
                continue;
            }
            if c == '\\' {
                escaped = true;
                continue;
            }
            if c == '"' {
                end = i;
                break;
            }
        }
        let body = &rest[..end];
        // Unescape the string
        Some(
            body.replace("\\n", "\n")
                .replace("\\\"", "\"")
                .replace("\\\\", "\\"),
        )
    } else {
        None
    }
}

/// Sets commit status based on concern_blocks_status config.
fn set_commit_status(
    config: &Config,
    commit_hash: &str,
    message: &str,
    verbose: bool,
) -> Result<()> {
    if !is_gh_cli_available() {
        return Ok(());
    }

    let (state, description) = if config.review.concern_blocks_status {
        ("failure", format!("Audit Concern: {}", message))
    } else {
        (
            "pending",
            format!("Awaiting fix-forward for concern: {}", message),
        )
    };

    // Get repo owner/name
    let repo_info = Command::new("gh")
        .args(["repo", "view", "--json", "owner,name"])
        .output();

    let repo = match repo_info {
        Ok(output) if output.status.success() => {
            let json = String::from_utf8_lossy(&output.stdout);
            extract_repo_from_json(&json)
        }
        _ => return Ok(()),
    };

    let Some((owner, name)) = repo else {
        return Ok(());
    };

    if verbose {
        println!(
            "{} Setting commit status to '{}' for {}",
            "[INFO]".cyan(),
            state,
            short_hash(commit_hash)
        );
    }

    let api_path = format!("repos/{}/{}/statuses/{}", owner, name, commit_hash);

    let _ = Command::new("gh")
        .args([
            "api",
            &api_path,
            "-f",
            &format!("state={}", state),
            "-f",
            "context=peer-review",
            "-f",
            &format!("description={}", description),
        ])
        .output();

    Ok(())
}

/// Extracts owner and name from GitHub CLI repo JSON output.
fn extract_repo_from_json(json: &str) -> Option<(String, String)> {
    // Simple extraction for {"owner":{"login":"..."},"name":"..."}
    let owner_start = json.find("\"login\":\"")?;
    let owner_rest = &json[owner_start + 9..];
    let owner_end = owner_rest.find('"')?;
    let owner = owner_rest[..owner_end].to_string();

    let name_start = json.find("\"name\":\"")?;
    let name_rest = &json[name_start + 8..];
    let name_end = name_rest.find('"')?;
    let name = name_rest[..name_end].to_string();

    Some((owner, name))
}

/// Dismisses a GitHub review issue (closes with dismissed label).
fn dismiss_github_review_issue(
    labels: &ReviewLabelsConfig,
    short_hash: &str,
    message: &str,
    verbose: bool,
) -> Result<()> {
    if !is_gh_cli_available() {
        println!(
            "{}",
            "Warning: GitHub CLI (gh) not found. Cannot dismiss review.".yellow()
        );
        return Ok(());
    }

    // Search for the review issue
    let search_query = format!("[Review] in:title {} in:title is:open", short_hash);

    if verbose {
        println!("{} Searching for review issue...", "[INFO]".cyan());
    }

    let output = Command::new("gh")
        .args([
            "issue",
            "list",
            "--search",
            &search_query,
            "--json",
            "number",
            "--limit",
            "1",
        ])
        .output()
        .context("Failed to search for GitHub issues")?;

    if output.status.success() {
        let json_output = String::from_utf8_lossy(&output.stdout);

        if let Some(issue_num) = extract_issue_number(&json_output) {
            let issue_num_str = issue_num.to_string();

            // Update labels: remove pending/concern, add dismissed
            if verbose {
                println!(
                    "{} Updating labels on issue #{}",
                    "[INFO]".cyan(),
                    issue_num
                );
            }

            let _ = Command::new("gh")
                .args([
                    "issue",
                    "edit",
                    &issue_num_str,
                    "--remove-label",
                    &labels.pending,
                ])
                .output();

            let _ = Command::new("gh")
                .args([
                    "issue",
                    "edit",
                    &issue_num_str,
                    "--remove-label",
                    &labels.concern,
                ])
                .output();

            let _ = Command::new("gh")
                .args([
                    "issue",
                    "edit",
                    &issue_num_str,
                    "--add-label",
                    &labels.dismissed,
                ])
                .output();

            // Close with a comment
            let comment = format!(
                "**Dismissed** via `tbdflow review --dismiss`\n\nReason: {}",
                message
            );

            let close_output = Command::new("gh")
                .args(["issue", "close", &issue_num_str, "--comment", &comment])
                .output()
                .context("Failed to close GitHub issue")?;

            if close_output.status.success() {
                println!(
                    "{}",
                    format!(
                        "Review for commit {} dismissed and issue #{} closed (label: {})",
                        short_hash, issue_num, labels.dismissed
                    )
                    .dimmed()
                );
            } else {
                println!(
                    "{}",
                    format!("Review dismissed (issue close failed)").yellow()
                );
            }
        } else {
            println!(
                "{}",
                format!(
                    "Review for {} dismissed (no open review issue found)",
                    short_hash
                )
                .dimmed()
            );
        }
    } else {
        println!(
            "{}",
            format!("Review for {} dismissed", short_hash).dimmed()
        );
    }

    Ok(())
}

/// Closes a GitHub issue associated with a commit review, adding the accepted label.
fn close_github_review_issue(
    labels: &ReviewLabelsConfig,
    short_hash: &str,
    verbose: bool,
) -> Result<()> {
    if !is_gh_cli_available() {
        println!(
            "{}",
            "Warning: GitHub CLI (gh) not found. Marking as approved locally only.".yellow()
        );
        println!("{}", format!("Commit {} approved", short_hash).green());
        return Ok(());
    }

    // Search for the review issue
    let search_query = format!("[Review] in:title {} in:title is:open", short_hash);

    if verbose {
        println!("{} Searching for review issue...", "[INFO]".cyan());
    }

    let output = Command::new("gh")
        .args([
            "issue",
            "list",
            "--search",
            &search_query,
            "--json",
            "number",
            "--limit",
            "1",
        ])
        .output()
        .context("Failed to search for GitHub issues")?;

    if output.status.success() {
        let json_output = String::from_utf8_lossy(&output.stdout);

        // Simple JSON parsing for issue number
        if let Some(issue_num) = extract_issue_number(&json_output) {
            let issue_num_str = issue_num.to_string();

            // Remove pending/concern labels and add accepted label
            if verbose {
                println!(
                    "{} Updating labels on issue #{}",
                    "[INFO]".cyan(),
                    issue_num
                );
            }

            let _ = Command::new("gh")
                .args([
                    "issue",
                    "edit",
                    &issue_num_str,
                    "--remove-label",
                    &labels.pending,
                ])
                .output();

            let _ = Command::new("gh")
                .args([
                    "issue",
                    "edit",
                    &issue_num_str,
                    "--remove-label",
                    &labels.concern,
                ])
                .output();

            let _ = Command::new("gh")
                .args([
                    "issue",
                    "edit",
                    &issue_num_str,
                    "--add-label",
                    &labels.accepted,
                ])
                .output();

            if verbose {
                println!("{} Closing issue #{}", "[INFO]".cyan(), issue_num);
            }

            let close_output = Command::new("gh")
                .args([
                    "issue",
                    "close",
                    &issue_num_str,
                    "--comment",
                    "Approved via `tbdflow review --approve`",
                ])
                .output()
                .context("Failed to close GitHub issue")?;

            if close_output.status.success() {
                println!(
                    "{}",
                    format!(
                        "Commit {} approved and review issue #{} closed (label: {})",
                        short_hash, issue_num, labels.accepted
                    )
                    .green()
                );
            } else {
                println!(
                    "{}",
                    format!("Commit {} approved (issue close failed)", short_hash).yellow()
                );
            }
        } else {
            println!(
                "{}",
                format!(
                    "Commit {} approved (no open review issue found)",
                    short_hash
                )
                .green()
            );
        }
    } else {
        println!("{}", format!("Commit {} approved", short_hash).green());
    }

    Ok(())
}

/// Extracts issue number from GitHub CLI JSON output.
fn extract_issue_number(json: &str) -> Option<i64> {
    // Simple extraction without full JSON parsing
    // Looking for pattern like: [{"number":123}]
    if json.contains("\"number\":") {
        let start = json.find("\"number\":")?;
        let rest = &json[start + 9..];
        let end = rest.find(|c: char| !c.is_ascii_digit())?;
        rest[..end].parse().ok()
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_hash_returns_first_seven_chars() {
        assert_eq!(short_hash("abc1234567890"), "abc1234");
    }

    #[test]
    fn short_hash_handles_exact_seven_chars() {
        assert_eq!(short_hash("abc1234"), "abc1234");
    }

    #[test]
    fn short_hash_handles_short_input() {
        assert_eq!(short_hash("abc"), "abc");
    }

    #[test]
    fn short_hash_handles_empty_input() {
        assert_eq!(short_hash(""), "");
    }

    #[test]
    fn extract_issue_number_parses_valid_json() {
        let json = r#"[{"number":123}]"#;
        assert_eq!(extract_issue_number(json), Some(123));
    }

    #[test]
    fn extract_issue_number_parses_larger_number() {
        let json = r#"[{"number":98765}]"#;
        assert_eq!(extract_issue_number(json), Some(98765));
    }

    #[test]
    fn extract_issue_number_returns_none_for_empty_array() {
        let json = r#"[]"#;
        assert_eq!(extract_issue_number(json), None);
    }

    #[test]
    fn extract_issue_number_returns_none_for_invalid_json() {
        let json = r#"not json"#;
        assert_eq!(extract_issue_number(json), None);
    }

    #[test]
    fn extract_issue_number_handles_whitespace() {
        let json = r#"[{"number": 42}]"#;
        // Note: current impl doesn't handle space after colon
        // This documents the limitation
        assert_eq!(extract_issue_number(json), None);
    }
}
