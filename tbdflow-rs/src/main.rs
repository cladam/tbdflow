// ===============================================================
// FILE: src/main.rs
// ===============================================================
// Project: tbdflow-rs - Trunk-Based Development Git CLI
// Description: Entry point for the CLI application.
// Author: Claes Adamsson @cladam
// ===============================================================

use clap::Parser;
use colored::Colorize;
use tbdflow::{cli, git};
use tbdflow::cli::Commands;

/// A helper to print the result of a workflow.
fn print_workflow_result(result: Result<String, String>, success_message: String) {
    match result {
        Ok(_) => println!("\n{}", success_message.green()),
        Err(e) => println!("\n{}", format!("Workflow failed:\n{}", e).red()),
    }
}

fn main() {
    let cli = cli::Cli::parse();

    match cli.command {
        Commands::Feature { name } => {
            println!("--- Creating feature branch ---");
            let branch_name = format!("feature/{}", name);
            let result = || -> Result<String, String> {
                git::is_working_directory_clean()?;
                git::checkout_main()?;
                git::pull_latest_with_rebase()?;
                git::create_branch(&branch_name, None)?;
                Ok(branch_name.clone())
            }();
            print_workflow_result(result, format!("Success! Switched to new feature branch: '{}'", branch_name));
        }
        Commands::Release { version, from_commit } => {
            println!("--- Creating release branch ---");
            let branch_name = format!("release/{}", version);
            let result = || -> Result<String, String> {
                git::is_working_directory_clean()?;
                git::checkout_main()?;
                git::pull_latest_with_rebase()?;
                git::create_branch(&branch_name, from_commit.as_deref())?;
                Ok(branch_name.clone())
            }();
            print_workflow_result(result, format!("Success! Switched to new release branch: '{}'", branch_name));
        }
        Commands::Hotfix { name } => {
            println!("--- Creating hotfix branch ---");
            let branch_name = format!("hotfix/{}", name);
            let result = || -> Result<String, String> {
                git::is_working_directory_clean()?;
                git::checkout_main()?;
                git::pull_latest_with_rebase()?;
                git::create_branch(&branch_name, None)?;
                Ok(branch_name.clone())
            }();
            print_workflow_result(result, format!("Success! Switched to new hotfix branch: '{}'", branch_name));
        }
        Commands::Commit { r#type, scope, message, breaking } => {
            // The 'commit' command is special. It's designed to work with local changes,
            // so we don't do the clean directory check here.
            println!("--- Committing directly to main branch ---");
            let scope_part = scope.map_or("".to_string(), |s| format!("({})", s));
            let breaking_part = if breaking { "!" } else { "" };
            let header = format!("{}{}{}: {}", r#type, scope_part, breaking_part, message);
            let footer = if breaking { format!("\n\nBREAKING CHANGE: {}", message) } else { "".to_string() };
            let commit_message = format!("{}{}", header, footer);

            println!("{}", format!("Commit message will be:\n---\n{}\n---", commit_message).blue());

            let result = || -> Result<String, String> {
                git::add_all()?;
                git::commit(&commit_message)?;
                git::push()?;
                Ok("".to_string())
            }();
            print_workflow_result(result, "Successfully committed and pushed changes to main.".to_string());
        }
        Commands::Complete { r#type, name } => {
            println!("--- Completing short-lived branch ---");
            let branch_name = match r#type.as_str() {
                "feature" | "hotfix" | "release" => format!("{}/{}", r#type, name),
                _ => {
                    println!("{}", "Error: Invalid branch type. Use 'feature', 'release', or 'hotfix'.".red());
                    return;
                }
            };
            println!("{}", format!("Branch to complete: {}", branch_name).blue());

            let result = || -> Result<String, String> {
                git::is_working_directory_clean()?;
                git::checkout_main()?;
                git::pull_latest_with_rebase()?;
                git::merge_branch(&branch_name)?;
                git::push()?;
                git::delete_local_branch(&branch_name)?;
                git::delete_remote_branch(&branch_name)?;
                Ok("".to_string())
            }();
            print_workflow_result(result, format!("Success! Branch '{}' was merged into main and deleted.", branch_name));
        }
        Commands::Status => {
            println!("--- Showing git status ---");
            match git::status() {
                Ok(output) => println!("{}", output.blue()),
                Err(e) => println!("{}", format!("Error running git status:\n{}", e).red()),
            }
        }
        Commands::CurrentBranch => {
            println!("--- Getting current branch ---");
            match git::get_current_branch() {
                Ok(branch_name) => println!("{}", format!("Current branch is: {}", branch_name).green()),
                Err(e) => println!("{}", format!("Error getting current branch:\n{}", e).red()),
            }
        }
    }
}
