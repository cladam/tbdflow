// ===============================================================
// FILE: src/main.rs
// ===============================================================
// Project: tbdflow - Trunk-Based Development Git CLI
// Description: Entry point for the CLI application.
// Author: Claes Adamsson @cladam
// ===============================================================

use clap::{CommandFactory, Parser};
use colored::Colorize;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use tbdflow::cli::Commands;
use tbdflow::git::{get_current_branch, GitError};
use tbdflow::{changelog, cli, commit, config, git, misc};

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    let verbose = cli.verbose;
    let dry_run = cli.dry_run;

    // Before running any command, check if we are in a git repository,
    // unless the command is `init` itself.
    if !matches!(
        cli.command,
        Commands::Init | Commands::Update | Commands::Completion { .. }
    ) {
        if git::is_git_repository(verbose, dry_run).is_err() {
            println!(
                "{}",
                "Error: Not a git repository (or any of the parent directories).".red()
            );
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
            misc::handle_init_command(verbose, dry_run)?;
        }
        Commands::Update => {
            misc::handle_update_command()?;
        }
        Commands::Commit {
            r#type,
            scope,
            message,
            body,
            breaking,
            breaking_description,
            tag,
            no_verify,
            issue,
        } => {
            commit::handle_commit(
                verbose,
                dry_run,
                &config,
                r#type,
                scope,
                message,
                body,
                breaking,
                breaking_description,
                tag,
                no_verify,
                issue,
            )?;
        }
        Commands::Feature { name } => {
            println!("{}", "--- Creating feature branch ---".to_string().blue());
            let branch_name = format!("{}{}", config.branch_prefixes.feature, name);
            git::is_working_directory_clean(verbose, dry_run)?;
            git::checkout_main(verbose, dry_run, main_branch_name)?;
            git::pull_latest_with_rebase(verbose, dry_run)?;
            git::create_branch(&branch_name, None, verbose, dry_run)?;
            git::push_set_upstream(&branch_name, verbose, dry_run)?;
            println!(
                "\n{}",
                format!("Success! Switched to new feature branch: '{}'", branch_name).green()
            );
        }
        Commands::Release {
            version,
            from_commit,
        } => {
            println!("{}", "--- Creating release branch ---".to_string().blue());
            let branch_name = format!("{}{}", config.branch_prefixes.release, version);
            git::is_working_directory_clean(verbose, dry_run)?;
            git::checkout_main(verbose, dry_run, main_branch_name)?;
            git::pull_latest_with_rebase(verbose, dry_run)?;
            git::create_branch(&branch_name, from_commit.as_deref(), verbose, dry_run)?;
            git::push_set_upstream(&branch_name, verbose, dry_run)?;
            println!(
                "\n{}",
                format!("Success! Switched to new release branch: '{}'", branch_name).green()
            );
        }
        Commands::Hotfix { name } => {
            println!("{}", "--- Creating hotfix branch ---".to_string().blue());
            let branch_name = format!("{}{}", config.branch_prefixes.hotfix, name);
            git::is_working_directory_clean(verbose, dry_run)?;
            git::checkout_main(verbose, dry_run, main_branch_name)?;
            git::pull_latest_with_rebase(verbose, dry_run)?;
            git::create_branch(&branch_name, None, verbose, dry_run)?;
            git::push_set_upstream(&branch_name, verbose, dry_run)?;
            println!(
                "\n{}",
                format!("Success! Switched to new hotfix branch: '{}'", branch_name).green()
            );
        }
        Commands::Branch {
            r#type,
            name,
            issue,
            from_commit,
        } => {
            println!(
                "{}",
                "--- Creating short-lived branch ---".to_string().blue()
            );

            let prefix = misc::get_branch_prefix_or_error(&config.branch_types, &r#type)?;

            // Construct the branch name based on the configured strategy
            let branch_name = match config.issue_handling.strategy {
                config::IssueHandlingStrategy::BranchName => {
                    let issue_part = issue.map_or("".to_string(), |i| format!("{}-", i));
                    format!("{}{}{}", prefix, issue_part, name)
                }
                config::IssueHandlingStrategy::CommitScope => {
                    format!("{}{}", prefix, name)
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
        }
        Commands::Complete { r#type, name } => {
            println!(
                "{}",
                "--- Completing short-lived branch ---".to_string().blue()
            );

            // Cannot complete the main branch
            if name == main_branch_name {
                return Err(GitError::CannotCompleteMainBranch.into());
            }

            let branch_name = git::find_branch(&name, &r#type, &config, verbose, dry_run)?;
            println!("{}", format!("Branch to complete: {}", branch_name).blue());

            // pre-flight check the branch name
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

            // Create tag for release branches
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

            git::push(verbose, dry_run)?;
            git::delete_local_branch(&branch_name, verbose, dry_run)?;
            git::delete_remote_branch(&branch_name, verbose, dry_run)?;
            println!(
                "\n{}",
                format!(
                    "Success! Branch '{}' was merged into main and deleted.",
                    branch_name
                )
                .green()
            );
        }
        Commands::Sync => {
            misc::handle_sync(verbose, dry_run, &config)?;
        }
        Commands::Status => {
            println!("--- Checking status ---");
            let git_root = PathBuf::from(git::get_git_root(verbose, dry_run)?);
            let current_dir = std::env::current_dir()?;

            let status_output = if config::is_monorepo_root(&config, &current_dir, &git_root) {
                println!(
                    "{}",
                    "Monorepo root detected. Showing status for root-level files only.".yellow()
                );
                git::status_excluding_projects(&config.monorepo.project_dirs, verbose, dry_run)?
            } else {
                git::status(verbose, dry_run)?
            };

            if status_output.is_empty() {
                println!("{}", "Working directory is clean.".green());
            } else {
                println!("{}", status_output.yellow());
            }
        }
        Commands::CurrentBranch => {
            println!("{}", "--- Current branch ---".to_string().blue());
            let branch_name = get_current_branch(verbose, dry_run)?;
            println!("{}", format!("Current branch is: {}", branch_name).green());
        }
        Commands::CheckBranches => {
            misc::handle_check_branches(verbose, dry_run, &config)?;
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
        Commands::Changelog {
            from,
            to,
            unreleased,
        } => {
            //println!("{}", "--- Generating changelog ---".blue());
            // Don't print the header, good for when piping to a file
            let changelog =
                changelog::handle_changelog(verbose, dry_run, &config, from, to, unreleased)?;
            if changelog.is_empty() {
                println!(
                    "{}",
                    "No conventional commits found in the specified range.".yellow()
                );
            } else {
                println!("{}", changelog);
            }
        }
    }

    Ok(())
}
