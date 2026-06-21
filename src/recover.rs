use crate::git::RunOpts;
use crate::{git, intent};
use anyhow::{Context, Result};
use colored::Colorize;
use dialoguer::{Confirm, theme::ColorfulTheme};
use std::path::Path;

/// A single recoverable snapshot entry.
#[derive(Debug)]
pub struct SnapshotEntry {
    pub index: usize,
    pub timestamp: String,
    pub note: String,
    pub hash: String,
}

/// Collects all available snapshots from the intent log.
pub fn collect_snapshots(
    git_root: &Path,
) -> Result<(Option<intent::IntentLog>, Vec<SnapshotEntry>)> {
    let log = intent::load_intent_log(git_root)?;
    let mut entries = Vec::new();

    if let Some(ref log) = log {
        // Collect note snapshots (intent, sync, undo, radar — all stored as notes)
        for note in &log.notes {
            if let Some(hash) = &note.snapshot_hash {
                entries.push(SnapshotEntry {
                    index: 0, // assigned below
                    timestamp: note.timestamp.clone(),
                    note: note.message.clone(),
                    hash: hash.clone(),
                });
            }
        }
    }

    // Assign 1-based indices
    for (i, entry) in entries.iter_mut().enumerate() {
        entry.index = i + 1;
    }

    Ok((log, entries))
}

pub fn handle_recover_list(git_root: &Path, current_branch: &str) -> Result<()> {
    let (log, entries) = collect_snapshots(git_root)?;

    if entries.is_empty() {
        println!(
            "{}",
            "No snapshots available. Snapshots are created automatically when you use 'tbdflow n' or 'tbdflow sync'."
                .dimmed()
        );
        return Ok(());
    }

    // Stale-branch warning (reuses already-loaded log — no second read)
    if let Some(intent::IntentLog {
        branch: Some(ref log_branch),
        ..
    }) = log
    {
        if log_branch != current_branch {
            intent::warn_stale(log_branch, current_branch);
            println!();
        }
    }

    println!("{}", "Available WIP snapshots:".blue().bold());
    println!("  {:<5} {:<22} {:<42} {}", "#", "Timestamp", "Note", "Hash");
    println!("  {}", "-".repeat(85));

    for entry in &entries {
        let ts_display = if entry.timestamp.len() >= 19 {
            &entry.timestamp[..19]
        } else {
            &entry.timestamp
        };
        let note_display: String = entry.note.chars().take(40).collect();
        let short_hash = if entry.hash.len() >= 10 {
            &entry.hash[..10]
        } else {
            &entry.hash
        };
        println!(
            "  {:<5} {:<22} {:<42} {}",
            entry.index, ts_display, note_display, short_hash,
        );
    }

    println!(
        "\n{}",
        "Use 'tbdflow recover <index>' to restore a snapshot.".dimmed()
    );
    Ok(())
}

/// Applies a snapshot by index or hash.
pub fn handle_recover_apply(git_root: &Path, selector: &str, opts: RunOpts) -> Result<()> {
    let (_log, entries) = collect_snapshots(git_root)?;

    let hash = if let Ok(idx) = selector.parse::<usize>() {
        entries
            .iter()
            .find(|e| e.index == idx)
            .map(|e| e.hash.clone())
            .ok_or_else(|| anyhow::anyhow!("No snapshot at index {}", idx))?
    } else {
        selector.to_string()
    };

    println!(
        "{}",
        "Warning: This will apply the snapshot over your current working directory."
            .bold()
            .yellow()
    );

    if opts.dry_run {
        println!(
            "{}",
            format!("[DRY RUN] Would run: git stash apply {}", hash).yellow()
        );
    } else {
        let confirmed = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Apply snapshot?")
            .default(true)
            .interact()?;

        if !confirmed {
            println!("{}", "Recover aborted.".yellow());
            return Ok(());
        }

        git::stash_apply(&hash, opts).context(
            "Failed to apply snapshot. The commit object may have been garbage-collected.",
        )?;
    }

    println!("{}", "Snapshot applied successfully.".green());
    println!(
        "{}",
        "The snapshot remains available for future recovery.".dimmed()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intent::{add_note, add_note_with_snapshot, record_safety_snapshot};

    fn setup() -> tempfile::TempDir {
        tempfile::tempdir().unwrap()
    }

    #[test]
    fn collect_snapshots_returns_empty_when_no_log() {
        let dir = setup();
        let (log, entries) = collect_snapshots(dir.path()).unwrap();
        assert!(log.is_none());
        assert!(entries.is_empty());
    }

    #[test]
    fn collect_snapshots_skips_notes_without_hashes() {
        let dir = setup();
        add_note(dir.path(), "plain note", "feat/x").unwrap();

        let (_log, entries) = collect_snapshots(dir.path()).unwrap();
        assert!(entries.is_empty());
    }

    #[test]
    fn collect_snapshots_includes_notes_with_hashes() {
        let dir = setup();
        add_note_with_snapshot(dir.path(), "snap note", "feat/x", Some("abc123".into())).unwrap();
        add_note(dir.path(), "plain note", "feat/x").unwrap();
        add_note_with_snapshot(dir.path(), "snap 2", "feat/x", Some("def456".into())).unwrap();

        let (_log, entries) = collect_snapshots(dir.path()).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].index, 1);
        assert_eq!(entries[0].hash, "abc123");
        assert_eq!(entries[0].note, "snap note");
        assert_eq!(entries[1].index, 2);
        assert_eq!(entries[1].hash, "def456");
    }

    #[test]
    fn collect_snapshots_includes_safety_snapshots() {
        let dir = setup();
        record_safety_snapshot(dir.path(), "sync1", "main", "Pre-sync safety snapshot").unwrap();
        record_safety_snapshot(dir.path(), "sync2", "main", "Pre-undo safety snapshot").unwrap();

        let (_log, entries) = collect_snapshots(dir.path()).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].hash, "sync1");
        assert_eq!(entries[1].hash, "sync2");
    }

    #[test]
    fn collect_snapshots_returns_log_for_reuse() {
        let dir = setup();
        add_note_with_snapshot(dir.path(), "note", "feat/x", Some("hash".into())).unwrap();

        let (log, _entries) = collect_snapshots(dir.path()).unwrap();
        assert!(log.is_some());
        assert_eq!(log.unwrap().branch.as_deref(), Some("feat/x"));
    }
}
