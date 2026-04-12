use clap::{CommandFactory, Parser};
use colored::Colorize;
use std::io;
use std::io::Write;
use tbdflow::cli::Commands;
use tbdflow::cli::TaskAction;
use tbdflow::commit::CommitParams;
use tbdflow::git::get_current_branch;
use tbdflow::{
    branch, changelog, cli, commands, commit, config, git, intent, radar, review, wizard,
};

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    let verbose = cli.verbose;
    let dry_run = cli.dry_run;

    if !matches!(
        cli.command,
        Commands::Init | Commands::Update | Commands::Completion { .. }
    ) && git::is_git_repository(verbose, dry_run).is_err()
    {
        println!(
            "{}",
            "Error: Not a git repository (or any of the parent directories).".red()
        );
        println!("Hint: Run 'tbdflow init' to initialise a new repository here.");
        std::process::exit(1);
    }

    let config = config::load_tbdflow_config()?;

    match cli.command {
        Commands::Init => {
            commands::handle_init_command(verbose, dry_run)?;
        }
        Commands::Info { edit } => {
            commands::handle_info(verbose, dry_run, edit)?;
        }
        Commands::Config { get_dod } => {
            if get_dod {
                if let Ok(dod_config) = config::load_dod_config() {
                    for item in dod_config.checklist {
                        println!("{}", item);
                    }
                }
            }
        }
        Commands::HeadSha => {
            let sha = git::get_head_commit_hash(verbose, dry_run)?;
            println!("{}", &sha[..std::cmp::min(7, sha.len())]);
        }
        Commands::Update => {
            commands::handle_update_command()?;
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
            let params = match (r#type, message) {
                (Some(t), Some(m)) => CommitParams {
                    r#type: t,
                    scope,
                    message: m,
                    body,
                    breaking,
                    breaking_description,
                    tag,
                    issue,
                    include_projects,
                    no_verify,
                },
                _ => {
                    let w = wizard::run_commit_wizard(&config)?;
                    CommitParams {
                        r#type: w.r#type,
                        scope: w.scope,
                        message: w.message,
                        body: w.body,
                        breaking: w.breaking,
                        breaking_description: w.breaking_description,
                        tag: w.tag,
                        issue: w.issue,
                        include_projects,
                        no_verify,
                    }
                }
            };

            commit::handle_commit(verbose, dry_run, &config, params)?;
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
                branch::handle_branch(r#type, &config, name, issue, from_commit, dry_run, verbose)?;
            }
        }
        Commands::Complete { r#type, name } => match (r#type, name) {
            (Some(t), Some(n)) => {
                branch::handle_complete(t, n, &config, dry_run, verbose)?;
            }
            _ => {
                let wizard_result = wizard::run_complete_wizard(&config)?;
                branch::handle_complete(
                    wizard_result.branch_type,
                    wizard_result.name,
                    &config,
                    dry_run,
                    verbose,
                )?;
            }
        },
        Commands::Sync => {
            commands::handle_sync(verbose, dry_run, &config)?;
        }
        Commands::Radar => {
            radar::handle_radar(verbose, dry_run, &config)?;
        }
        Commands::Status => {
            println!("--- Checking status ---");
            let status_output = git::get_scoped_status(&config, verbose, dry_run)?;

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
            commands::handle_check_branches(verbose, dry_run, &config)?;
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
                commands::render_manpage_section(sub, &mut buffer)?;
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
            if from.is_none() && to.is_none() && !unreleased {
                // Enter interactive wizard mode
                let wizard_result = wizard::run_changelog_wizard()?;
                let changelog = changelog::handle_changelog(
                    verbose,
                    dry_run,
                    &config,
                    wizard_result.from,
                    wizard_result.to,
                    wizard_result.unreleased,
                )?;
                if changelog.is_empty() {
                    println!(
                        "{}",
                        "No conventional commits found in the specified range.".yellow()
                    );
                } else {
                    println!("{}", changelog);
                }
            } else {
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
        Commands::Undo { sha, no_push } => {
            commands::handle_undo(&sha, no_push, verbose, dry_run, &config)?;
        }
        Commands::Note { message, show } => {
            let git_root = std::path::PathBuf::from(git::get_git_root(verbose, dry_run)?);
            let current_branch = git::get_current_branch(verbose, dry_run)?;
            if show {
                intent::show_intent_log(&git_root, Some(&current_branch))?;
            } else if let Some(msg) = message {
                intent::add_note(&git_root, &msg, &current_branch)?;
                println!("{}", format!("Note recorded: \"{}\"", msg).green());
            } else {
                intent::show_intent_log(&git_root, Some(&current_branch))?;
            }
        }
        Commands::Task(action) => {
            let git_root = std::path::PathBuf::from(git::get_git_root(verbose, dry_run)?);
            let current_branch = git::get_current_branch(verbose, dry_run)?;
            match action {
                TaskAction::Start { description } => {
                    intent::start_task(&git_root, &description, &current_branch)?;
                    println!("{}", format!("Task started: \"{}\"", description).green());
                    println!(
                        "{}",
                        "Use 'tbdflow +' or 'tbdflow note' to log your thoughts as you work."
                            .dimmed()
                    );
                }
                TaskAction::Show => {
                    intent::show_intent_log(&git_root, Some(&current_branch))?;
                }
                TaskAction::Clear => {
                    intent::cleanup_intent_log(&git_root)?;
                    println!("{}", "Intent log cleared.".green());
                }
            }
        }
        Commands::Review {
            sha,
            trigger,
            digest,
            approve,
            concern,
            dismiss,
            message,
            since,
            reviewers,
        } => {
            if let Some(commit_hash) = approve {
                review::handle_review_approve(&config, &commit_hash, verbose, dry_run)?;
            } else if let Some(commit_hash) = concern {
                let msg = message.ok_or_else(|| {
                    anyhow::anyhow!("--message is required when raising a concern")
                })?;
                review::handle_review_concern(&config, &commit_hash, &msg, verbose, dry_run)?;
            } else if let Some(commit_hash) = dismiss {
                let msg = message.ok_or_else(|| {
                    anyhow::anyhow!("--message is required when dismissing a review")
                })?;
                review::handle_review_dismiss(&config, &commit_hash, &msg, verbose, dry_run)?;
            } else if digest {
                review::handle_review_digest(&config, &since, verbose, dry_run)?;
            } else if let Some(commit_sha) = sha {
                review::handle_review_trigger(
                    &config,
                    reviewers,
                    Some(commit_sha.as_str()),
                    verbose,
                    dry_run,
                )?;
            } else if trigger {
                review::handle_review_trigger(&config, reviewers, None, verbose, dry_run)?;
            } else {
                review::handle_review_digest(&config, &since, verbose, dry_run)?;
            }
        }
    }

    Ok(())
}
