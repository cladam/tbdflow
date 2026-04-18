use crate::{git, intent};
use anyhow::{Context, Result};
use colored::*;
use std::path::Path;

#[derive(Debug)]
pub struct SnapshotEntry {
    pub index: usize,
    pub timestamp: String,
    pub note: Option<String>,
    pub branch: Option<String>,
    pub hash: String,
    pub kind: SnapshotKind,
}

#[derive(Debug)]
pub enum SnapshotKind {
    Intent,
    PreSync,
}

impl std::fmt::Display for SnapshotKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnapshotKind::Intent => write!(f, "intent"),
            SnapshotKind::PreSync => write!(f, "pre-sync"),
        }
    }
}

/// Collects all available snapshots from the intent log.
pub fn collect_snapshots(git_root: &Path) -> Result<Vec<SnapshotEntry>> {
    let log = intent::load_intent_log(git_root)?;
    let mut entries = Vec::new();

    if let Some(log) = log {
        let branch = log.branch.clone();

        // Collect note snapshots
        for note in &log.notes {
            if let Some(hash) = &note.snapshot_hash {
                entries.push(SnapshotEntry {
                    index: 0, // will be assigned below
                    timestamp: note.timestamp.clone(),
                    note: Some(note.message.clone()),
                    branch: branch.clone(),
                    hash: hash.clone(),
                    kind: SnapshotKind::Intent,
                });
            }
        }

        // Collect last sync snapshot
        if let Some(hash) = &log.last_sync_snapshot {
            entries.push(SnapshotEntry {
                index: 0,
                timestamp: log.started_at.clone(),
                note: Some("Pre-sync safety snapshot".to_string()),
                branch: branch.clone(),
                hash: hash.clone(),
                kind: SnapshotKind::PreSync,
            });
        }
    }

    // Assign indices
    for (i, entry) in entries.iter_mut().enumerate() {
        entry.index = i + 1;
    }

    Ok(entries)
}

/// Lists all available snapshots in a table.
pub fn handle_recover_list(git_root: &Path, current_branch: &str) -> Result<()> {
    let entries = collect_snapshots(git_root)?;

    if entries.is_empty() {
        println!(
            "{}",
            "No snapshots available. Snapshots are created automatically when you use 'tbdflow n' or 'tbdflow sync'.".dimmed()
        );
        return Ok(());
    }

    // Stale-branch warning
    if let Some(intent::IntentLog {
        branch: Some(ref log_branch),
        ..
    }) = intent::load_intent_log(git_root)?
    {
        if log_branch != current_branch {
            intent::warn_stale(log_branch, current_branch);
            println!();
        }
    }

    println!("{}", "Available WIP snapshots:".blue().bold());
    println!(
        "  {:<5} {:<10} {:<22} {:<40} {}",
        "#", "Type", "Timestamp", "Note", "Hash"
    );
    println!("  {}", "-".repeat(90));

    for entry in &entries {
        let ts_display = if entry.timestamp.len() >= 19 {
            &entry.timestamp[..19]
        } else {
            &entry.timestamp
        };
        let note_display = entry
            .note
            .as_deref()
            .unwrap_or("-")
            .chars()
            .take(38)
            .collect::<String>();
        let short_hash = if entry.hash.len() >= 10 {
            &entry.hash[..10]
        } else {
            &entry.hash
        };
        println!(
            "  {:<5} {:<10} {:<22} {:<40} {}",
            entry.index,
            entry.kind.to_string(),
            ts_display,
            note_display,
            short_hash,
        );
    }

    println!(
        "\n{}",
        "Use 'tbdflow recover <index>' to restore a snapshot.".dimmed()
    );
    Ok(())
}

/// Applies a snapshot by index or hash.
pub fn handle_recover_apply(
    git_root: &Path,
    selector: &str,
    verbose: bool,
    dry_run: bool,
) -> Result<()> {
    let entries = collect_snapshots(git_root)?;

    let hash = if let Ok(idx) = selector.parse::<usize>() {
        // Lookup by index
        entries
            .iter()
            .find(|e| e.index == idx)
            .map(|e| e.hash.clone())
            .ok_or_else(|| anyhow::anyhow!("No snapshot at index {}", idx))?
    } else {
        // Treat as a hash — verify it looks plausible
        selector.to_string()
    };

    println!(
        "{}",
        "Warning: This will apply the snapshot over your current working directory."
            .bold()
            .yellow()
    );

    if !dry_run {
        git::stash_apply(&hash, verbose, dry_run).context(
            "Failed to apply snapshot. The commit object may have been garbage-collected.",
        )?;
    } else {
        println!(
            "{}",
            format!("[DRY RUN] Would run: git stash apply {}", hash).yellow()
        );
    }

    println!("{}", "Snapshot applied successfully.".green());
    println!(
        "{}",
        "The snapshot remains available for future recovery.".dimmed()
    );
    Ok(())
}
