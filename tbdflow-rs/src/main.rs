// ===============================================================
// FILE: src/main.rs
// ===============================================================
// Project: tbdflow - Trunk-Based Development Git CLI
// Description: Entry point for the CLI application.
// Author: Claes Adamsson @cladam
// ===============================================================

use clap::Parser;
use colored::Colorize;
use tbdflow::{cli, git};
use tbdflow::cli::Commands;
use tbdflow::git::GitError;

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();

    match cli.command {
        Commands::Feature { name } => {
            println!("--- Creating feature branch ---");
            let branch_name = format!("feature/{}", name);
            git::is_working_directory_clean()?;
            git::checkout_main()?;
            git::pull_latest_with_rebase()?;
            git::create_branch(&branch_name, None)?;
            println!("\n{}", format!("Success! Switched to new feature branch: '{}'", branch_name).green());
        }
        Commands::Release { version, from_commit } => {
            println!("--- Creating release branch ---");
            let branch_name = format!("release/{}", version);
            git::is_working_directory_clean()?;
            git::checkout_main()?;
            git::pull_latest_with_rebase()?;
            git::create_branch(&branch_name, from_commit.as_deref())?;
            println!("\n{}", format!("Success! Switched to new release branch: '{}'", branch_name).green());
        }
        Commands::Hotfix { name } => {
            println!("--- Creating hotfix branch ---");
            let branch_name = format!("hotfix/{}", name);
            git::is_working_directory_clean()?;
            git::checkout_main()?;
            git::pull_latest_with_rebase()?;
            git::create_branch(&branch_name, None)?;
            println!("\n{}", format!("Success! Switched to new hotfix branch: '{}'", branch_name).green());
        }
        Commands::Commit { r#type, scope, message, breaking } => {
            println!("--- Committing directly to main branch ---");
            let scope_part = scope.map_or("".to_string(), |s| format!("({})", s));
            let breaking_part = if breaking { "!" } else { "" };
            let header = format!("{}{}{}: {}", r#type, scope_part, breaking_part, message);
            let footer = if breaking { format!("\n\nBREAKING CHANGE: {}", message) } else { "".to_string() };
            let commit_message = format!("{}{}", header, footer);

            println!("{}", format!("Commit message will be:\n---\n{}\n---", commit_message).blue());

            git::add_all()?;
            git::commit(&commit_message)?;
            git::push()?;
            println!("\n{}", "Successfully committed and pushed changes to main.".green());
        }
        Commands::Complete { r#type, name } => {
            println!("--- Completing short-lived branch ---");
            let branch_name = match r#type.as_str() {
                "feature" | "hotfix" | "release" => format!("{}/{}", r#type, name),
                _ => return Err(GitError::InvalidBranchType(r#type).into()),
            };
            println!("{}", format!("Branch to complete: {}", branch_name).blue());

            git::is_working_directory_clean()?;
            git::checkout_main()?;
            git::pull_latest_with_rebase()?;
            git::merge_branch(&branch_name)?;
            git::push()?;
            git::delete_local_branch(&branch_name)?;
            git::delete_remote_branch(&branch_name)?;
            println!("\n{}", format!("Success! Branch '{}' was merged into main and deleted.", branch_name).green());
        }
        Commands::Status => {
            println!("--- Git Status ---");
            let output = git::status()?;
            println!("{}", output.blue());
        }
        Commands::CurrentBranch => {
            let branch_name = git::get_current_branch()?;
            println!("{}", format!("Current branch is: {}", branch_name).green());
        }
    }

    Ok(())
}
