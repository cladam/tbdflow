use crate::config::{Config, DodConfig};
use crate::{config, git, intent, radar, review};
use anyhow::Result;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect};
use std::path::PathBuf;

pub struct CommitParams {
    pub r#type: String,
    pub scope: Option<String>,
    pub message: String,
    pub body: Option<String>,
    pub breaking: bool,
    pub breaking_description: Option<String>,
    pub tag: Option<String>,
    pub issue: Option<String>,
    pub include_projects: bool,
    pub no_verify: bool,
}

pub fn run_checklist_interactive(checklist: &[String]) -> Result<Vec<usize>> {
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Please confirm each item before committing:")
        .items(checklist)
        .interact()?;
    Ok(selections)
}

pub fn build_todo_footer(checklist: &[String], checked_indices: &[usize]) -> String {
    let unchecked_items: Vec<String> = checklist
        .iter()
        .enumerate()
        .filter(|(i, _)| !checked_indices.contains(i))
        .map(|(_, item)| format!("- [ ] {}", item))
        .collect();
    if unchecked_items.is_empty() {
        String::new()
    } else {
        format!("\n\nTODO:\n{}", unchecked_items.join("\n"))
    }
}

pub fn handle_interactive_commit(
    config: &DodConfig,
    base_message: &str,
) -> Result<Option<String>, anyhow::Error> {
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

pub fn is_valid_commit_type(commit_type: &str, config: &Config) -> bool {
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

pub fn is_valid_issue_key(issue_key: &Option<String>, config: &Config) -> Result<bool> {
    if let Some(lint_config) = &config.lint {
        if let Some(issue_key_config) = &lint_config.issue_key_missing {
            if let Some(enabled) = issue_key_config.enabled {
                if !enabled {
                    return Ok(true); // If linting is disabled, any issue key is valid
                }
            }
            if let Some(issue_key_pattern) = &issue_key_config.pattern {
                let re = regex::Regex::new(issue_key_pattern).map_err(|e| {
                    anyhow::anyhow!("Invalid issue_key pattern '{}': {}", issue_key_pattern, e)
                })?;
                return Ok(re.is_match(issue_key.as_ref().unwrap_or(&"".to_string())));
            }
        }
    }
    Ok(true)
}

pub fn is_valid_scope(scope: &Option<String>, config: &Config) -> bool {
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

pub fn is_valid_subject_line(subject: &str, config: &Config) -> Result<(), String> {
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

pub fn is_valid_body_lines(body: &str, config: &Config) -> bool {
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
    params: CommitParams,
) -> Result<()> {
    println!("{}", "--- Committing changes ---".blue());

    // Check for conflicting flags based on issue handling strategy
    if config.issue_handling.strategy == config::IssueHandlingStrategy::CommitScope
        && params.scope.is_some()
        && params.issue.is_some()
    {
        println!(
                "{}",
                "Error: Cannot use both --scope and --issue when the 'commit-scope' strategy is active.".red()
            );
        println!(
            "{}",
            "Hint: To associate this commit with the issue, please provide only the --issue flag."
                .yellow()
        );
        return Err(anyhow::anyhow!(
            "Aborted: Conflicting flags for commit-scope strategy."
        ));
    }

    // Linting based on the provided configuration
    if !is_valid_commit_type(&params.r#type, config) {
        println!(
            "{}",
            format!(
                "Error: '{}' is not a valid Conventional Commit type.",
                params.r#type
            )
            .red()
        );
        return Err(anyhow::anyhow!("Aborted: Invalid commit type."));
    }

    if !is_valid_issue_key(&params.issue, config)? {
        println!(
            "{}",
            "Issue reference is required by your .tbdflow.yml config.".red()
        );
        return Err(anyhow::anyhow!("Aborted: Issue reference required."));
    }

    if let Err(e) = is_valid_subject_line(&params.message, config) {
        println!("{}", format!("Commit message subject error: {}", e).red());
        return Err(anyhow::anyhow!("Aborted: Invalid commit message subject."));
    }

    if let Some(body_text) = &params.body {
        if !is_valid_body_lines(body_text, config) {
            println!(
                "{}",
                "Commit message body contains lines that exceed the maximum length.".red()
            );
            return Err(anyhow::anyhow!("Aborted: Invalid commit message body."));
        }
    }

    if let Some(s) = &params.scope {
        if !is_valid_scope(&Some(s.clone()), config) {
            println!("{}", "Scope must be lowercase.".red());
            return Err(anyhow::anyhow!("Aborted: Invalid commit scope."));
        }
    }

    let scope_part = params.scope.map_or("".to_string(), |s| format!("({})", s));
    let breaking_part = if params.breaking { "!" } else { "" };
    let header = format!(
        "{}{}{}: {}",
        params.r#type, scope_part, breaking_part, params.message
    );

    let dod_config = config::load_dod_config().unwrap_or_default();
    let todo_footer_result = if params.no_verify || dod_config.checklist.is_empty() {
        Ok(Some(String::new()))
    } else {
        handle_interactive_dod(&dod_config)
    };

    if let Some(todo_footer) = todo_footer_result? {
        let git_root = PathBuf::from(git::get_git_root(verbose, dry_run)?);

        // Read the intent log (if any) for inclusion in the commit body.
        let intent_log = intent::load_intent_log(&git_root)?;
        let intent_section = intent_log
            .as_ref()
            .and_then(|log| intent::format_for_commit(log));

        let mut commit_message = header;
        if let Some(body_text) = params.body {
            commit_message.push_str("\n\n");
            commit_message.push_str(&body_text);
        }
        // Append the Intent Log section (before breaking change / refs / TODO)
        if let Some(intent_text) = &intent_section {
            commit_message.push_str("\n\n");
            commit_message.push_str(intent_text);
        }
        if let Some(desc) = params.breaking_description {
            commit_message.push_str(&format!("\n\nBREAKING CHANGE: {}", desc));
        }
        if let Some(issue_ref) = &params.issue {
            commit_message.push_str(&format!("\n\nRefs: {}", issue_ref));
        }
        commit_message.push_str(&todo_footer);

        println!(
            "{}",
            format!("Commit message will be:\n---\n{}\n---", commit_message).blue()
        );

        if verbose {
            let current_dir = std::env::current_dir()?;
            println!("Git root: {:?}", git_root);
            println!("Current dir: {:?}", current_dir);
            println!("monorepo: {:?}", config.monorepo);
        }
        git::stage_scoped_changes(config, params.include_projects, verbose, dry_run)?;

        if !git::has_staged_changes(verbose, dry_run)? {
            println!("{}", "No changes added to commit.".yellow());
            return Ok(());
        }

        // Radar: check for overlapping work before committing
        if !radar::check_before_commit(config, verbose, dry_run)? {
            println!("{}", "Commit aborted by user.".yellow());
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

            // Cleanup the intent log after successful push to trunk
            if intent_section.is_some() {
                intent::cleanup_intent_log(&git_root)?;
                println!("{}", "Intent log consumed and cleared.".dimmed());
            }

            // Auto-trigger review if rules match the changed files
            let commit_hash = git::get_head_commit_hash(verbose, dry_run)?;
            if review::should_auto_trigger_review(config, &commit_hash, verbose, dry_run)? {
                let author = git::get_user_name(verbose, dry_run)?;
                review::trigger_review(
                    config,
                    None,
                    &commit_hash,
                    &commit_message,
                    &author,
                    verbose,
                    dry_run,
                )?;
            }
        } else {
            println!("--- Committing to feature branch '{}' ---", current_branch);
            git::commit(&commit_message, verbose, dry_run)?;
            git::push(verbose, dry_run)?;
            println!(
                "\n{}",
                format!("Successfully pushed changes to '{}'.", current_branch).green()
            );
        }

        if let Some(tag_name) = params.tag {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::*;

    fn config_with_defaults() -> Config {
        Config::default()
    }

    fn config_without_lint() -> Config {
        Config {
            lint: None,
            ..Default::default()
        }
    }

    fn config_with_allowed_types(types: Vec<&str>) -> Config {
        Config {
            lint: Some(LintConfig {
                conventional_commit_type: Some(ConventionalCommitTypeConfig {
                    enabled: Some(true),
                    allowed_types: Some(types.iter().map(|s| s.to_string()).collect()),
                }),
                ..config_with_defaults().lint.unwrap()
            }),
            ..Default::default()
        }
    }

    #[test]
    fn commit_type_accepts_allowed_type() {
        let config = config_with_defaults();
        assert!(is_valid_commit_type("feat", &config));
        assert!(is_valid_commit_type("fix", &config));
        assert!(is_valid_commit_type("chore", &config));
    }

    #[test]
    fn commit_type_rejects_unknown_type() {
        let config = config_with_defaults();
        assert!(!is_valid_commit_type("yolo", &config));
        assert!(!is_valid_commit_type("", &config));
    }

    #[test]
    fn commit_type_accepts_anything_when_lint_disabled() {
        let config = config_without_lint();
        assert!(is_valid_commit_type("anything", &config));
    }

    #[test]
    fn commit_type_accepts_anything_when_rule_disabled() {
        let config = Config {
            lint: Some(LintConfig {
                conventional_commit_type: Some(ConventionalCommitTypeConfig {
                    enabled: Some(false),
                    allowed_types: Some(vec!["feat".to_string()]),
                }),
                ..config_with_defaults().lint.unwrap()
            }),
            ..Default::default()
        };
        assert!(is_valid_commit_type("yolo", &config));
    }

    #[test]
    fn commit_type_respects_custom_allowed_list() {
        let config = config_with_allowed_types(vec!["feat", "hotfix"]);
        assert!(is_valid_commit_type("feat", &config));
        assert!(is_valid_commit_type("hotfix", &config));
        assert!(!is_valid_commit_type("fix", &config));
    }

    #[test]
    fn scope_accepts_lowercase() {
        let config = config_with_defaults();
        assert!(is_valid_scope(&Some("api".to_string()), &config));
    }

    #[test]
    fn scope_rejects_uppercase_when_enforced() {
        let config = config_with_defaults();
        assert!(!is_valid_scope(&Some("API".to_string()), &config));
        assert!(!is_valid_scope(&Some("Api".to_string()), &config));
    }

    #[test]
    fn scope_accepts_none() {
        let config = config_with_defaults();
        assert!(is_valid_scope(&None, &config));
    }

    #[test]
    fn scope_accepts_anything_when_lint_disabled() {
        let config = config_without_lint();
        assert!(is_valid_scope(&Some("UPPER".to_string()), &config));
    }

    #[test]
    fn subject_accepts_valid_message() {
        let config = config_with_defaults();
        assert!(is_valid_subject_line("add user endpoint", &config).is_ok());
    }

    #[test]
    fn subject_rejects_too_long() {
        let config = config_with_defaults();
        let long_subject = "a".repeat(73);
        assert!(is_valid_subject_line(&long_subject, &config).is_err());
    }

    #[test]
    fn subject_accepts_exactly_max_length() {
        let config = config_with_defaults();
        let exact = "a".repeat(72);
        assert!(is_valid_subject_line(&exact, &config).is_ok());
    }

    #[test]
    fn subject_rejects_uppercase_start() {
        let config = config_with_defaults();
        assert!(is_valid_subject_line("Add user endpoint", &config).is_err());
    }

    #[test]
    fn subject_rejects_trailing_period() {
        let config = config_with_defaults();
        assert!(is_valid_subject_line("add user endpoint.", &config).is_err());
    }

    #[test]
    fn subject_accepts_anything_when_lint_disabled() {
        let config = config_without_lint();
        assert!(is_valid_subject_line("Whatever. YOLO.", &config).is_ok());
    }

    #[test]
    fn body_accepts_short_lines() {
        let config = config_with_defaults();
        assert!(is_valid_body_lines(
            "short line\nanother short line",
            &config
        ));
    }

    #[test]
    fn body_rejects_line_exceeding_max_length() {
        let config = config_with_defaults();
        let long_line = "x".repeat(81);
        assert!(!is_valid_body_lines(&long_line, &config));
    }

    #[test]
    fn body_accepts_exactly_max_length() {
        let config = config_with_defaults();
        let exact = "x".repeat(80);
        assert!(is_valid_body_lines(&exact, &config));
    }

    #[test]
    fn body_rejects_if_any_line_too_long() {
        let config = config_with_defaults();
        let body = format!("short\n{}\nshort", "x".repeat(81));
        assert!(!is_valid_body_lines(&body, &config));
    }

    #[test]
    fn body_accepts_anything_when_lint_disabled() {
        let config = config_without_lint();
        let long = "x".repeat(200);
        assert!(is_valid_body_lines(&long, &config));
    }

    #[test]
    fn issue_key_accepts_valid_key() {
        let config = Config {
            lint: Some(LintConfig {
                issue_key_missing: Some(IssueKeyConfig {
                    enabled: Some(true),
                    pattern: Some(r"^[A-Z]+-\d+$".to_string()),
                }),
                ..config_with_defaults().lint.unwrap()
            }),
            ..Default::default()
        };
        assert!(is_valid_issue_key(&Some("PROJ-123".to_string()), &config).unwrap());
    }

    #[test]
    fn issue_key_rejects_invalid_key() {
        let config = Config {
            lint: Some(LintConfig {
                issue_key_missing: Some(IssueKeyConfig {
                    enabled: Some(true),
                    pattern: Some(r"^[A-Z]+-\d+$".to_string()),
                }),
                ..config_with_defaults().lint.unwrap()
            }),
            ..Default::default()
        };
        assert!(!is_valid_issue_key(&Some("bad".to_string()), &config).unwrap());
    }

    #[test]
    fn issue_key_rejects_none_when_required() {
        let config = Config {
            lint: Some(LintConfig {
                issue_key_missing: Some(IssueKeyConfig {
                    enabled: Some(true),
                    pattern: Some(r"^[A-Z]+-\d+$".to_string()),
                }),
                ..config_with_defaults().lint.unwrap()
            }),
            ..Default::default()
        };
        assert!(!is_valid_issue_key(&None, &config).unwrap());
    }

    #[test]
    fn issue_key_accepts_anything_when_disabled() {
        // Default config has issue_key enabled: false
        let config = config_with_defaults();
        assert!(is_valid_issue_key(&None, &config).unwrap());
        assert!(is_valid_issue_key(&Some("whatever".to_string()), &config).unwrap());
    }

    #[test]
    fn issue_key_returns_error_on_invalid_regex() {
        let config = Config {
            lint: Some(LintConfig {
                issue_key_missing: Some(IssueKeyConfig {
                    enabled: Some(true),
                    pattern: Some(r"[unclosed".to_string()),
                }),
                ..config_with_defaults().lint.unwrap()
            }),
            ..Default::default()
        };
        assert!(is_valid_issue_key(&Some("PROJ-1".to_string()), &config).is_err());
    }

    #[test]
    fn todo_footer_empty_when_all_checked() {
        let checklist = vec!["item1".to_string(), "item2".to_string()];
        assert_eq!(build_todo_footer(&checklist, &[0, 1]), "");
    }

    #[test]
    fn todo_footer_lists_unchecked_items() {
        let checklist = vec![
            "first".to_string(),
            "second".to_string(),
            "third".to_string(),
        ];
        let footer = build_todo_footer(&checklist, &[1]);
        assert!(footer.contains("- [ ] first"));
        assert!(!footer.contains("- [ ] second"));
        assert!(footer.contains("- [ ] third"));
    }

    #[test]
    fn todo_footer_lists_all_when_none_checked() {
        let checklist = vec!["a".to_string(), "b".to_string()];
        let footer = build_todo_footer(&checklist, &[]);
        assert!(footer.contains("- [ ] a"));
        assert!(footer.contains("- [ ] b"));
        assert!(footer.starts_with("\n\nTODO:\n"));
    }
}
