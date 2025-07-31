use crate::config::DodConfig;
use anyhow::Result;
use colored::Colorize;
use dialoguer::{MultiSelect, Confirm, theme::ColorfulTheme};

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
pub fn handle_interactive_commit(config: &DodConfig, base_message: &str, issue: &Option<String>) -> Result<Option<String>, anyhow::Error> {
    // Start with the base commit message.
    let mut commit_message = base_message.to_string();

    if config.issue_reference_required.unwrap_or(false) && issue.is_none() {
        println!("{}", "Issue reference is required for commits, see .dod.yml file.".red());
        return Err(anyhow::anyhow!("Aborted: Issue reference required."));
    }

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

    // Append the issue reference as a trailer/footer if required.
    if config.issue_reference_required.unwrap_or(false) {
        if let Some(issue_ref) = issue {
            commit_message.push_str(&format!("\n\nRefs: {}", issue_ref));
        }
    }

    Ok(Some(commit_message))
}

/// Check if the TYPE in the commit message is valid.
pub fn is_valid_commit_type(commit_type: &str) -> bool {
    matches!(
        commit_type,
        "feat" | "fix" | "docs" | "style" | "refactor" | "perf" | "test" | "chore"
        | "build" | "ci" | "revert" | "wip"
    )
}
