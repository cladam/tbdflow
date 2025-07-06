// This file is part of tbdflow, a CLI tool for Trunk-Based Development workflows.
// It provides commands to initialize, show, and run operations in the context of tbdflow.
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Creates a new short-lived feature branch from 'main'.
    Feature {
        /// Name of the feature (e.g., 'add-login-page').
        #[arg(short, long)]
        name: String,
    },
    /// Creates a new short-lived release branch from 'main'.
    Release {
        /// Version for the release branch (e.g. '1.0.0').
        #[arg(short, long)]
        version: String,
        /// Optional commit hash on 'main' to branch from.
        #[arg(short, long)]
        from_commit: Option<String>,
    },
    /// Creates a new short-lived hotfix branch from 'main'.
    Hotfix {
        /// Name of the hotfix (e.g. 'fix-critical-bug').
        #[arg(short, long)]
        name: String,
    },
    /// Commits directly to the 'main' branch.
    Commit {
        /// Commit type (e.g. 'feat', 'fix', 'chore').
        #[arg(short, long)]
        r#type: String,
        /// Optional scope of the commit.
        #[arg(short, long)]
        scope: Option<String>,
        /// The commit message description.
        #[arg(short, long)]
        message: String,
        /// Mark this commit as a breaking change.
        #[arg(short, long)]
        breaking: bool,
    },
    /// Merges a short-lived branch into 'main' and deletes it.
    Complete {
        /// Type of branch to complete ('feature', 'release', 'hotfix').
        #[arg(short, long)]
        r#type: String,
        /// Name/version of the branch to complete.
        #[arg(short, long)]
        name: String,
    },
    /// Shows the current git status.
    Status,
    /// Shows the current git branch name.
    #[command(name = "current-branch")]
    CurrentBranch,
}