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
use tbdflow::{changelog, cli, commit, branch, config, git, misc, wizard};

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
        Commands::Config { get_dod } => {
            if get_dod {
                // Print the DoD checklist for our plugin
                if let Ok(dod_config) = config::load_dod_config() {
                    for item in dod_config.checklist {
                        println!("{}", item);
                    }
                }
                // Silently exit if no .dod.yml is found
                // Add more functionality later for the config command ...
            }
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
            include_projects,
        } => {
            if r#type.is_none() || message.is_none() {
                // Enter interactive wizard mode
                let wizard_result = wizard::run_commit_wizard(&config)?;
                commit::handle_commit(
                    verbose,
                    dry_run,
                    &config,
                    wizard_result.r#type,
                    wizard_result.scope,
                    wizard_result.message,
                    wizard_result.body,
                    wizard_result.breaking,
                    wizard_result.breaking_description,
                    wizard_result.tag,
                    no_verify, // Still respect no_verify from flags
                    wizard_result.issue,
                    include_projects,
                )?;
            } else {
                commit::handle_commit(
                    verbose,
                    dry_run,
                    &config,
                    r#type.unwrap(),
                    scope,
                    message.unwrap(),
                    body,
                    breaking,
                    breaking_description,
                    tag,
                    no_verify,
                    issue,
                    include_projects,
                )?;
            }
        }
        Commands::Branch {
            r#type,
            name,
            issue,
            from_commit,
        } => {
            if r#type.is_none() || name.is_none() {
                // Enter interactive wizard mode
                let wizard_result = wizard::run_branch_wizard(&config)?;
                branch::handle_branch(
                    Some(wizard_result.branch_type),
                    &config,
                    Some(wizard_result.name),
                    wizard_result.issue,
                    wizard_result.from_commit,
                    dry_run,
                    verbose,
                )?;
            } else {
                branch::handle_branch(
                    r#type,
                    &config,
                    name,
                    issue,
                    from_commit,
                    dry_run,
                    verbose,
                )?;
            }
        }
        Commands::Complete { r#type, name } => {
            if r#type.is_none() || name.is_none() {
                // Enter interactive wizard mode
                let wizard_result = wizard::run_complete_wizard(&config)?;
                branch::handle_complete(
                    wizard_result.branch_type,
                    wizard_result.name,
                    &config,
                    dry_run,
                    verbose,
                )?;
            } else {
                branch::handle_complete(r#type.unwrap(), name.unwrap(), &config, dry_run, verbose)?;
            }
        }
        Commands::Sync => {
            misc::handle_sync(verbose, dry_run, &config)?;
        }
        Commands::Status => {
            println!("--- Checking status ---");
            let git_root = PathBuf::from(git::get_git_root(verbose, dry_run)?);
            let project_root = config::find_project_root()?;
            let current_dir = std::env::current_dir()?;

            let status_output = if let Some(proj_root) = project_root {
                // We are in a sub-project, so scope the status to its root.
                // if the relative path is same as current dir we send in "."
                if current_dir == proj_root {
                    git::status_for_path(".", verbose, dry_run)?
                } else {
                    let relative_path = proj_root.strip_prefix(&git_root).unwrap_or(&proj_root);
                    git::status_for_path(relative_path.to_str().unwrap(), verbose, dry_run)?
                }
            } else {
                // We are at the monorepo root.
                if config::is_monorepo_root(&config, &current_dir, &git_root) {
                    println!(
                        "{}",
                        "Monorepo root detected. Showing status for root-level files only."
                            .yellow()
                    );
                    git::status_excluding_projects(&config.monorepo.project_dirs, verbose, dry_run)?
                } else {
                    git::status(verbose, dry_run)?
                }
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
