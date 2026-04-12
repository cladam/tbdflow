use crate::config::{Config, RadarLevel, RadarOnCommit};
use crate::git;
use anyhow::Result;
use colored::*;
use std::collections::HashSet;

#[derive(Debug)]
pub enum OverlapKind {
    SameFile,
    LineOverlap {
        my_lines: Vec<git::HunkRange>,
        their_lines: Vec<git::HunkRange>,
    },
}

#[derive(Debug)]
pub struct FileOverlap {
    pub file_path: String,
    pub overlap_kind: OverlapKind,
}

#[derive(Debug)]
pub struct BranchOverlap {
    pub branch_name: String,
    pub author: String,
    pub commits_ahead: u32,
    pub overlapping_files: Vec<FileOverlap>,
}

#[derive(Debug)]
pub struct RadarResult {
    pub overlaps: Vec<BranchOverlap>,
    pub branches_scanned: usize,
    pub local_files_count: usize,
}

/// Run the full radar scan: fetch, compare local changes against all active remote branches.
pub fn scan(config: &Config, verbose: bool, dry_run: bool) -> Result<RadarResult> {
    let main_branch = &config.main_branch_name;

    if verbose {
        println!("{}", "[RADAR] Fetching latest from origin...".dimmed());
    }
    git::fetch_origin(verbose, dry_run)?;

    let local_files = git::get_local_changed_files(verbose, dry_run)?;
    if local_files.is_empty() {
        return Ok(RadarResult {
            overlaps: vec![],
            branches_scanned: 0,
            local_files_count: 0,
        });
    }
    let local_file_set: HashSet<&str> = local_files.iter().map(|s| s.as_str()).collect();

    let active_branches = git::get_active_remote_branches(main_branch, verbose, dry_run)?;

    let current_branch = git::get_current_branch(verbose, dry_run).unwrap_or_default();
    let branches_to_scan: Vec<&String> = active_branches
        .iter()
        .filter(|b| b.as_str() != current_branch)
        .collect();

    let branches_scanned = branches_to_scan.len();
    let level = &config.radar.level;
    let ignore_patterns = &config.radar.ignore_patterns;

    let mut overlaps = Vec::new();
    let main_ref = format!("origin/{}", main_branch);

    for branch in &branches_to_scan {
        let branch_ref = format!("origin/{}", branch);

        // Get files changed by this branch relative to main
        let branch_files =
            match git::get_diff_files_between_refs(&main_ref, &branch_ref, verbose, dry_run) {
                Ok(files) => files,
                Err(_) => continue, // Skip branches that can't be diffed (e.g. orphan)
            };

        // Find intersection (file-level overlap)
        let overlapping_files: Vec<&String> = branch_files
            .iter()
            .filter(|f| local_file_set.contains(f.as_str()))
            .filter(|f| !should_ignore(f, ignore_patterns))
            .collect();

        if overlapping_files.is_empty() {
            continue;
        }

        // Get branch metadata
        let author = git::get_branch_author(branch, verbose, dry_run)
            .unwrap_or_else(|_| "unknown".to_string());
        let commits_ahead =
            git::get_remote_branch_commit_count(branch, main_branch, verbose, dry_run).unwrap_or(0);

        // Build file overlaps with appropriate detail level
        let mut file_overlaps = Vec::new();
        for file in &overlapping_files {
            let overlap_kind = match level {
                RadarLevel::Line => {
                    // Level 2: line-level overlap detection
                    match detect_line_overlap(file, &main_ref, &branch_ref, verbose, dry_run) {
                        Some(kind) => kind,
                        None => OverlapKind::SameFile, // Fall back to file-level if hunks can't be parsed
                    }
                }
                RadarLevel::File => OverlapKind::SameFile,
            };

            file_overlaps.push(FileOverlap {
                file_path: file.to_string(),
                overlap_kind,
            });
        }

        overlaps.push(BranchOverlap {
            branch_name: branch.to_string(),
            author,
            commits_ahead,
            overlapping_files: file_overlaps,
        });
    }

    Ok(RadarResult {
        overlaps,
        branches_scanned,
        local_files_count: local_files.len(),
    })
}

fn detect_line_overlap(
    file: &str,
    main_ref: &str,
    branch_ref: &str,
    verbose: bool,
    dry_run: bool,
) -> Option<OverlapKind> {
    let my_hunks = git::get_local_diff_hunks(file, verbose, dry_run).ok()?;
    let their_hunks =
        git::get_diff_hunks_between_refs(main_ref, branch_ref, file, verbose, dry_run).ok()?;

    if my_hunks.is_empty() || their_hunks.is_empty() {
        return None;
    }

    // Check if any hunk pairs overlap
    let has_overlap = my_hunks
        .iter()
        .any(|mine| their_hunks.iter().any(|theirs| mine.overlaps(theirs)));

    if has_overlap {
        Some(OverlapKind::LineOverlap {
            my_lines: my_hunks,
            their_lines: their_hunks,
        })
    } else {
        // Same file but different line ranges — still report as SameFile
        Some(OverlapKind::SameFile)
    }
}

fn should_ignore(file: &str, patterns: &[String]) -> bool {
    for pattern in patterns {
        if let Ok(glob_pattern) = glob::Pattern::new(pattern) {
            if glob_pattern.matches(file) {
                return true;
            }
        }
    }
    false
}

pub fn handle_radar(verbose: bool, dry_run: bool, config: &Config) -> Result<()> {
    println!(
        "{}",
        "--- Scanning for overlapping work ---".to_string().blue()
    );

    if !config.radar.enabled {
        println!(
            "{}",
            "Radar is disabled. Enable it in .tbdflow.yml with:\n\n  radar:\n    enabled: true"
                .yellow()
        );
        return Ok(());
    }

    println!("Fetching latest from origin...");
    let result = scan(config, verbose, dry_run)?;

    if result.local_files_count == 0 {
        println!("{}", "No local changes detected. Nothing to scan.".green());
        return Ok(());
    }

    println!(
        "Scanned {} active branch(es) against {} local file(s).\n",
        result.branches_scanned, result.local_files_count
    );

    if result.overlaps.is_empty() {
        println!(
            "{}",
            format!(
                "✅ No overlaps detected across {} active branch(es). You're clear!",
                result.branches_scanned
            )
            .green()
        );
    } else {
        println!(
            "{}",
            format!(
                "OVERLAP DETECTED with {} active branch(es):\n",
                result.overlaps.len()
            )
            .yellow()
            .bold()
        );

        for overlap in &result.overlaps {
            print_branch_overlap(overlap);
        }

        let clean_count = result.branches_scanned - result.overlaps.len();
        if clean_count > 0 {
            println!(
                "{}",
                format!(
                    "  ✅ {} other active branch(es) have no overlap with your changes.",
                    clean_count
                )
                .green()
            );
        }

        println!(
            "\n{}",
            "Hint: Coordinate with the overlapping author(s) before pushing. Consider syncing more frequently."
                .dimmed()
        );
    }

    Ok(())
}

/// Print a single branch overlap in a tree-like format.
fn print_branch_overlap(overlap: &BranchOverlap) {
    println!(
        "  {} (by {}, {} commit(s) ahead)",
        overlap.branch_name.bold(),
        format!("@{}", overlap.author).cyan(),
        overlap.commits_ahead
    );

    let file_count = overlap.overlapping_files.len();
    for (i, file_overlap) in overlap.overlapping_files.iter().enumerate() {
        let connector = if i == file_count - 1 {
            "└──"
        } else {
            "├──"
        };
        let indicator = match &file_overlap.overlap_kind {
            OverlapKind::LineOverlap { .. } => "⚡ LINE OVERLAP".red().bold().to_string(),
            OverlapKind::SameFile => "📁 SAME FILE".yellow().to_string(),
        };

        let detail = match &file_overlap.overlap_kind {
            OverlapKind::LineOverlap {
                my_lines,
                their_lines,
            } => {
                let my_ranges = format_hunk_ranges(my_lines);
                let their_ranges = format_hunk_ranges(their_lines);
                format!("  you: lines {}, them: lines {}", my_ranges, their_ranges)
            }
            OverlapKind::SameFile => String::new(),
        };

        println!(
            "  {} {}    {}{}",
            connector, file_overlap.file_path, indicator, detail
        );
    }
    println!();
}

/// Format hunk ranges into a human-readable string like "14-28, 42-50".
fn format_hunk_ranges(hunks: &[git::HunkRange]) -> String {
    hunks
        .iter()
        .map(|h| {
            if h.line_count <= 1 {
                format!("{}", h.start_line)
            } else {
                format!("{}-{}", h.start_line, h.start_line + h.line_count - 1)
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// Lightweight radar check intended for integration into `tbdflow sync`.
/// Returns a short summary string if overlaps are found, or None if clear.
pub fn quick_scan_for_sync(
    config: &Config,
    verbose: bool,
    dry_run: bool,
) -> Result<Option<String>> {
    if !config.radar.enabled || !config.radar.on_sync {
        return Ok(None);
    }

    let result = scan(config, verbose, dry_run)?;
    if result.overlaps.is_empty() || result.local_files_count == 0 {
        return Ok(None);
    }

    // Build a compact summary
    let mut lines = Vec::new();
    for overlap in &result.overlaps {
        for file_overlap in &overlap.overlapping_files {
            lines.push(format!(
                "  @{} is also modifying {} on {}",
                overlap.author, file_overlap.file_path, overlap.branch_name
            ));
        }
    }

    let summary = format!(
        "Radar: {}\n   Run 'tbdflow radar' for details.",
        lines.join("\n")
    );

    Ok(Some(summary))
}

/// Radar check for the commit workflow.
/// Returns true if the user should proceed, false if they chose to abort.
pub fn check_before_commit(config: &Config, verbose: bool, dry_run: bool) -> Result<bool> {
    if config.radar.on_commit == RadarOnCommit::Off {
        return Ok(true);
    }

    if !config.radar.enabled {
        return Ok(true);
    }

    let result = scan(config, verbose, dry_run)?;
    if result.overlaps.is_empty() || result.local_files_count == 0 {
        return Ok(true);
    }

    // Print warnings
    println!("\n{}", "Radar detected overlapping work:".yellow().bold());
    for overlap in &result.overlaps {
        for file_overlap in &overlap.overlapping_files {
            let indicator = match &file_overlap.overlap_kind {
                OverlapKind::LineOverlap { .. } => "⚡",
                OverlapKind::SameFile => "📁",
            };
            println!(
                "  {} {} — @{} on {}",
                indicator, file_overlap.file_path, overlap.author, overlap.branch_name
            );
        }
    }

    match config.radar.on_commit {
        RadarOnCommit::Warn => {
            println!("{}", "  Consider coordinating before pushing.\n".dimmed());
            Ok(true)
        }
        RadarOnCommit::Confirm => {
            let proceed =
                dialoguer::Confirm::with_theme(&dialoguer::theme::ColorfulTheme::default())
                    .with_prompt("Overlapping work detected. Continue with commit?")
                    .default(true)
                    .interact()?;
            Ok(proceed)
        }
        RadarOnCommit::Off => Ok(true),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git::HunkRange;

    #[test]
    fn test_hunk_overlap_detection() {
        let a = HunkRange {
            start_line: 10,
            line_count: 5,
        }; // lines 10-14
        let b = HunkRange {
            start_line: 12,
            line_count: 5,
        }; // lines 12-16
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));
    }

    #[test]
    fn test_hunk_no_overlap() {
        let a = HunkRange {
            start_line: 10,
            line_count: 5,
        }; // lines 10-14
        let b = HunkRange {
            start_line: 20,
            line_count: 5,
        }; // lines 20-24
        assert!(!a.overlaps(&b));
        assert!(!b.overlaps(&a));
    }

    #[test]
    fn test_hunk_adjacent_no_overlap() {
        let a = HunkRange {
            start_line: 10,
            line_count: 5,
        }; // lines 10-14
        let b = HunkRange {
            start_line: 15,
            line_count: 5,
        }; // lines 15-19
        assert!(!a.overlaps(&b));
    }

    #[test]
    fn test_hunk_single_line_overlap() {
        let a = HunkRange {
            start_line: 10,
            line_count: 1,
        }; // line 10
        let b = HunkRange {
            start_line: 10,
            line_count: 1,
        }; // line 10
        assert!(a.overlaps(&b));
    }

    #[test]
    fn test_hunk_contained() {
        let a = HunkRange {
            start_line: 5,
            line_count: 20,
        }; // lines 5-24
        let b = HunkRange {
            start_line: 10,
            line_count: 3,
        }; // lines 10-12
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));
    }

    #[test]
    fn test_should_ignore_patterns() {
        let patterns = vec![
            "*.lock".to_string(),
            "*.generated.*".to_string(),
            "CHANGELOG.md".to_string(),
        ];

        assert!(should_ignore("Cargo.lock", &patterns));
        assert!(!should_ignore("package-lock.json", &patterns)); // *.lock does not match .json extension
        assert!(should_ignore("CHANGELOG.md", &patterns));
        assert!(should_ignore("api.generated.rs", &patterns));
        assert!(!should_ignore("src/main.rs", &patterns));
        assert!(!should_ignore("README.md", &patterns));
    }

    #[test]
    fn test_format_hunk_ranges() {
        let hunks = vec![
            HunkRange {
                start_line: 14,
                line_count: 15,
            },
            HunkRange {
                start_line: 42,
                line_count: 1,
            },
        ];
        let formatted = format_hunk_ranges(&hunks);
        assert_eq!(formatted, "14-28, 42");
    }

    #[test]
    fn test_format_hunk_ranges_empty() {
        let hunks: Vec<HunkRange> = vec![];
        let formatted = format_hunk_ranges(&hunks);
        assert_eq!(formatted, "");
    }
}
