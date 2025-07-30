// ===============================================================
// FILE: src/main.rs
// ===============================================================
// Project: tbdflow - Trunk-Based Development Git CLI
// Description: Entry point for the CLI application.
// Author: Claes Adamsson @cladam
// ===============================================================

use std::io::Write;
use anyhow::Context;
use clap::{Command, CommandFactory, Parser};
use colored::Colorize;
use dialoguer::{MultiSelect, theme::ColorfulTheme, Confirm};
use serde::Deserialize;
use tbdflow::{cli, git};
use tbdflow::cli::Commands;
use tbdflow::git::{get_current_branch, GitError};

#[derive(Debug, Deserialize, Default)]
struct DodConfig {
    issue_reference_required: Option<bool>,
    #[serde(default)]
    checklist: Vec<String>,
}

fn render_manpage_section(cmd: &Command, buffer: &mut Vec<u8>) -> Result<(), anyhow::Error> {
    let man = clap_mangen::Man::new(cmd.clone());
    // Render the command's sections into the buffer
    man.render_name_section(buffer)?;
    man.render_synopsis_section(buffer)?;
    man.render_description_section(buffer)?;
    man.render_options_section(buffer)?;

    // Only add SUBCOMMANDS header if there are subcommands
    if cmd.has_subcommands() {
        use std::io::Write;
        writeln!(buffer, "\nSUBCOMMANDS\n")?;
        let mut cmd_mut = cmd.clone();
        for sub in cmd_mut.get_subcommands_mut() {
            render_manpage_section(sub, buffer)?;
        }
    }

    Ok(())
}

/// Reads the DoD configuration from `.dod.yml` file in the current directory (root of the git repository).
fn read_dod_config() -> anyhow::Result<DodConfig> {
    let content = std::fs::read_to_string(".dod.yml")
        .context("Failed to read .dod.yml")?;
    let config: DodConfig = serde_yaml::from_str(&content)
        .context("Failed to parse .dod.yml")?;
    Ok(config)
}

/// Runs the checklist interactively, allowing the user to confirm each item before committing.
fn run_checklist_interactive(checklist: &[String]) -> anyhow::Result<Vec<usize>> {
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Please confirm each item before committing:")
        .items(checklist)
        .interact()?;
    Ok(selections)
}

/// Builds the TODO footer for the commit message based on unchecked items in the checklist.
fn build_todo_footer(checklist: &[String], checked_indices: &[usize]) -> String {
    //let checked_indices: Vec<usize> = checked_indices.iter().cloned().collect();
    let unchecked_items: Vec<String> = checklist
        .iter()
        .enumerate()
        .filter(|(i, _)| !checked_indices.contains(&i))
        .map(|(_, item)| format!("- [ ] {}", item))
        //.filter_map(|(i, item)| if !checked_indices.contains(&i) { Some(item.clone()) } else { None })
        .collect();
    if unchecked_items.is_empty() {
        String::new()
    } else {
        format!("\n\nTODO:\n{}", unchecked_items.join("\n"))
    }
}

/// Handles the interactive commit process, including checklist confirmation and issue reference handling.
fn handle_interactive_commit(config: &DodConfig, base_message: &str, issue: &Option<String>) -> Result<Option<String>, anyhow::Error> {
    // Start with the base commit message.
    let mut commit_message = base_message.to_string();

    let checked = run_checklist_interactive(&config.checklist)?;
    if checked.len() != config.checklist.len() {
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Warning: Not all DoD items were checked. Proceed by adding a 'TODO' list to the commit message?")
            .interact()?
        {
            let todo_footer = build_todo_footer(&config.checklist, &checked);
            commit_message.push_str(&todo_footer);
        } else {
            println!("Commit aborted.");
            return Ok(None);
        }
    }

    if config.issue_reference_required.unwrap_or(false) && issue.is_none() {
        println!("{}", "Issue reference is required for commits.".red());
        return Err(anyhow::anyhow!("Aborted: Issue reference required."));
    }

    // Append the issue reference as a trailer/footer if required.
    if config.issue_reference_required.unwrap_or(false) {
        if let Some(issue_ref) = issue {
            commit_message.push_str(&format!("\n\nRefs: {}", issue_ref));
        }
    }

    Ok(Some(commit_message))
}

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    let verbose = cli.verbose;
    let config = read_dod_config().unwrap_or_else(|e| {
        println!("{}", format!("Warning: {}. Proceeding without DoD checks.", e).yellow());
        DodConfig::default()
    });
    if std::path::Path::new(".dod.yml").exists() && config.checklist.is_empty() {
        println!("{}", "Warning: .dod.yml found, but contains no checklist items.".yellow());
    }

    match cli.command {
        Commands::Feature { name } => {
            println!("{}", "--- Creating feature branch ---".to_string().blue());
            let branch_name = format!("feature_{}", name);
            git::is_working_directory_clean(verbose)?;
            git::checkout_main(verbose)?;
            git::pull_latest_with_rebase(verbose)?;
            git::create_branch(&branch_name, None, verbose)?;
            git::push_set_upstream(&branch_name, verbose)?;
            println!("\n{}", format!("Success! Switched to new feature branch: '{}'", branch_name).green());
        }
        Commands::Release { version, from_commit } => {
            println!("{}", "--- Creating release branch ---".to_string().blue());
            let branch_name = format!("release_{}", version);
            git::is_working_directory_clean(verbose)?;
            git::checkout_main(verbose)?;
            git::pull_latest_with_rebase(verbose)?;
            git::create_branch(&branch_name, from_commit.as_deref(), verbose)?;
            git::push_set_upstream(&branch_name, verbose)?;
            println!("\n{}", format!("Success! Switched to new release branch: '{}'", branch_name).green());
        }
        Commands::Hotfix { name } => {
            println!("{}", "--- Creating hotfix branch ---".to_string().blue());
            let branch_name = format!("hotfix_{}", name);
            git::is_working_directory_clean(verbose)?;
            git::checkout_main(verbose)?;
            git::pull_latest_with_rebase(verbose)?;
            git::create_branch(&branch_name, None, verbose)?;
            git::push_set_upstream(&branch_name, verbose)?;
            println!("\n{}", format!("Success! Switched to new hotfix branch: '{}'", branch_name).green());
        }
        Commands::Commit { r#type, scope, message, breaking, breaking_description, tag, no_verify, issue } => {
            println!("{}", "--- Committing changes ---".to_string().blue());
            let scope_part = scope.map_or("".to_string(), |s| format!("({})", s));
            let breaking_part = if breaking { "!" } else { "" };
            let header = format!("{}{}{}: {}", r#type, scope_part, breaking_part, message);
            let footer = if let Some(desc) = breaking_description {
                format!("\n\nBREAKING CHANGE: {}", desc)
            } else {
                "".to_string()
            };
            let commit_message = format!("{}{}", header, footer);

            println!("{}", format!("Commit message will be:\n---\n{}\n---", commit_message).blue());

            // Stage changes first, before any other operations.
            git::add_all(verbose)?;

            let current_branch = get_current_branch(verbose)?;

            if current_branch == "main" {
                println!("--- Committing directly to main branch ---");
                // Now that changes are staged, `pull --rebase --autostash` will work correctly.
                git::pull_latest_with_rebase(verbose)?;
                git::commit(&commit_message, verbose)?;
                git::push(verbose)?;
                println!("\n{}", "Successfully committed and pushed changes to main.".green());
            } else {
                println!("--- Committing to feature branch '{}' ---", current_branch);
                // For feature branches, we just commit and push the staged changes.
                git::commit(&commit_message, verbose)?;
                git::push(verbose)?;
                println!("\n{}", format!("Successfully pushed changes to '{}'.", current_branch).green());
            }

            if let Some(tag_name) = tag {
                let commit_hash = git::get_head_commit_hash(verbose)?;
                git::create_tag(&tag_name, &commit_message, &commit_hash, verbose)?;
                git::push_tags(verbose)?;
                println!("{}", format!("Success! Created and pushed tag '{}'", tag_name).green());
            }
        }
        Commands::Complete { r#type, name } => {
            println!("{}", "--- Completing short-lived branch ---".to_string().blue());
            let branch_name = match r#type.as_str() {
                "feature" | "hotfix" | "release" => format!("{}_{}", r#type, name),
                _ => return Err(GitError::InvalidBranchType(r#type).into()),
            };
            println!("{}", format!("Branch to complete: {}", branch_name).blue());

            git::is_working_directory_clean(verbose)?;
            git::checkout_main(verbose)?;
            git::pull_latest_with_rebase(verbose)?;
            git::merge_branch(&branch_name, verbose)?;

            let mut should_push_tags = false;
            match r#type.as_str() {
                "release" => {
                    let tag_name = format!("v{}", name);
                    let merge_commit_hash = git::get_head_commit_hash(verbose)?;
                    git::create_tag(&tag_name, &format!("Release {}", name), &merge_commit_hash, verbose)?;
                    println!("{}", format!("Created tag '{}' on merge commit.", tag_name).green());
                    should_push_tags = true;
                }
                "hotfix" => {
                    let tag_name = format!("hotfix/{}", name);
                    let merge_commit_hash = git::get_head_commit_hash(verbose)?;
                    git::create_tag(&tag_name, &format!("Hotfix {}", name), &merge_commit_hash, verbose)?;
                    println!("{}", format!("Created tag '{}' on merge commit.", tag_name).green());
                    should_push_tags = true;
                }
                _ => {} // Do nothing for feature branches
            }

            git::push(verbose)?;
            if should_push_tags {
                git::push_tags(verbose)?;
            }
            git::push(verbose)?;
            git::delete_local_branch(&branch_name, verbose)?;
            git::delete_remote_branch(&branch_name, verbose)?;
            println!("\n{}", format!("Success! Branch '{}' was merged into main and deleted.", branch_name).green());
        }
        Commands::Status => {
            println!("{}", "--- Checking status ---".to_string().blue());
            let output = git::status(verbose)?;
            println!("{}", output.blue());
        }
        Commands::CurrentBranch => {
            println!("{}", "--- Current branch ---".to_string().blue());
            let branch_name = get_current_branch(verbose)?;
            println!("{}", format!("Current branch is: {}", branch_name).green());
        }
        Commands::Sync => {
            println!("{}", "--- Syncing with remote and showing status ---".to_string().blue());
            if get_current_branch(verbose)? != "main" {
                git::checkout_main(verbose)?;
            }
            git::pull_latest_with_rebase(verbose)?;

            // Add the status check to the sync workflow
            println!("\n{}", "Current status:".bold());
            let status_output = git::status(verbose)?;
            if status_output.is_empty() {
                println!("{}", "Working directory is clean.".green());
            } else {
                // Show local changes in yellow to draw attention to them.
                println!("{}", status_output.yellow());
            }

            let log_output = git::log_graph(verbose)?;
            println!("\n{}", "Recent activity on main:".bold());
            println!("{}", log_output.cyan());

            // Adding the stale branch check to the sync workflow
            println!("\n{}", "Checking for stale branches:".bold());
            git::check_and_warn_for_stale_branches(verbose)?;
        }
        Commands::CheckBranches => {
            println!("{}", "--- Checking for stale branches ---".to_string().blue());
            let current_branch = get_current_branch(verbose)?;
            if current_branch != "main" {
                return Err(GitError::NotOnMainBranch(current_branch).into());
            }
            git::check_and_warn_for_stale_branches(verbose)?;
        }
        Commands::GenerateManPage => {
            println!("{}", "--- Generating a man page ---".to_string().blue());
            let mut cmd = cli::Cli::command();
            let mut buffer: Vec<u8> = Default::default();
            // Render the main command sections
            let man = clap_mangen::Man::new(cmd.clone());
            man.render(&mut buffer)?;
            writeln!(buffer, "\n--------------------------------------------------------------------------------\n")?;

            // Manually render each subcommand's details into the same buffer
            for sub in cmd.get_subcommands_mut() {
                render_manpage_section(sub, &mut buffer)?;
            }
            std::io::stdout().write_all(&buffer)?;
        }
    }

    Ok(())
}
