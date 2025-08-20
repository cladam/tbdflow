use crate::git;
use anyhow::Result;
use colored::*;
use git_conventional::Commit;
use std::collections::HashMap;

/// Returns the section header based on the commit type.
fn get_section_header(commit_type: &str) -> &'static str {
    match commit_type {
        "feat" => "### ✨ Features",
        "fix" => "### 🐛 Bug Fixes",
        "build" | "chore" | "ci" | "docs" | "style" | "refactor" | "test" => "### ⚙️ Maintenance",
        _ => "###  miscellaneous",
    }
}

pub fn handle_changelog(
    verbose: bool,
    from: Option<String>,
    to: Option<String>,
    unreleased: bool,
) -> Result<String> {
    // Range from last tag to HEAD if unreleased
    let range = if unreleased {
        let latest_tag = git::get_latest_tag(verbose)?;
        format!("{}..HEAD", latest_tag)
    } else {
        // Get the range from the specified 'from' commit to 'to' commit
        format!("{}..{}", from.unwrap_or_default(), to.unwrap_or("HEAD".to_string()))
    };

    // Fetch the commit history in a friendly format
    let history = git::get_commit_history(&range, verbose)?;
    let mut sections: HashMap<&'static str, Vec<String>> = HashMap::new();
    let remote_url = git::get_remote_url(verbose).unwrap_or_default();

    // Parse each line of the commit history
    // Expected format: "hash|message"
    for line in history.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 2 {
            continue;
        }
        let hash = parts[0];
        let message = parts[1];

        // Parse the commit message using git_conventional
        // This will extract the type, scope, and description
        if let Ok(commit) = Commit::parse(message) {
            let section_header = get_section_header(commit.type_().as_str());
            let scope = commit.scope().map_or("".to_string(), |s| format!("**({}):** ", s));
            let short_hash = &hash[..7];
            let commit_link = if !remote_url.is_empty() {
                format!("[`{}`]({}/commit/{})", short_hash, remote_url, hash)
            } else {
                format!("`{}`", short_hash)
            };

            let entry = format!("- {}{}{}", scope, commit.description(), commit_link);
            sections.entry(section_header).or_default().push(entry);
        }
    }

    let mut changelog = String::new();
    let section_order = [
        "### ✨ Features",
        "### 🐛 Bug Fixes",
        "### ⚙️ Maintenance",
        "### miscellaneous",
    ];

    // Build the changelog by iterating over the sections in a defined order
    for section in &section_order {
        if let Some(items) = sections.get(section) {
            changelog.push_str(&format!("\n{}\n", section.bold()));
            for item in items {
                changelog.push_str(&format!("{}\n", item));
            }
        }
    }

    Ok(changelog)
}