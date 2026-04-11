use crate::config::Config;
use crate::git::GitError;
use crate::{commands, config, git, intent};
use anyhow::Result;
use colored::Colorize;
use std::path::PathBuf;

pub fn get_default_branch_name(config: &Config) -> &str {
    config.main_branch_name.as_str()
}

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

    let main_branch_name = get_default_branch_name(config);
    let prefix = commands::get_branch_prefix_or_error(&config.branch_types, &r#type.unwrap())?;

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

pub fn handle_complete(
    r#type: String,
    name: String,
    config: &Config,
    dry_run: bool,
    verbose: bool,
) -> Result<()> {
    println!(
        "{}",
        "--- Completing short-lived branch ---".to_string().blue()
    );

    let main_branch_name = get_default_branch_name(config);

    if name == main_branch_name {
        return Err(GitError::CannotCompleteMainBranch.into());
    }

    let branch_name = git::find_branch(&name, &r#type, config, verbose, dry_run)?;
    println!("{}", format!("Branch to complete: {}", branch_name).blue());

    git::branch_exists_locally(&branch_name, verbose, dry_run)?;

    if r#type == "release" {
        let tag_name = format!("{}{}", config.automatic_tags.release_prefix, name);

        if git::tag_exists(&tag_name, verbose, dry_run)? {
            return Err(GitError::TagAlreadyExists(tag_name).into());
        }
    }

    git::is_working_directory_clean(verbose, dry_run)?;
    git::checkout_main(verbose, dry_run, main_branch_name)?;
    git::pull_latest_with_rebase(verbose, dry_run)?;
    git::merge_branch(&branch_name, verbose, dry_run)?;

    if r#type == "release" {
        let tag_name = format!("{}{}", config.automatic_tags.release_prefix, name);
        let merge_commit_hash = git::get_head_commit_hash(verbose, dry_run)?;
        git::create_tag(
            &tag_name,
            &format!("Release {}", name),
            &merge_commit_hash,
            verbose,
            dry_run,
        )?;
        println!(
            "{}",
            format!("Created tag '{}' on merge commit.", tag_name).green()
        );
    }

    git::push(verbose, dry_run)?;
    if r#type == "release" {
        git::push_tags(verbose, dry_run)?;
    }

    git::delete_local_branch(&branch_name, verbose, dry_run)?;
    git::delete_remote_branch(&branch_name, verbose, dry_run)?;

    // Cleanup the intent log after merging back to trunk
    let git_root = PathBuf::from(git::get_git_root(verbose, dry_run)?);
    if intent::load_intent_log(&git_root)?.is_some() {
        intent::cleanup_intent_log(&git_root)?;
        println!("{}", "Intent log cleared after branch completion.".dimmed());
    }

    println!(
        "\n{}",
        format!(
            "Success! Branch '{}' was merged into main and deleted.",
            branch_name
        )
        .green()
    );
    Ok(())
}
