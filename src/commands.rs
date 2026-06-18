use crate::git::RunOpts;
use crate::{config, git, intent, radar};
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

/// Options for the init command, allowing non-interactive usage.
#[derive(Debug, Clone, Default)]
pub struct InitOptions {
    /// When true, skip all interactive prompts and use defaults.
    pub non_interactive: bool,
    /// Override the main branch name (defaults to "main").
    pub main_branch: Option<String>,
    /// Remote URL to link after initialising.
    pub remote: Option<String>,
}

pub fn handle_init_command(opts: RunOpts, init_opts: InitOptions) -> Result<()> {
    println!("--- Initialising tbdflow configuration ---");

    if git::is_git_repository(opts).is_err() {
        if init_opts.non_interactive {
            // In non-interactive mode, automatically initialise the git repository.
            git::init_git_repository(opts)?;
            println!("{}", "New git repository initialised.".green());
        } else {
            let current_dir = env::current_dir()?.to_string_lossy().to_string();
            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(format!(
                    "Currently not in a git repository ({}). Would you like to initialise one?",
                    current_dir
                ))
                .interact()?
            {
                git::init_git_repository(opts)?;
                println!("{}", "New git repository initialised.".green());
            } else {
                println!("Aborted. Please run `tbdflow init` from within a git repository.");
                return Ok(());
            }
        }
    }

    let git_root = git::get_git_root(opts)?;
    let current_dir = env::current_dir()?;
    let tbdflow_path = std::path::Path::new(&git_root).join(".tbdflow.yml");
    let mut files_created = false;

    if current_dir.as_path() != std::path::Path::new(&git_root) {
        // We are in a subdirectory, create a project-specific config.
        let project_config_path = current_dir.join(".tbdflow.yml");
        if !project_config_path.exists() {
            let project_config = config::Config {
                project_root: Some(".".to_string()),
                ..build_init_config(&init_opts)
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
            let init_config = build_init_config(&init_opts);
            let yaml_string = yaml_serde::to_string(&init_config)?;
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
        git::add_all(opts)?;
        git::commit("chore: Initialise tbdflow configuration", opts)?;
        println!("{}", "Initial commit created.".green());

        // Determine remote URL: from flag, interactive prompt, or skip.
        let remote_url =
            if let Some(ref url) = init_opts.remote {
                Some(url.clone())
            } else if init_opts.non_interactive {
                None // No remote linking in non-interactive mode unless explicitly provided.
            } else {
                if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(
                    "\nDo you want to link a remote repository and push the initial commit now?",
                )
                .interact()?
            {
                let url: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Please enter the remote repository URL (e.g. from GitHub)")
                    .interact_text()?;
                if url.is_empty() { None } else { Some(url) }
            } else {
                None
            }
            };

        if let Some(url) = remote_url {
            let main_branch = init_opts.main_branch.as_deref().unwrap_or("main");

            git::add_remote("origin", &url, opts)?;
            git::fetch_origin(opts)?;

            if git::remote_branch_exists(main_branch, opts).is_ok() {
                println!(
                    "{}",
                    "Remote branch found. Reconciling histories...".yellow()
                );
                git::rebase_onto_main(main_branch, opts)?;
            }

            git::push_set_upstream(main_branch, opts)?;
            println!(
                "{}",
                "Successfully linked remote and pushed initial commit.".green()
            );
        }
    }
    Ok(())
}

/// Build a Config based on init options, falling back to defaults.
fn build_init_config(init_opts: &InitOptions) -> config::Config {
    let mut cfg = config::Config::default();

    if let Some(ref branch) = init_opts.main_branch {
        cfg.main_branch_name = branch.clone();
    }

    cfg
}

pub fn handle_info(opts: RunOpts, edit: bool) -> Result<()> {
    let git_root = git::get_git_root(RunOpts::new(false, false))?;
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

    let root_config: config::Config = if root_config_path.exists() {
        let yaml_str = fs::read_to_string(&root_config_path)?;
        yaml_serde::from_str(&yaml_str)?
    } else {
        config::Config::default()
    };

    let final_config = config::load_tbdflow_config()?;

    print_mode_and_settings(&root_config, &root_config_path, &final_config)?;
    print_review_config(&final_config.review);
    print_radar_config(&final_config.radar);
    print_ci_config(&final_config.ci_check);
    print_git_info(opts)?;

    Ok(())
}

fn print_mode_and_settings(
    root_config: &config::Config,
    root_config_path: &PathBuf,
    final_config: &config::Config,
) -> Result<()> {
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

    Ok(())
}

fn print_review_config(review: &config::ReviewConfig) {
    println!("\n{}", "--- Review ---".bold());
    if review.enabled {
        println!("Review: {}", "Enabled".green());
        println!("Strategy: {}", format!("{:?}", review.strategy).cyan());
        if !review.default_reviewers.is_empty() {
            println!(
                "Default Reviewers: {}",
                review.default_reviewers.join(", ").cyan()
            );
        }
        if let Some(ref workflow) = review.workflow {
            println!("Workflow: {}", workflow.cyan());
        }
        if !review.rules.is_empty() {
            println!(
                "Targeted Rules: {}",
                format!("{}", review.rules.len()).cyan()
            );
        }
        println!(
            "Concern Blocks Status: {}",
            if review.concern_blocks_status {
                "Yes".yellow()
            } else {
                "No".dimmed()
            }
        );
    } else {
        println!("Review: {}", "Disabled".red());
    }
}

fn print_radar_config(radar: &config::RadarConfig) {
    println!("\n{}", "--- Radar ---".bold());
    if radar.enabled {
        println!("Radar: {}", "Enabled".green());
        println!("Detection Level: {}", format!("{:?}", radar.level).cyan());
        println!(
            "On Sync: {}",
            if radar.on_sync {
                "Yes".green()
            } else {
                "No".dimmed()
            }
        );
        println!("On Commit: {}", format!("{:?}", radar.on_commit).cyan());
        if !radar.ignore_patterns.is_empty() {
            println!(
                "Ignore Patterns: {}",
                radar.ignore_patterns.join(", ").dimmed()
            );
        }
    } else {
        println!("Radar: {}", "Disabled".red());
    }
}

fn print_ci_config(ci_check: &config::CiCheckConfig) {
    println!("\n{}", "--- CI Check ---".bold());
    if ci_check.enabled {
        println!("CI Check on Sync: {}", "Enabled".green());
    } else {
        println!("CI Check on Sync: {}", "Disabled".red());
    }
}

fn print_git_info(opts: RunOpts) -> Result<()> {
    println!("\n{}", "--- Git Info ---".bold());
    if let Ok(remote_url) = git::get_remote_url(opts) {
        println!("Remote 'origin' URL: {}", remote_url.to_string().cyan());
    } else {
        println!("Remote 'origin' URL: Not found.");
    }

    let current_branch = git::get_current_branch(opts)?;
    println!("Current branch: {}", current_branch.to_string().cyan());

    if let Ok(latest_tag) = git::get_latest_tag(opts) {
        println!("Latest tag: {}", latest_tag.to_string().cyan());
    } else {
        println!("Latest tag: Not found.");
    }

    Ok(())
}

pub fn handle_sync(opts: RunOpts, config: &config::Config) -> Result<()> {
    println!(
        "{}",
        "--- Syncing with remote and showing status ---"
            .to_string()
            .blue()
    );
    let current_branch = git::get_current_branch(opts)?;

    // Anti-collision pre-flight: abort if a git operation is already in progress
    if let Some(msg) = git::check_git_operation_in_progress(opts)? {
        println!(
            "{}",
            format!("Error: {} Please resolve it before using tbdflow.", msg).red()
        );
        return Err(anyhow::anyhow!("{}", msg));
    }

    if let Ok(Some(hash)) = git::stash_create(opts) {
        let git_root = std::path::PathBuf::from(git::get_git_root(opts)?);
        intent::record_safety_snapshot(
            &git_root,
            &hash,
            &current_branch,
            "Pre-sync safety snapshot",
        )?;
        if opts.verbose {
            println!(
                "{}",
                format!(
                    "Pre-sync snapshot captured: {}",
                    &hash[..std::cmp::min(10, hash.len())]
                )
                .dimmed()
            );
        }
    }

    // Check trunk CI status before pulling to avoid importing a broken build
    if config.ci_check.enabled {
        let ci_status = git::check_ci_status(&config.main_branch_name, opts);
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
                if opts.verbose {
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
        git::pull_latest_with_rebase(opts)?;
    } else {
        println!(
            "On feature branch '{}', rebasing onto latest '{}'...",
            current_branch, config.main_branch_name
        );
        git::fetch_origin(opts)?;
        git::rebase_onto_main(&config.main_branch_name, opts)?;
    }

    println!("\n{}", "Current status:".bold());

    let status_output = git::get_scoped_status(config, opts)?;

    if status_output.is_empty() {
        println!("{}", "Working directory is clean.".green());
    } else {
        println!("{}", status_output.yellow());
    }

    let log_output = git::log_graph(opts)?;
    println!("\n{}", "Recent activity:".bold());
    println!("{}", log_output.cyan());

    // Radar: quick overlap scan
    if let Ok(Some(radar_summary)) = radar::quick_scan_for_sync(config, opts) {
        println!("\n{}", radar_summary.yellow());
    }

    check_and_warn_for_stale_branches(opts, &current_branch, config)?;
    Ok(())
}

pub fn handle_check_branches(opts: RunOpts, config: &config::Config) -> Result<()> {
    println!(
        "{}",
        "--- Checking current branch and stale branches ---"
            .to_string()
            .blue()
    );

    let current_branch = git::get_current_branch(opts)?;
    if current_branch != config.main_branch_name {
        return Err(git::GitError::NotOnMainBranch(current_branch).into());
    }
    check_and_warn_for_stale_branches(opts, &current_branch, config)?;
    Ok(())
}

pub fn check_and_warn_for_stale_branches(
    opts: RunOpts,
    current_branch: &str,
    config: &config::Config,
) -> Result<()> {
    let stale_branches =
        git::get_stale_branches(opts, current_branch, config.stale_branch_threshold_days)?;
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

pub fn handle_undo(sha: &str, no_push: bool, opts: RunOpts, config: &config::Config) -> Result<()> {
    println!(
        "{}",
        "--- Undo: The Panic Button ---".to_string().bold().red()
    );

    // Anti-collision pre-flight
    if let Some(msg) = git::check_git_operation_in_progress(opts)? {
        println!(
            "{}",
            format!("Error: {} Please resolve it before using tbdflow.", msg).red()
        );
        return Err(anyhow::anyhow!("{}", msg));
    }

    // WIP Guard: snapshot before the destructive checkout + fast-forward
    if let Ok(Some(hash)) = git::stash_create(opts) {
        let git_root = std::path::PathBuf::from(git::get_git_root(opts)?);
        let current_branch = git::get_current_branch(opts)?;
        intent::record_safety_snapshot(
            &git_root,
            &hash,
            &current_branch,
            "Pre-undo safety snapshot",
        )?;
        if opts.verbose {
            println!(
                "{}",
                format!(
                    "Pre-undo snapshot captured: {}",
                    &hash[..std::cmp::min(10, hash.len())]
                )
                .dimmed()
            );
        }
    }

    let main_branch = &config.main_branch_name;

    if !git::commit_exists(sha, opts)? {
        println!(
            "{}",
            format!("Error: Commit '{}' does not exist in this repository.", sha).red()
        );
        return Err(anyhow::anyhow!("Commit not found: {}", sha));
    }

    let subject = git::get_commit_subject(sha, opts)?;
    println!(
        "{}",
        format!("Commit to revert: {} ({})", sha, subject).yellow()
    );

    git::is_working_directory_clean(opts)?;

    // Sync with remote (fast-forward only to preserve commit SHAs)
    println!("Syncing with remote before reverting...");
    git::checkout_main(opts, main_branch)?;
    git::pull_fast_forward_only(opts)?;

    if !git::is_ancestor_of(sha, main_branch, opts)? {
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
    git::revert_commit(sha, opts)?;

    if no_push {
        println!(
            "{}",
            "Revert commit created locally (--no-push). Remember to push when ready.".yellow()
        );
    } else {
        println!("Pushing revert to remote...");
        git::push(opts)?;
        println!(
            "\n{}",
            format!(
                "Success! Commit '{}' has been reverted on '{}'.",
                sha, main_branch
            )
            .green()
        );
    }

    let log_output = git::log_graph(opts)?;
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
