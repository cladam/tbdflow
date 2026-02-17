// This file is part of tbdflow, a CLI tool for Trunk-Based Development workflows.
// It provides non-blocking post-commit review functionality.

use crate::config::{Config, ReviewStrategy};
use crate::git;
use anyhow::{Context, Result};
use colored::Colorize;
use std::process::Command;

/// Returns the first 7 characters of a commit hash for display purposes.
fn short_hash(hash: &str) -> &str {
    &hash[..7.min(hash.len())]
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

    println!("{}", "--- Triggering Non-blocking Review ---".blue());

    let short = short_hash(commit_hash);
    let reviewers = reviewers_override.unwrap_or(&config.review.default_reviewers);

    println!(
        "{} {} ({})",
        "Review requested for:".green(),
        message.bold(),
        short.dimmed()
    );
    println!("   Author: {}", author);
    if !reviewers.is_empty() {
        println!("   Reviewers: {}", reviewers.join(", "));
    }

    if dry_run {
        println!("{}", "[DRY RUN] Would create review request".yellow());
        return Ok(());
    }

    // Strategy-specific handling using type-safe enum
    match &config.review.strategy {
        ReviewStrategy::GithubIssue => {
            create_github_issue(reviewers, commit_hash, message, author, verbose)?;
        }
        ReviewStrategy::GithubWorkflow => {
            trigger_github_workflow(config, commit_hash, message, author, reviewers, verbose)?;
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
            create_github_issue(reviewers, commit_hash, message, author, verbose)?;
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

    // Ensure the 'review' label exists (create if missing)
    ensure_review_label_exists(verbose);

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
        ### When to Close\n\n\
        - **Approve & Close**: Code is safe and understandable.\n\
        - **Comment & Close**: Non-critical improvements noted.\n\
        - **Keep Open**: Only for critical bugs or security flaws requiring immediate fix-forward.\n\n\
        ---\n\n\
        To approve via CLI:\n\
        ```\n\
        tbdflow review --approve {}\n\
        ```",
        commit_url, author, message, short
    );

    let mut args = vec!["issue", "create", "--title", &title, "--body", &body];

    // Only add label if it exists
    if review_label_exists() {
        args.push("--label");
        args.push("review");
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

/// Checks if the 'review' label exists in the repository.
fn review_label_exists() -> bool {
    Command::new("gh")
        .args(["label", "list", "--search", "review", "--json", "name"])
        .output()
        .map(|o| {
            o.status.success() && String::from_utf8_lossy(&o.stdout).contains("\"name\":\"review\"")
        })
        .unwrap_or(false)
}

/// Ensures the 'review' label exists, creating it if necessary.
fn ensure_review_label_exists(verbose: bool) {
    if review_label_exists() {
        return;
    }

    if verbose {
        println!("{} Creating 'review' label...", "[INFO]".cyan());
    }

    let result = Command::new("gh")
        .args([
            "label",
            "create",
            "review",
            "--description",
            "Post-commit review request from tbdflow",
            "--color",
            "0E8A16",
        ])
        .output();

    match result {
        Ok(output) if output.status.success() => {
            if verbose {
                println!("{} Created 'review' label", "[INFO]".cyan());
            }
        }
        _ => {
            // Silently continue - label creation may fail due to permissions
            // The issue will still be created, just without the label
        }
    }
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
            close_github_review_issue(short, verbose)?;
        }
        ReviewStrategy::GithubWorkflow => {
            // For workflow strategy, close the issue which will trigger
            // the server-side Action to update commit status
            close_github_review_issue(short, verbose)?;
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

/// Closes a GitHub issue associated with a commit review.
fn close_github_review_issue(short_hash: &str, verbose: bool) -> Result<()> {
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
            if verbose {
                println!("{} Closing issue #{}", "[INFO]".cyan(), issue_num);
            }

            let close_output = Command::new("gh")
                .args([
                    "issue",
                    "close",
                    &issue_num.to_string(),
                    "--comment",
                    "Approved via `tbdflow review --approve`",
                ])
                .output()
                .context("Failed to close GitHub issue")?;

            if close_output.status.success() {
                println!(
                    "{}",
                    format!(
                        "Commit {} approved and review issue #{} closed",
                        short_hash, issue_num
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
