// ===============================================================
// FILE: src/main.rs
// ===============================================================
// Project: tbdflow - Trunk-Based Development Git CLI
// Description: Entry point for the CLI application.
// Author: Claes Adamsson @cladam
// ===============================================================

use std::io::Write;
use std::io;
use clap::{CommandFactory, Parser};
use colored::Colorize;
use tbdflow::{cli, git, config, commit, misc};
use tbdflow::cli::Commands;
use tbdflow::git::{get_current_branch, GitError};

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    let verbose = cli.verbose;

    // Before running any command, check if we are in a git repository,
    // unless the command is `init` itself.
    if !matches!(cli.command, Commands::Init | Commands::Update | Commands::Completion {..}) {
        if git::is_git_repository(verbose).is_err() {
            println!("{}", "Error: Not a git repository (or any of the parent directories).".red());
            println!("Hint: Run 'tbdflow init' to initialise a new repository here.");
            // Exit gracefully without a scary error stack trace.
            std::process::exit(1);
        }
    }

    let config = config::load_tbdflow_config()?;
    // Lookup the default branch name.
    let main_branch_name = config.main_branch_name.as_str();

    // Match the commands and execute the functionality.
    match cli.command {
        Commands::Init => {
            misc::handle_init_command(verbose)?;
        }
        Commands::Update => {
            misc::handle_update_command()?;
        }
        Commands::Commit { r#type, scope, message, breaking, breaking_description, tag, no_verify, issue, body } => {
            println!("{}", "--- Committing changes ---".to_string().blue());
            // Linting checks for commit type and issue reference
            if !commit::is_valid_commit_type(&r#type, &config) {
                // Print a helpful error message and exit
                println!("{}", format!("Error: '{}' is not a valid Conventional Commit type.", r#type).red());
                return Err(anyhow::anyhow!("Aborted: Invalid commit type."));
            }
            if !commit::is_valid_issue_key(&issue, &config) {
                println!("{}", "Issue reference is required for commits, see .tbdflow.yml file.".red());
                return Err(anyhow::anyhow!("Aborted: Issue reference required."));
            }

            // Linting checks for commit message subject line
            if let Err(subject_err) = commit::is_valid_subject_line(&message, &config) {
                println!("{}", format!("Commit message subject line error: {}", subject_err).red());
                return Err(anyhow::anyhow!("Aborted: Invalid commit message subject line."));
            }

            // Linting checks for commit message body
            if let Some(body_text) = &body {
                if !commit::is_valid_body_lines(body_text, &config) {
                    println!("{}", "Commit message body contains lines that exceed the maximum length.".red());
                    return Err(anyhow::anyhow!("Aborted: Invalid commit message body."));
                }
            }

            // Read the DoD configuration from the `.dod.yml` file.
            let dod_config = config::load_dod_config().unwrap_or_else(|e| {
                println!("{}", format!("Warning: {}. Proceeding without DoD checks.", e).yellow());
                config::DodConfig::default()
            });
            if std::path::Path::new(".dod.yml").exists() && dod_config.checklist.is_empty() {
                println!("{}", "Warning: .dod.yml found, but contains no checklist items.".yellow());
            }

            // Assemble the commit message
            let scope_part = scope.map_or("".to_string(), |s| format!("({})", s));
            let breaking_part = if breaking { "!" } else { "" };
            let header = format!("{}{}{}: {}", r#type, scope_part, breaking_part, message);

            let mut commit_message = header;

            if let Some(body_text) = body {
                commit_message.push_str("\n\n");
                commit_message.push_str(&body_text);
            }

            if let Some(desc) = breaking_description {
                commit_message.push_str(&format!("\n\nBREAKING CHANGE: {}", desc));
            }

            if let Some(issue_ref) = &issue {
                commit_message.push_str(&format!("\n\nRefs: {}", issue_ref));
            }

            // Handle the interactive DoD check to get the TODO footer
            let todo_footer_result = if no_verify || dod_config.checklist.is_empty() {
                Ok(Some(String::new())) // Skip check, success with empty footer
            } else {
                commit::handle_interactive_dod(&dod_config)
            };

            // Proceed only if the user did not abort the interactive check
            if let Some(todo_footer) = todo_footer_result? {
                commit_message.push_str(&todo_footer);

                println!("{}", format!("Commit message will be:\n---\n{}\n---", commit_message).blue());
                git::add_all(verbose)?;
                if !git::has_staged_changes(verbose)? {
                    println!("{}", "No changes added to commit.".yellow());
                    return Ok(());
                }
                let current_branch = git::get_current_branch(verbose)?;

                if current_branch == config.main_branch_name {
                    if verbose {
                        println!("--- Committing directly to main branch ---");
                    }
                    git::pull_latest_with_rebase(verbose)?;
                    git::commit(&commit_message, verbose)?;
                    git::push(verbose)?;
                    println!("\n{}", "Successfully committed and pushed changes to main.".green());
                } else {
                    if verbose {
                        println!("--- Committing to feature branch '{}' ---", current_branch);
                    }
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
        }
        Commands::Feature { name } => {
            println!("{}", "--- Creating feature branch ---".to_string().blue());
            let branch_name = format!("{}{}", config.branch_prefixes.feature, name);
            git::is_working_directory_clean(verbose)?;
            git::checkout_main(verbose, main_branch_name)?;
            git::pull_latest_with_rebase(verbose)?;
            git::create_branch(&branch_name, None, verbose)?;
            git::push_set_upstream(&branch_name, verbose)?;
            println!("\n{}", format!("Success! Switched to new feature branch: '{}'", branch_name).green());
        }
        Commands::Release { version, from_commit } => {
            println!("{}", "--- Creating release branch ---".to_string().blue());
            let branch_name = format!("{}{}", config.branch_prefixes.release, version);
            git::is_working_directory_clean(verbose)?;
            git::checkout_main(verbose, main_branch_name)?;
            git::pull_latest_with_rebase(verbose)?;
            git::create_branch(&branch_name, from_commit.as_deref(), verbose)?;
            git::push_set_upstream(&branch_name, verbose)?;
            println!("\n{}", format!("Success! Switched to new release branch: '{}'", branch_name).green());
        }
        Commands::Hotfix { name } => {
            println!("{}", "--- Creating hotfix branch ---".to_string().blue());
            let branch_name = format!("{}{}", config.branch_prefixes.hotfix, name);
            git::is_working_directory_clean(verbose)?;
            git::checkout_main(verbose, main_branch_name)?;
            git::pull_latest_with_rebase(verbose)?;
            git::create_branch(&branch_name, None, verbose)?;
            git::push_set_upstream(&branch_name, verbose)?;
            println!("\n{}", format!("Success! Switched to new hotfix branch: '{}'", branch_name).green());
        }
        Commands::Complete { r#type, name } => {
            println!("{}", "--- Completing short-lived branch ---".to_string().blue());

            // Cannot complete the main branch
            if name == main_branch_name {
                return Err(GitError::CannotCompleteMainBranch.into());
            }

            let branch_name= git::find_branch_case_insensitive(&name, &r#type, &config.branch_prefixes, verbose)?;
            println!("{}", format!("Branch to complete: {}", branch_name).blue());

            // pre-flight check the branch name
            git::branch_exists_locally(&branch_name, verbose)?;

            if r#type == "release" || r#type == "hotfix" {
                let tag_name = if r#type == "release" {
                    format!("{}{}", config.automatic_tags.release_prefix, name)
                } else {
                    format!("{}{}", config.automatic_tags.hotfix_prefix, name)
                };
                if git::tag_exists(&tag_name, verbose)? {
                    return Err(GitError::TagAlreadyExists(tag_name).into());
                }
            }

            git::is_working_directory_clean(verbose)?;
            git::checkout_main(verbose, main_branch_name)?;
            git::pull_latest_with_rebase(verbose)?;
            git::merge_branch(&branch_name, verbose)?;

            let mut should_push_tags = false;
            match r#type.as_str() {
                "release" => {
                    let tag_name = format!("{}{}", config.automatic_tags.release_prefix, name);
                    let merge_commit_hash = git::get_head_commit_hash(verbose)?;
                    git::create_tag(&tag_name, &format!("Release {}", name), &merge_commit_hash, verbose)?;
                    println!("{}", format!("Created tag '{}' on merge commit.", tag_name).green());
                    should_push_tags = true;
                }
                "hotfix" => {
                    let tag_name = format!("{}{}", config.automatic_tags.hotfix_prefix, name);
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
        Commands::Sync => {
            misc::handle_sync(verbose, &config)?;
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
        Commands::CheckBranches => {
            misc::handle_check_branches(verbose, &config)?;
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
                misc::render_manpage_section(sub, &mut buffer)?;
            }
            std::io::stdout().write_all(&buffer)?;
        }
        Commands::Completion { shell } => {
            let mut cmd = cli::Cli::command();
            let bin_name = cmd.get_name().to_string();
            clap_complete::generate(shell, &mut cmd, bin_name, &mut io::stdout());
        }
    }

    Ok(())
}
