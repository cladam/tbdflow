use crate::config::{Config, DodConfig};
use crate::{config, git};
use anyhow::Result;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect};
use std::path::PathBuf;

/// Runs the checklist interactively, allowing the user to confirm each item before committing.
pub fn run_checklist_interactive(checklist: &[String]) -> anyhow::Result<Vec<usize>> {
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Please confirm each item before committing:")
        .items(checklist)
        .interact()?;
    Ok(selections)
}

/// Builds the TODO footer for the commit message based on unchecked items in the checklist.
pub fn build_todo_footer(checklist: &[String], checked_indices: &[usize]) -> String {
    //let checked_indices: Vec<usize> = checked_indices.iter().cloned().collect();
    let unchecked_items: Vec<String> = checklist
        .iter()
        .enumerate()
        .filter(|(i, _)| !checked_indices.contains(&i))
        .map(|(_, item)| format!("- [ ] {}", item))
        .collect();
    if unchecked_items.is_empty() {
        String::new()
    } else {
        format!("\n\nTODO:\n{}", unchecked_items.join("\n"))
    }
}

/// Handles the interactive commit process, including checklist confirmation and issue reference handling.
pub fn handle_interactive_commit(
    config: &DodConfig,
    base_message: &str,
) -> Result<Option<String>, anyhow::Error> {
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

    Ok(Some(commit_message))
}

/// Runs the interactive DoD check.
/// Returns Ok(Some(footer)) on success.
/// Returns Ok(None) if the user aborts.
/// Returns Err if something goes wrong.
pub fn handle_interactive_dod(config: &DodConfig) -> Result<Option<String>> {
    let checked = run_checklist_interactive(&config.checklist)?;
    if checked.len() != config.checklist.len() {
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Warning: Not all DoD items were checked. Proceed by adding a 'TODO' list to the commit message?")
            .interact()?
        {
            let todo_footer = build_todo_footer(&config.checklist, &checked);
            Ok(Some(todo_footer))
        } else {
            println!("Commit aborted.");
            Ok(None)
        }
    } else {
        Ok(Some(String::new())) // All items checked, return an empty footer.
    }
}

/// Check if the TYPE in the commit message is valid.
pub fn is_valid_commit_type(commit_type: &str, config: &config::Config) -> bool {
    if let Some(lint_config) = &config.lint {
        if let Some(conventional_commit_type) = &lint_config.conventional_commit_type {
            if let Some(enabled) = conventional_commit_type.enabled {
                if !enabled {
                    return true; // If linting is disabled, any type is valid
                }
            }
            if let Some(allowed_types) = &conventional_commit_type.allowed_types {
                return allowed_types.iter().any(|t| t == commit_type);
            }
        }
    }
    true
}

/// Check if the issue key in the commit message is valid.
pub fn is_valid_issue_key(issue_key: &Option<String>, config: &config::Config) -> bool {
    if let Some(lint_config) = &config.lint {
        if let Some(issue_key_config) = &lint_config.issue_key_missing {
            if let Some(enabled) = issue_key_config.enabled {
                if !enabled {
                    return true; // If linting is disabled, any issue key is valid
                }
            }
            if let Some(issue_key_pattern) = &issue_key_config.pattern {
                let re = regex::Regex::new(issue_key_pattern).unwrap();
                return re.is_match(&issue_key.as_ref().unwrap_or(&"".to_string()));
            }
        }
    }
    true
}

pub fn is_valid_scope(scope: &Option<String>, config: &config::Config) -> bool {
    if let Some(lint_config) = &config.lint {
        if let Some(scope_config) = &lint_config.scope {
            if let Some(enabled) = scope_config.enabled {
                if !enabled {
                    return true; // If linting is disabled, any scope is valid
                }
            }
            if let Some(enforce_lowercase) = scope_config.enforce_lowercase {
                if enforce_lowercase {
                    if let Some(s) = scope {
                        return s.chars().all(|c| c.is_lowercase());
                    }
                }
            }
        }
    }
    true
}

/// Check if the subject line of the commit message is valid.
/// Validations include maximum length, capitalization, and period at the end.
pub fn is_valid_subject_line(subject: &str, config: &config::Config) -> Result<(), String> {
    if let Some(lint) = &config.lint {
        if let Some(rules) = &lint.subject_line_rules {
            if let Some(max_len) = rules.max_length {
                if subject.len() > max_len {
                    return Err(format!(
                        "Subject line exceeds maximum length of {} characters.",
                        max_len
                    ));
                }
            }
            if let Some(enforce_lowercase) = rules.enforce_lowercase {
                if enforce_lowercase {
                    if let Some(first) = subject.chars().next() {
                        if first.is_uppercase() {
                            return Err(
                                "Subject line must not start with a capital letter.".to_string()
                            );
                        }
                    }
                }
            }
            if let Some(no_period) = rules.no_period {
                if no_period && subject.trim_end().ends_with('.') {
                    return Err("Subject line should not end with a period.".to_string());
                }
            }
        }
    }
    Ok(())
}

/// Check if the body lines of the commit message are valid.
/// Validations include maximum line length.
pub fn is_valid_body_lines(body: &str, config: &config::Config) -> bool {
    if let Some(lint) = &config.lint {
        if let Some(rules) = &lint.body_line_rules {
            if let Some(max_len) = rules.max_line_length {
                for line in body.lines() {
                    if line.len() > max_len {
                        return false;
                    }
                }
            }
            // Enforced in code already, but can be uncommented later on
            // if let Some(leading_blank) = rules.leading_blank {
            //     if leading_blank && !body.starts_with("\n\n") {
            //         return false; // Body must start with a leading blank line
            //     }
            // }
        }
    }
    true
}

pub fn handle_commit(
    verbose: bool,
    dry_run: bool,
    config: &Config,
    r#type: String,
    scope: Option<String>,
    message: String,
    body: Option<String>,
    breaking: bool,
    breaking_description: Option<String>,
    tag: Option<String>,
    no_verify: bool,
    issue: Option<String>,
) -> Result<()> {
    println!("{}", "--- Committing changes ---".blue());

    // Check for conflicting flags based on issue handling strategy
    if config.issue_handling.strategy == config::IssueHandlingStrategy::CommitScope {
        if scope.is_some() && issue.is_some() {
            println!(
                "{}",
                "Error: Cannot use both --scope and --issue when the 'commit-scope' strategy is active.".red()
            );
            println!(
                "{}",
                "Hint: To associate this commit with the issue, please provide only the --issue flag.".yellow()
            );
            return Err(anyhow::anyhow!(
                "Aborted: Conflicting flags for commit-scope strategy."
            ));
        }
    }

    // Linting based on the provided configuration
    if !is_valid_commit_type(&r#type, config) {
        println!(
            "{}",
            format!(
                "Error: '{}' is not a valid Conventional Commit type.",
                r#type
            )
            .red()
        );
        return Err(anyhow::anyhow!("Aborted: Invalid commit type."));
    }

    if !is_valid_issue_key(&issue, config) {
        println!(
            "{}",
            "Issue reference is required by your .tbdflow.yml config.".red()
        );
        return Err(anyhow::anyhow!("Aborted: Issue reference required."));
    }

    if let Err(e) = is_valid_subject_line(&message, config) {
        println!("{}", format!("Commit message subject error: {}", e).red());
        return Err(anyhow::anyhow!("Aborted: Invalid commit message subject."));
    }

    if let Some(body_text) = &body {
        if !is_valid_body_lines(body_text, config) {
            println!(
                "{}",
                "Commit message body contains lines that exceed the maximum length.".red()
            );
            return Err(anyhow::anyhow!("Aborted: Invalid commit message body."));
        }
    }

    if let Some(s) = &scope {
        if !is_valid_scope(&Some(s.clone()), config) {
            println!("{}", "Scope must be lowercase.".red());
            return Err(anyhow::anyhow!("Aborted: Invalid commit scope."));
        }
    }

    let scope_part = scope.map_or("".to_string(), |s| format!("({})", s));
    let breaking_part = if breaking { "!" } else { "" };
    let header = format!("{}{}{}: {}", r#type, scope_part, breaking_part, message);

    let dod_config = config::load_dod_config().unwrap_or_default();
    let todo_footer_result = if no_verify || dod_config.checklist.is_empty() {
        Ok(Some(String::new()))
    } else {
        handle_interactive_dod(&dod_config)
    };

    if let Some(todo_footer) = todo_footer_result? {
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
        commit_message.push_str(&todo_footer);

        println!(
            "{}",
            format!("Commit message will be:\n---\n{}\n---", commit_message).blue()
        );

        let git_root = PathBuf::from(git::get_git_root(verbose, dry_run)?);
        let current_dir = std::env::current_dir()?;
        // debug print
        if verbose {
            println!("Git root: {:?}", git_root);
            println!("Current dir: {:?}", current_dir);
            println!("monorepo: {:?}", config.monorepo);
        }
        if config::is_monorepo_root(&config, &current_dir, &git_root) {
            // We are at the root of a configured monorepo.
            // Exclude project directories from the commit.
            println!(
                "{}",
                "Monorepo root detected. Staging root-level files only.".yellow()
            );
            git::add_excluding_projects(&config.monorepo.project_dirs, verbose, dry_run)?;
        } else {
            // We are in a sub-project or not in a monorepo.
            // Add all changes from the current directory.
            git::add_all(verbose, dry_run)?;
        }

        if !git::has_staged_changes(verbose, dry_run)? {
            println!("{}", "No changes added to commit.".yellow());
            return Ok(());
        }

        let current_branch = git::get_current_branch(verbose, dry_run)?;
        if current_branch == config.main_branch_name {
            println!("--- Committing directly to main branch ---");
            git::pull_latest_with_rebase(verbose, dry_run)?;
            git::commit(&commit_message, verbose, dry_run)?;
            git::push(verbose, dry_run)?;
            println!(
                "\n{}",
                "Successfully committed and pushed changes to main.".green()
            );
        } else {
            println!("--- Committing to feature branch '{}' ---", current_branch);
            git::commit(&commit_message, verbose, dry_run)?;
            git::push(verbose, dry_run)?;
            println!(
                "\n{}",
                format!("Successfully pushed changes to '{}'.", current_branch).green()
            );
        }

        if let Some(tag_name) = tag {
            let commit_hash = git::get_head_commit_hash(verbose, dry_run)?;
            git::create_tag(&tag_name, &commit_message, &commit_hash, verbose, dry_run)?;
            git::push_tags(verbose, dry_run)?;
            println!(
                "{}",
                format!("Success! Created and pushed tag '{}'", tag_name).green()
            );
        }
    }
    Ok(())
}
