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
    println!("Hello, world!");

    let cli = cli::Cli::parse();

    match cli.command {
        Commands::Feature { name } => {
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
        Commands::Status => {
            match git::status() {
                Ok(output) => println!("{}", output.blue()),
                Err(e) => println!("{}", format!("Error running git status:\n{}", e).red()),
            }
        }
        Commands::CurrentBranch => {
            match git::get_current_branch() {
                Ok(branch_name) => println!("{}", format!("Current branch is: {}", branch_name).green()),
                Err(e) => println!("{}", format!("Error getting current branch:\n{}", e).red()),
            }
        }
        _ => {
            println!("Command not implemented yet.");
        }
    }
/*    match cli.command {
        cli::Commands::Feature { name } => {
            let result = tbdflow::git::feature_branch(&name);
            print_workflow_result(result, format!("Feature branch '{}' created successfully.", name));
        }
        cli::Commands::Release { version, from_commit } => {
            let result = tbdflow::git::release_branch(&version, from_commit.as_deref());
            print_workflow_result(result, format!("Release branch '{}' created successfully.", version));
        }
        cli::Commands::Hotfix { name } => {
            let result = tbdflow::git::hotfix_branch(&name);
            print_workflow_result(result, format!("Hotfix branch '{}' created successfully.", name));
        }
        cli::Commands::Commit { r#type, scope, message, breaking } => {
            let result = tbdflow::git::commit(&r#type, scope.as_deref(), &message, breaking);
            print_workflow_result(result, "Commit successful.".to_string());
        }
        cli::Commands::Complete { r#type, name } => {
            let result = tbdflow::git::complete_branch(&r#type, &name);
            print_workflow_result(result, format!("{} '{}' completed successfully.", r#type, name));
        }
        cli::Commands::Status => {
            let result = tbdflow::git::status();
            print_workflow_result(result, "Current git status displayed.".to_string());
        }
    }*/
}
