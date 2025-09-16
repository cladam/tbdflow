use crate::config::{Config, DodConfig};
use crate::{config, git, misc};
use anyhow::Result;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect};
use std::path::PathBuf;

pub fn handle_branch(
    r#type: Option<String>,
    config: &Config,
    name: Option<String>,
    issue: Option<String>,
    from_commit: Option<String>,
    dry_run: bool,
    verbose: bool,
) -> Result<()> {
    println!(
        "{}",
        "--- Creating short-lived branch ---".to_string().blue()
    );

    // Lookup the default branch name.
    let main_branch_name = config.main_branch_name.as_str();
    let prefix = misc::get_branch_prefix_or_error(&config.branch_types, &r#type.unwrap())?;

    // Construct the branch name based on the configured strategy
    let branch_name = match config.issue_handling.strategy {
        config::IssueHandlingStrategy::BranchName => {
            let issue_part = issue.map_or("".to_string(), |i| format!("{}-", i));
            format!("{}{}{}", prefix, issue_part, name.unwrap())
        }
        config::IssueHandlingStrategy::CommitScope => {
            format!("{}{}", prefix, name.unwrap())
        }
    };

    git::is_working_directory_clean(verbose, dry_run)?;
    git::checkout_main(verbose, dry_run, main_branch_name)?;
    git::pull_latest_with_rebase(verbose, dry_run)?;
    git::create_branch(&branch_name, from_commit.as_deref(), verbose, dry_run)?;
    git::push_set_upstream(&branch_name, verbose, dry_run)?;
    println!(
        "\n{}",
        format!("Success! Switched to new branch: '{}'", branch_name).green()
    );
    Ok(())
}
