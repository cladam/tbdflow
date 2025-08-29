use crate::{config, git};
use anyhow::Result;
use clap::Command as Commands;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use std::fs;
use std::path::PathBuf;

/// Handle update command for tbdflow
pub fn handle_update_command() -> Result<(), anyhow::Error> {
    println!("{}", "--- Checking for updates ---".blue());
    let status = self_update::backends::github::Update::configure()
        .repo_owner("cladam")
        .repo_name("tbdflow")
        .bin_name("tbdflow")
        .show_download_progress(true)
        .current_version(self_update::cargo_crate_version!())
        .build()?
        .update()?;

    println!("Update status: `{}`!", status.version());
    if status.updated() {
        println!("{}", "Successfully updated tbdflow!".green());
    } else {
        println!("{}", "tbdflow is already up to date.".green());
    }
    Ok(())
}

/// Handle init command for tbdflow
pub fn handle_init_command(verbose: bool, dry_run: bool) -> Result<()> {
    println!("--- Initialising tbdflow configuration ---");

    if git::is_git_repository(verbose, dry_run).is_err() {
        let current_dir = std::env::current_dir()?.to_string_lossy().to_string();
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "Currently not in a git repository ({}). Would you like to initialise one?",
                current_dir
            ))
            .interact()?
        {
            git::init_git_repository(verbose, dry_run)?;
            println!("{}", "New git repository initialised.".green());
        } else {
            println!("Aborted. Please run `tbdflow init` from within a git repository.");
            return Ok(());
        }
    }

    let git_root = git::get_git_root(verbose, dry_run)?;
    let current_dir = std::env::current_dir()?;
    let tbdflow_path = std::path::Path::new(&git_root).join(".tbdflow.yml");
    let mut files_created = false;

    // Check if we are in a subdirectory of the git repo
    if current_dir != PathBuf::from(&git_root) {
        // We are in a subdirectory, create a project-specific config.
        let project_config_path = current_dir.join(".tbdflow.yml");
        if !project_config_path.exists() {
            let project_config = config::Config {
                project_root: Some(".".to_string()),
                ..Default::default()
            };
            let yaml_string = serde_yaml::to_string(&project_config)?;
            fs::write(&project_config_path, yaml_string)?;
            println!(
                "{}",
                "Created project-specific .tbdflow.yml in current directory.".green()
            );
        } else {
            println!(
                "{}",
                ".tbdflow.yml already exists in this directory. Skipping.".yellow()
            );
        }
    } else {
        // We are at the root, create the global config files.
        if !tbdflow_path.exists() {
            let default_config = config::Config::default();
            let yaml_string = serde_yaml::to_string(&default_config)?;
            fs::write(&tbdflow_path, yaml_string)?;
            println!(
                "{}",
                "Created default .tbdflow.yml configuration file.".green()
            );
            files_created = true;
        } else {
            println!("{}", ".tbdflow.yml already exists. Skipping.".yellow());
        }
    }

    let dod_path = std::path::Path::new(&git_root).join(".dod.yml");
    if !dod_path.exists() {
        let default_dod = r#"
checklist:
  - "Code is clean, readable, and adheres to team coding standards."
  - "All relevant automated tests (unit, integration) pass successfully."
  - "New features or bug fixes are covered by appropriate new tests."
  - "Security implications of this change have been considered."
  - "Relevant documentation (code comments, READMEs, etc.) is updated."
"#
        .trim();
        fs::write(&dod_path, default_dod)?;
        println!("{}", "Created default .dod.yml checklist file.".green());
        files_created = true;
    } else {
        println!("{}", ".dod.yml already exists. Skipping.".yellow());
    }

    if files_created {
        println!(
            "\n{}",
            "Creating initial commit for configuration files...".blue()
        );
        git::add_all(verbose, dry_run)?;
        git::commit("chore: Initialise tbdflow configuration", verbose, dry_run)?;
        println!("{}", "Initial commit created.".green());

        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(
                "\nDo you want to link a remote repository and push the initial commit now?",
            )
            .interact()?
        {
            let remote_url: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Please enter the remote repository URL (e.g. from GitHub)")
                .interact_text()?;

            if !remote_url.is_empty() {
                git::add_remote("origin", &remote_url, verbose, dry_run)?;
                git::fetch_origin(verbose, dry_run)?;

                if git::remote_branch_exists("main", verbose, dry_run).is_ok() {
                    println!(
                        "{}",
                        "Remote 'main' branch found. Reconciling histories...".yellow()
                    );
                    git::rebase_onto_main("main", verbose, dry_run)?;
                }

                git::push_set_upstream("main", verbose, dry_run)?;
                println!(
                    "{}",
                    "Successfully linked remote and pushed initial commit.".green()
                );
            } else {
                println!("{}", "No URL provided. Skipping remote setup.".yellow());
            }
        }
    }
    Ok(())
}

pub fn handle_sync(verbose: bool, dry_run: bool, config: &config::Config) -> Result<()> {
    println!(
        "{}",
        "--- Syncing with remote and showing status ---"
            .to_string()
            .blue()
    );
    let current_branch = git::get_current_branch(verbose, dry_run)?;

    if current_branch == config.main_branch_name {
        println!("On main branch, pulling latest changes...");
        git::pull_latest_with_rebase(verbose, dry_run)?;
    } else {
        println!(
            "On feature branch '{}', rebasing onto latest '{}'...",
            current_branch, config.main_branch_name
        );
        git::fetch_origin(verbose, dry_run)?;
        git::rebase_onto_main(&config.main_branch_name, verbose, dry_run)?;
    }

    println!("\n{}", "Current status:".bold());

    let git_root = PathBuf::from(git::get_git_root(verbose, dry_run)?);
    let current_dir = std::env::current_dir()?;
    let project_root = config::find_project_root()?;

    let status_output = if let Some(proj_root) = project_root {
        let relative_path = proj_root.strip_prefix(&git_root).unwrap_or(&proj_root);
        git::status_for_path(relative_path.to_str().unwrap(), verbose, dry_run)?
    } else {
        if config::is_monorepo_root(config, &current_dir, &git_root) {
            println!(
                "{}",
                "Monorepo root detected. Showing status for root-level files only.".yellow()
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

    let log_output = git::log_graph(verbose, dry_run)?;
    println!("\n{}", "Recent activity:".bold());
    println!("{}", log_output.cyan());

    check_and_warn_for_stale_branches(verbose, dry_run, &current_branch, config)?;
    Ok(())
}

pub fn handle_check_branches(verbose: bool, dry_run: bool, config: &config::Config) -> Result<()> {
    println!(
        "{}",
        "--- Checking current branch and stale branches ---"
            .to_string()
            .blue()
    );

    let current_branch = git::get_current_branch(verbose, dry_run)?;
    if current_branch != config.main_branch_name {
        return Err(git::GitError::NotOnMainBranch(current_branch).into());
    }
    check_and_warn_for_stale_branches(verbose, dry_run, &current_branch, config)?;
    Ok(())
}

pub fn check_and_warn_for_stale_branches(
    verbose: bool,
    dry_run: bool,
    current_branch: &str,
    config: &config::Config,
) -> Result<()> {
    let stale_branches = git::get_stale_branches(
        verbose,
        dry_run,
        current_branch,
        config.stale_branch_threshold_days,
    )?;
    if !stale_branches.is_empty() {
        println!(
            "\n{}",
            "Warning: The following branches may be stale:"
                .bold()
                .yellow()
        );
        for (branch, days) in stale_branches {
            println!(
                "{}",
                format!("  - {} (last commit {} days ago)", branch, days).yellow()
            );
        }
    }
    Ok(())
}

/// Get the branch prefix for a given branch type, or return an error if the type is invalid.
pub fn get_branch_prefix_or_error<'a>(
    branch_types: &'a std::collections::HashMap<String, String>,
    r#type: &str,
) -> anyhow::Result<&'a String> {
    branch_types.get(r#type).ok_or_else(|| {
        let allowed_types = branch_types
            .keys()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
            .join(", ");
        anyhow::anyhow!(
            "Invalid branch type '{}'. Allowed types are: {}",
            r#type,
            allowed_types
        )
    })
}

/// Generate a flattened man page for tbdflow to stdout, users can pipe this to a file.
pub fn render_manpage_section(cmd: &Commands, buffer: &mut Vec<u8>) -> Result<(), anyhow::Error> {
    let man = clap_mangen::Man::new(cmd.clone());
    // Render the command's sections into the buffer
    man.render_name_section(buffer)?;
    man.render_synopsis_section(buffer)?;
    man.render_description_section(buffer)?;
    man.render_options_section(buffer)?;

    // Only add a SUBCOMMANDS header if there are subcommands
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
