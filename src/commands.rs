use crate::{config, git, radar};
use anyhow::Result;
use clap::Command as Commands;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use std::env;
use std::fs;
use std::path::PathBuf;

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

pub fn handle_init_command(verbose: bool, dry_run: bool) -> Result<()> {
    println!("--- Initialising tbdflow configuration ---");

    if git::is_git_repository(verbose, dry_run).is_err() {
        let current_dir = env::current_dir()?.to_string_lossy().to_string();
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
    let current_dir = env::current_dir()?;
    let tbdflow_path = std::path::Path::new(&git_root).join(".tbdflow.yml");
    let mut files_created = false;

    if current_dir.as_path() != std::path::Path::new(&git_root) {
        // We are in a subdirectory, create a project-specific config.
        let project_config_path = current_dir.join(".tbdflow.yml");
        if !project_config_path.exists() {
            let project_config = config::Config {
                project_root: Some(".".to_string()),
                ..Default::default()
            };
            let yaml_string = yaml_serde::to_string(&project_config)?;
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
        if !tbdflow_path.exists() {
            let default_config = config::Config::default();
            let yaml_string = yaml_serde::to_string(&default_config)?;
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

pub fn handle_info(verbose: bool, dry_run: bool, edit: bool) -> Result<()> {
    let git_root = git::get_git_root(false, false)?;
    let root_config_path = PathBuf::from(&git_root).join(".tbdflow.yml");

    if edit {
        let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
        std::process::Command::new(&editor)
            .arg(&root_config_path)
            .status()
            .map_err(|e| anyhow::anyhow!("Failed to open editor: {}", e))?;
        return Ok(());
    }

    println!("{}", "--- tbdflow Configuration ---".blue());

    // Load root config or default
    let root_config: config::Config = if root_config_path.exists() {
        let yaml_str = fs::read_to_string(&root_config_path)?;
        yaml_serde::from_str(&yaml_str)?
    } else {
        config::Config::default()
    };

    let final_config = config::load_tbdflow_config()?;

    if let Some(project_root) = config::find_project_root()? {
        let project_config_path = project_root.join(".tbdflow.yml");
        if project_config_path.exists() {
            println!("Mode: {} (Project)", "Monorepo".to_string().bold());
            println!("Project Root: {}", project_root.to_string_lossy());
            println!(
                "Loaded project-specific config from: {}",
                project_config_path.to_string_lossy()
            );

            let project_yaml_str = fs::read_to_string(&project_config_path)?;
            let project_config: config::Config = yaml_serde::from_str(&project_yaml_str)?;

            println!("\n{}", "--- Settings ---".bold());

            // Compare and print settings
            let main_branch_source =
                if project_config.main_branch_name != root_config.main_branch_name {
                    "(overridden by project)".yellow()
                } else {
                    "(inherited from root)".dimmed()
                };
            println!(
                "Main Branch: {} {}",
                project_config.main_branch_name, main_branch_source
            );

            let issue_strategy_source =
                if project_config.issue_handling.strategy != root_config.issue_handling.strategy {
                    "(overridden by project)".yellow()
                } else {
                    "(inherited from root)".dimmed()
                };
            println!(
                "Issue Handling Strategy: {:?} {}",
                format!("{:?}", project_config.issue_handling.strategy).cyan(),
                issue_strategy_source
            );
        }
    } else {
        // Not in a sub-project, check if we are at the root of a monorepo
        if root_config.monorepo.enabled && !root_config.monorepo.project_dirs.is_empty() {
            println!("Mode: {} (Root)", "Monorepo".to_string().bold());
            println!(
                "Loaded root config from: {}",
                root_config_path.to_string_lossy()
            );
            println!("Project Directories:");
            for dir in &root_config.monorepo.project_dirs {
                println!("- {}", dir.cyan());
            }
        } else {
            println!("Mode: {}", "Standalone".bold());
            if root_config_path.exists() {
                println!("Loaded config from: {}", root_config_path.to_string_lossy());
            }
        }

        println!("\n{}", "--- Settings ---".bold());
        println!(
            "Main Branch: {}",
            root_config.main_branch_name.to_string().cyan()
        );
        println!(
            "Issue Handling Strategy: {}",
            format!("{:?}", root_config.issue_handling.strategy).cyan(),
        );
    }

    // Common settings for all modes, using the final merged config
    println!(
        "Stale Branch Threshold: {} days",
        format!("{}", final_config.stale_branch_threshold_days).cyan()
    );

    let lint_status = if final_config.lint.is_some() {
        "Enabled".green()
    } else {
        "Disabled".red()
    };
    println!("Commit Linting: {}", lint_status);

    println!("\n{}", "--- Review ---".bold());
    if final_config.review.enabled {
        println!("Review: {}", "Enabled".green());
        println!(
            "Strategy: {}",
            format!("{:?}", final_config.review.strategy).cyan()
        );
        if !final_config.review.default_reviewers.is_empty() {
            println!(
                "Default Reviewers: {}",
                final_config.review.default_reviewers.join(", ").cyan()
            );
        }
        if let Some(ref workflow) = final_config.review.workflow {
            println!("Workflow: {}", workflow.cyan());
        }
        if !final_config.review.rules.is_empty() {
            println!(
                "Targeted Rules: {}",
                format!("{}", final_config.review.rules.len()).cyan()
            );
        }
        println!(
            "Concern Blocks Status: {}",
            if final_config.review.concern_blocks_status {
                "Yes".yellow()
            } else {
                "No".dimmed()
            }
        );
    } else {
        println!("Review: {}", "Disabled".red());
    }

    println!("\n{}", "--- Radar ---".bold());
    if final_config.radar.enabled {
        println!("Radar: {}", "Enabled".green());
        println!(
            "Detection Level: {}",
            format!("{:?}", final_config.radar.level).cyan()
        );
        println!(
            "On Sync: {}",
            if final_config.radar.on_sync {
                "Yes".green()
            } else {
                "No".dimmed()
            }
        );
        println!(
            "On Commit: {}",
            format!("{:?}", final_config.radar.on_commit).cyan()
        );
        if !final_config.radar.ignore_patterns.is_empty() {
            println!(
                "Ignore Patterns: {}",
                final_config.radar.ignore_patterns.join(", ").dimmed()
            );
        }
    } else {
        println!("Radar: {}", "Disabled".red());
    }

    println!("\n{}", "--- CI Check ---".bold());
    if final_config.ci_check.enabled {
        println!("CI Check on Sync: {}", "Enabled".green());
    } else {
        println!("CI Check on Sync: {}", "Disabled".red());
    }

    println!("\n{}", "--- Git Info ---".bold());
    if let Ok(remote_url) = git::get_remote_url(verbose, dry_run) {
        println!("Remote 'origin' URL: {}", remote_url.to_string().cyan());
    } else {
        println!("Remote 'origin' URL: Not found.");
    }

    let current_branch = git::get_current_branch(verbose, dry_run)?;
    println!("Current branch: {}", current_branch.to_string().cyan());

    if let Ok(latest_tag) = git::get_latest_tag(verbose, dry_run) {
        println!("Latest tag: {}", latest_tag.to_string().cyan());
    } else {
        println!("Latest tag: Not found.");
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

    // Check trunk CI status before pulling to avoid importing a broken build
    if config.ci_check.enabled {
        let ci_status = git::check_ci_status(&config.main_branch_name, verbose, dry_run);
        match ci_status {
            git::CiStatus::Green => {
                println!("{}", "Pre-flight CI check: trunk is green.".green());
            }
            git::CiStatus::Failed => {
                println!(
                    "\n{}",
                    "The trunk is currently failing CI. Pulling now might break your local build."
                        .bold()
                        .yellow()
                );
                let should_continue = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Continue with sync?")
                    .default(false)
                    .interact()?;
                if !should_continue {
                    println!("{}", "Sync aborted.".yellow());
                    return Ok(());
                }
            }
            git::CiStatus::Pending => {
                println!("\n{}", "⏳ Trunk CI is still running.".bold().yellow());
                let should_continue = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Pull anyway?")
                    .default(false)
                    .interact()?;
                if !should_continue {
                    println!("{}", "Sync aborted.".yellow());
                    return Ok(());
                }
            }
            git::CiStatus::Unknown(reason) => {
                if verbose {
                    println!(
                        "{} {}",
                        "Pre-flight CI check skipped:".dimmed(),
                        reason.dimmed()
                    );
                }
                // Proceed silently — no CI info available is not a blocker
            }
        }
    }

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

    let status_output = git::get_scoped_status(config, verbose, dry_run)?;

    if status_output.is_empty() {
        println!("{}", "Working directory is clean.".green());
    } else {
        println!("{}", status_output.yellow());
    }

    let log_output = git::log_graph(verbose, dry_run)?;
    println!("\n{}", "Recent activity:".bold());
    println!("{}", log_output.cyan());

    // Radar: quick overlap scan
    if let Ok(Some(radar_summary)) = radar::quick_scan_for_sync(config, verbose, dry_run) {
        println!("\n{}", radar_summary.yellow());
    }

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

pub fn get_branch_prefix_or_error<'a>(
    branch_types: &'a std::collections::HashMap<String, String>,
    r#type: &str,
) -> Result<&'a String> {
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

pub fn handle_undo(
    sha: &str,
    no_push: bool,
    verbose: bool,
    dry_run: bool,
    config: &config::Config,
) -> Result<()> {
    println!(
        "{}",
        "--- Undo: The Panic Button ---".to_string().bold().red()
    );

    let main_branch = &config.main_branch_name;

    if !git::commit_exists(sha, verbose, dry_run)? {
        println!(
            "{}",
            format!("Error: Commit '{}' does not exist in this repository.", sha).red()
        );
        return Err(anyhow::anyhow!("Commit not found: {}", sha));
    }

    let subject = git::get_commit_subject(sha, verbose, dry_run)?;
    println!(
        "{}",
        format!("Commit to revert: {} ({})", sha, subject).yellow()
    );

    git::is_working_directory_clean(verbose, dry_run)?;

    // Sync with remote (fast-forward only to preserve commit SHAs)
    println!("Syncing with remote before reverting...");
    git::checkout_main(verbose, dry_run, main_branch)?;
    git::pull_fast_forward_only(verbose, dry_run)?;

    if !git::is_ancestor_of(sha, main_branch, verbose, dry_run)? {
        println!(
            "{}",
            format!(
                "Error: Commit '{}' is not on the '{}' branch. Undo only works on trunk commits.",
                sha, main_branch
            )
            .red()
        );
        return Err(anyhow::anyhow!(
            "Commit '{}' is not on '{}'.",
            sha,
            main_branch
        ));
    }

    println!("{}", format!("Reverting commit {}...", sha).blue());
    git::revert_commit(sha, verbose, dry_run)?;

    if no_push {
        println!(
            "{}",
            "Revert commit created locally (--no-push). Remember to push when ready.".yellow()
        );
    } else {
        println!("Pushing revert to remote...");
        git::push(verbose, dry_run)?;
        println!(
            "\n{}",
            format!(
                "Success! Commit '{}' has been reverted on '{}'.",
                sha, main_branch
            )
            .green()
        );
    }

    let log_output = git::log_graph(verbose, dry_run)?;
    println!("\n{}", "Recent activity:".bold());
    println!("{}", log_output.cyan());

    println!(
        "\n{}",
        "Hint: The reverted changes are still in your git history. You can re-apply them later."
            .dimmed()
    );

    Ok(())
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
