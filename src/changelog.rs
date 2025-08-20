use crate::{config::Config, git};
use anyhow::Result;
use colored::*;
use git_conventional::Commit;
use std::collections::HashMap;

/// Returns the section header based on the commit type.
fn get_section_header(commit_type: &str) -> &'static str {
    match commit_type {
        "feat" => "### ✨ Features",
        "fix" => "### 🐛 Bug Fixes",
        "perf" => "### 🚀 Performance Improvements",
        "refactor" => "### 🔨 Code Refactoring",
        "build" | "chore" | "ci" | "docs" | "style" | "test" => "### ⚙️ Maintenance",
        _ => "### Miscellaneous",
    }
}

pub fn handle_changelog(
    verbose: bool,
    config: &Config,
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
        format!("{}..{}", from.unwrap_or_default(), to.clone().unwrap_or("HEAD".to_string()))
    };

    // Fetch the commit history in a friendly format
    let history = git::get_commit_history(&range, verbose)?;
    let mut sections: HashMap<&'static str, Vec<String>> = HashMap::new();
    let mut breaking_changes: Vec<String> = Vec::new();
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
            let scope = commit.scope().map_or("".to_string(), |s| format!("**({}):** ", s));
            let short_hash = &hash[..7];
            let commit_link = if !remote_url.is_empty() {
                format!(" [`{}`]({}/commit/{})", short_hash, remote_url, hash)
            } else {
                format!("`{}`", short_hash)
            };

            let entry = format!("- {}{}{}", scope, commit.description(), commit_link);

            if commit.breaking() {
                breaking_changes.push(entry.clone());
            }

            let section_header = get_section_header(commit.type_().as_str());
            sections.entry(section_header).or_default().push(entry);
        }
    }

    let mut changelog = String::new();

    // Add the version header
    if unreleased {
        changelog.push_str("# Unreleased Changes\n");
    } else {
        if let Some(tag) = &to {
            let version = tag.strip_prefix('v').unwrap_or(tag);
            let date = chrono::Local::now().format("%Y-%m-%d").to_string();

            let release_link = if let Some(template) = &config.release_url_template {
                let url = template.replace("{{version}}", tag);
                format!("[{}]({})", version, url)
            } else {
                version.to_string()
            };
            changelog.push_str(&format!("# {} ({})\n", release_link, date));
        }
    }

    let section_order = [
        "### ⚠️ BREAKING CHANGES",
        "### ✨ Features",
        "### 🐛 Bug Fixes",
        "### 🚀 Performance Improvements",
        "### 🔨 Code Refactoring",
        "### ⚙️ Maintenance",
        "### Miscellaneous",
    ];

    for section in &section_order {
        let items = if *section == "### ⚠️ BREAKING CHANGES" {
            Some(&breaking_changes)
        } else {
            sections.get(section)
        };

        if let Some(items) = items {
            if !items.is_empty() {
                changelog.push_str(&format!("\n{}\n", section.bold()));
                for item in items {
                    changelog.push_str(&format!("{}\n", item));
                }
            }
        }
    }

    Ok(changelog)
}