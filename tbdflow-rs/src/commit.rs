use crate::config::DodConfig;
use anyhow::Result;
use dialoguer::{MultiSelect, Confirm, theme::ColorfulTheme};
use crate::config;

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
        //.filter_map(|(i, item)| if !checked_indices.contains(&i) { Some(item.clone()) } else { None })
        .collect();
    if unchecked_items.is_empty() {
        String::new()
    } else {
        format!("\n\nTODO:\n{}", unchecked_items.join("\n"))
    }
}

/// Handles the interactive commit process, including checklist confirmation and issue reference handling.
pub fn handle_interactive_commit(config: &DodConfig, base_message: &str) -> Result<Option<String>, anyhow::Error> {
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

/// Check if the subject line of the commit message is valid.
/// Validations include maximum length, capitalization, and period at the end.
pub fn is_valid_subject_line(subject: &str, config: &config::Config) -> Result<(), String> {
    if let Some(lint) = &config.lint {
        if let Some(rules) = &lint.subject_line_rules {
            if let Some(max_len) = rules.subject_line_max_length {
                if subject.len() > max_len {
                    return Err(format!("Subject line exceeds maximum length of {} characters.", max_len));
                }
            }
            if let Some(enforce_not_capitalized) = rules.subject_line_not_capitalized {
                if enforce_not_capitalized {
                    if let Some(first) = subject.chars().next() {
                        if first.is_uppercase() {
                            return Err("Subject line must not start with a capital letter.".to_string());
                        }
                    }
                }
            }
            if let Some(no_period) = rules.subject_line_no_period {
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
            if let Some(max_len) = rules.body_max_line_length {
                for line in body.lines() {
                    if line.len() > max_len { return false; }
                }
            }
        }
    }
    true
}
