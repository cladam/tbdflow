use anyhow::{Context, Result};
use chrono::Utc;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const INTENT_FILE: &str = ".tbdflow-intent.json";

/// Appends `.tbdflow-intent.json` to `.gitignore` if not already present.
fn ensure_gitignored(git_root: &Path) -> Result<()> {
    let gitignore_path = git_root.join(".gitignore");
    let entry = INTENT_FILE;

    if gitignore_path.exists() {
        let content = fs::read_to_string(&gitignore_path)
            .with_context(|| format!("Failed to read {}", gitignore_path.display()))?;
        // Already present — nothing to do.
        if content.lines().any(|line| line.trim() == entry) {
            return Ok(());
        }
    }

    // Append the entry (with a leading newline to avoid joining with the last line).
    use std::io::Write;
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&gitignore_path)
        .with_context(|| format!("Failed to open {}", gitignore_path.display()))?;
    writeln!(file, "\n# tbdflow intent log (local-only, never committed)")?;
    writeln!(file, "{}", entry)?;
    Ok(())
}

/// A single intent note captured during development.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentNote {
    pub message: String,
    pub timestamp: String,
}

/// The full intent log stored in `.tbdflow-intent.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentLog {
    /// Optional task description set by `tbdflow task start`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<String>,
    /// The branch that was active when the intent log was created.
    /// Used to detect stale logs after a branch switch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// Timestamp when the intent log was first created.
    pub started_at: String,
    /// Ordered list of developer notes / breadcrumbs.
    pub notes: Vec<IntentNote>,
}

/// Result of a stale-branch check.
pub enum BranchCheck {
    /// Current branch matches the intent log — safe to proceed.
    Ok,
    /// The intent log belongs to a different branch.
    Stale {
        log_branch: String,
        current_branch: String,
    },
    /// No branch recorded in the log (legacy format) — safe to proceed.
    Unknown,
}

impl IntentLog {
    /// Creates a new empty intent log bound to a branch.
    fn new(task: Option<String>, branch: Option<String>) -> Self {
        Self {
            task,
            branch,
            started_at: Utc::now().to_rfc3339(),
            notes: Vec::new(),
        }
    }
}

/// Returns the path to the intent file in the repository root.
fn intent_file_path(git_root: &Path) -> PathBuf {
    git_root.join(INTENT_FILE)
}

/// Loads an existing intent log from disk, or returns `None` if it doesn't exist.
pub fn load_intent_log(git_root: &Path) -> Result<Option<IntentLog>> {
    let path = intent_file_path(git_root);
    if !path.exists() {
        return Ok(None);
    }
    let content =
        fs::read_to_string(&path).with_context(|| format!("Failed to read {}", path.display()))?;
    let log: IntentLog = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse {}", path.display()))?;
    Ok(Some(log))
}

/// Saves the intent log to disk.
fn save_intent_log(git_root: &Path, log: &IntentLog) -> Result<()> {
    // Guard: make sure the intent file won't be picked up by `git add .`.
    ensure_gitignored(git_root)?;

    let path = intent_file_path(git_root);
    let json = serde_json::to_string_pretty(log).context("Failed to serialize intent log")?;
    fs::write(&path, json).with_context(|| format!("Failed to write {}", path.display()))?;
    Ok(())
}

/// Deletes the intent log file. Called after a successful push to trunk.
pub fn cleanup_intent_log(git_root: &Path) -> Result<()> {
    let path = intent_file_path(git_root);
    if path.exists() {
        fs::remove_file(&path).with_context(|| format!("Failed to remove {}", path.display()))?;
    }
    Ok(())
}

/// Checks whether the intent log belongs to the current branch.
pub fn check_branch(log: &IntentLog, current_branch: &str) -> BranchCheck {
    match &log.branch {
        Some(b) if b == current_branch => BranchCheck::Ok,
        Some(b) => BranchCheck::Stale {
            log_branch: b.clone(),
            current_branch: current_branch.to_string(),
        },
        None => BranchCheck::Unknown,
    }
}

/// Prints a warning about a stale intent log and returns true if the user
/// should be blocked (i.e. they need to resolve it first).
pub fn warn_stale(log_branch: &str, current_branch: &str) {
    println!(
        "{}",
        format!(
            "Stale intent log detected: notes were captured on '{}', but you are now on '{}'.",
            log_branch, current_branch
        )
        .yellow()
    );
    println!(
        "{}",
        "Use 'tbdflow task clear' to discard, or switch back to the original branch.".yellow()
    );
}

/// Appends a note to the intent log. Creates the file if it doesn't exist.
/// `current_branch` is stamped on creation so stale logs can be detected later.
pub fn add_note(git_root: &Path, message: &str, current_branch: &str) -> Result<()> {
    let mut log = load_intent_log(git_root)?
        .unwrap_or_else(|| IntentLog::new(None, Some(current_branch.to_string())));

    // Warn if the log belongs to a different branch, but still allow appending.
    if let BranchCheck::Stale {
        log_branch,
        current_branch,
    } = check_branch(&log, current_branch)
    {
        warn_stale(&log_branch, &current_branch);
    }

    log.notes.push(IntentNote {
        message: message.to_string(),
        timestamp: Utc::now().to_rfc3339(),
    });

    save_intent_log(git_root, &log)?;
    Ok(())
}

/// Starts a new task, creating a fresh intent log (or updating the task name
/// on an existing one).
/// `current_branch` is stamped on creation so stale logs can be detected later.
pub fn start_task(git_root: &Path, description: &str, current_branch: &str) -> Result<()> {
    let existing = load_intent_log(git_root)?;
    if let Some(existing_log) = &existing {
        if !existing_log.notes.is_empty() {
            // Check for stale log from a different branch
            if let BranchCheck::Stale {
                log_branch,
                current_branch: cur,
            } = check_branch(existing_log, current_branch)
            {
                warn_stale(&log_branch, &cur);
            }
            println!(
                "{}",
                format!(
                    "Warning: Existing intent log has {} note(s). They will be preserved.",
                    existing_log.notes.len()
                )
                .yellow()
            );
        }
    }

    let mut log =
        existing.unwrap_or_else(|| IntentLog::new(None, Some(current_branch.to_string())));
    log.task = Some(description.to_string());
    // Update the branch to the current one when starting a new task
    log.branch = Some(current_branch.to_string());

    save_intent_log(git_root, &log)?;
    Ok(())
}

/// Formats the intent log for inclusion in a commit message body.
/// Returns `None` if there are no notes to include.
pub fn format_for_commit(log: &IntentLog) -> Option<String> {
    if log.notes.is_empty() {
        return None;
    }

    let mut lines = Vec::new();
    lines.push("Intent Log:".to_string());
    for note in &log.notes {
        lines.push(format!("- {}", note.message));
    }

    Some(lines.join("\n"))
}

/// Prints the current intent log status to stdout.
pub fn show_intent_log(git_root: &Path, current_branch: Option<&str>) -> Result<()> {
    match load_intent_log(git_root)? {
        Some(log) => {
            // Stale-branch warning
            if let Some(branch) = current_branch {
                if let BranchCheck::Stale {
                    log_branch,
                    current_branch: cur,
                } = check_branch(&log, branch)
                {
                    warn_stale(&log_branch, &cur);
                }
            }
            if let Some(task) = &log.task {
                println!("{} {}", "Task:".blue().bold(), task);
            }
            if let Some(branch) = &log.branch {
                println!("{} {}", "Branch:".blue().bold(), branch);
            }
            println!("{} {}", "Started:".blue().bold(), log.started_at);
            if log.notes.is_empty() {
                println!("{}", "No notes recorded yet.".dimmed());
            } else {
                println!("{} {} note(s)", "Notes:".blue().bold(), log.notes.len());
                for (i, note) in log.notes.iter().enumerate() {
                    println!(
                        "  {}. {} {}",
                        i + 1,
                        note.message,
                        format!("({})", &note.timestamp[..19]).dimmed()
                    );
                }
            }
        }
        None => {
            println!(
                "{}",
                "No active intent log. Use 'tbdflow task start' or 'tbdflow +' to begin.".dimmed()
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup() -> TempDir {
        tempfile::tempdir().unwrap()
    }

    #[test]
    fn load_returns_none_when_no_file() {
        let dir = setup();
        let result = load_intent_log(dir.path()).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn add_note_creates_file_and_appends() {
        let dir = setup();
        add_note(dir.path(), "first note", "feat/auth").unwrap();
        add_note(dir.path(), "second note", "feat/auth").unwrap();

        let log = load_intent_log(dir.path()).unwrap().unwrap();
        assert_eq!(log.notes.len(), 2);
        assert_eq!(log.notes[0].message, "first note");
        assert_eq!(log.notes[1].message, "second note");
        assert!(log.task.is_none());
        assert_eq!(log.branch.as_deref(), Some("feat/auth"));
    }

    #[test]
    fn start_task_sets_task_name() {
        let dir = setup();
        start_task(dir.path(), "refactor auth", "feat/auth").unwrap();

        let log = load_intent_log(dir.path()).unwrap().unwrap();
        assert_eq!(log.task.as_deref(), Some("refactor auth"));
        assert_eq!(log.branch.as_deref(), Some("feat/auth"));
        assert!(log.notes.is_empty());
    }

    #[test]
    fn start_task_preserves_existing_notes() {
        let dir = setup();
        add_note(dir.path(), "early thought", "feat/auth").unwrap();
        start_task(dir.path(), "new task", "feat/auth").unwrap();

        let log = load_intent_log(dir.path()).unwrap().unwrap();
        assert_eq!(log.task.as_deref(), Some("new task"));
        assert_eq!(log.notes.len(), 1);
        assert_eq!(log.notes[0].message, "early thought");
    }

    #[test]
    fn cleanup_removes_file() {
        let dir = setup();
        add_note(dir.path(), "note", "main").unwrap();
        assert!(intent_file_path(dir.path()).exists());

        cleanup_intent_log(dir.path()).unwrap();
        assert!(!intent_file_path(dir.path()).exists());
    }

    #[test]
    fn cleanup_is_idempotent() {
        let dir = setup();
        cleanup_intent_log(dir.path()).unwrap(); // no file to remove
    }

    #[test]
    fn format_for_commit_returns_none_when_empty() {
        let log = IntentLog::new(None, None);
        assert!(format_for_commit(&log).is_none());
    }

    #[test]
    fn format_for_commit_produces_expected_output() {
        let mut log = IntentLog::new(
            Some("auth refactor".to_string()),
            Some("feat/auth".to_string()),
        );
        log.notes.push(IntentNote {
            message: "tried factory pattern".to_string(),
            timestamp: Utc::now().to_rfc3339(),
        });
        log.notes.push(IntentNote {
            message: "switching to traits".to_string(),
            timestamp: Utc::now().to_rfc3339(),
        });

        let formatted = format_for_commit(&log).unwrap();
        assert!(formatted.starts_with("Intent Log:"));
        assert!(formatted.contains("- tried factory pattern"));
        assert!(formatted.contains("- switching to traits"));
    }

    #[test]
    fn check_branch_returns_ok_when_matching() {
        let log = IntentLog::new(None, Some("feat/auth".to_string()));
        assert!(matches!(check_branch(&log, "feat/auth"), BranchCheck::Ok));
    }

    #[test]
    fn check_branch_returns_stale_when_mismatched() {
        let log = IntentLog::new(None, Some("feat/auth".to_string()));
        assert!(matches!(
            check_branch(&log, "fix/login"),
            BranchCheck::Stale { .. }
        ));
    }

    #[test]
    fn check_branch_returns_unknown_when_no_branch() {
        let log = IntentLog::new(None, None);
        assert!(matches!(check_branch(&log, "main"), BranchCheck::Unknown));
    }

    #[test]
    fn legacy_log_without_branch_field_still_loads() {
        let dir = setup();
        let legacy_json = r#"{
            "task": "old task",
            "started_at": "2026-01-01T00:00:00+00:00",
            "notes": [{"message": "old note", "timestamp": "2026-01-01T00:00:00+00:00"}]
        }"#;
        fs::write(intent_file_path(dir.path()), legacy_json).unwrap();

        let log = load_intent_log(dir.path()).unwrap().unwrap();
        assert!(log.branch.is_none());
        assert_eq!(log.notes.len(), 1);
    }

    #[test]
    fn ensure_gitignored_creates_file_when_missing() {
        let dir = setup();
        let gitignore = dir.path().join(".gitignore");
        assert!(!gitignore.exists());

        ensure_gitignored(dir.path()).unwrap();

        let content = fs::read_to_string(&gitignore).unwrap();
        assert!(content.contains(INTENT_FILE));
    }

    #[test]
    fn ensure_gitignored_appends_when_entry_absent() {
        let dir = setup();
        let gitignore = dir.path().join(".gitignore");
        fs::write(&gitignore, "target/\n").unwrap();

        ensure_gitignored(dir.path()).unwrap();

        let content = fs::read_to_string(&gitignore).unwrap();
        assert!(content.starts_with("target/\n"));
        assert!(content.contains(INTENT_FILE));
    }

    #[test]
    fn ensure_gitignored_is_idempotent() {
        let dir = setup();
        let gitignore = dir.path().join(".gitignore");
        fs::write(&gitignore, format!("{}\n", INTENT_FILE)).unwrap();

        ensure_gitignored(dir.path()).unwrap();

        let content = fs::read_to_string(&gitignore).unwrap();
        // Should appear exactly once.
        assert_eq!(
            content.matches(INTENT_FILE).count(),
            1,
            "entry duplicated in .gitignore"
        );
    }

    #[test]
    fn add_note_ensures_gitignore_entry() {
        let dir = setup();
        add_note(dir.path(), "design note", "feat/x").unwrap();

        let gitignore = dir.path().join(".gitignore");
        let content = fs::read_to_string(&gitignore).unwrap();
        assert!(
            content.contains(INTENT_FILE),
            ".gitignore should contain intent file entry after first breadcrumb"
        );
    }
}
