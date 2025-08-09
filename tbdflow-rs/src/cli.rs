// This file is part of tbdflow, a CLI tool for Trunk-Based Development workflows.
// It provides commands to initialise, show, and run operations in the context of tbdflow.
use clap::{Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser, Debug)]
#[command(
    name = "tbdflow",
    author = "Claes Adamsson @cladam",
    version,
    about = "A CLI tool for Trunk-Based Development (TBD) workflows",
    long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    /// Enable verbose output for debugging. Use this to troubleshoot issues or understand the flow better.
    #[arg(long)]
    pub verbose: bool,
}

/// Subcommands for the tbdflow CLI tool.
/// Each command corresponds to a specific operation in the Trunk-Based Development workflow.
/// The commands include initialising the repository, committing changes, creating feature/release/hotfix branches,
/// completing branches, syncing with the remote, checking status, and checking branches.
/// The commands are designed to streamline the development process and enforce best practices in trunk-based development.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialises the repository for Trunk-Based Development.
    Init,
    /// Commits changes to the current branch or 'main' if no branch is checked out.
    #[command(after_help = "Use the imperative, present tense: \"change\" not \"changed\". Think of This commit will...\n\
    COMMON COMMIT TYPES:\n  \
    feat:     A new feature for the user.\n  \
    fix:      A bug fix for the user.\n  \
    chore:    Routine tasks, maintenance, dependency updates.\n  \
    docs:     Documentation changes.\n  \
    style:    Code style changes (formatting, etc).\n  \
    refactor: Code changes that neither fix a bug nor add a feature.\n  \
    test:     Adding or improving tests.\n\n\
    EXAMPLES:\n  \
    tbdflow commit --type feat --scope api -m \"Add user endpoint\"\n  \
    tbdflow commit -t fix -m \"Fix login bug\" --breaking\n  \
    tbdflow commit -t chore -m \"Update dependencies\" --tag \"v0.4.0\"\n  \
    tbdflow commit -t refactor -m \"Rename internal API\" --breaking --breaking-description \"The `getUser` function has been renamed to `fetchUser`.\"\n  \
    tbdflow commit -t fix -s ui -m \"Fix button alignment\" --issue \"#123\"")]
    Commit {
        /// Commit type (e.g. 'feat', 'fix', 'chore', 'docs').
        #[arg(short, long)]
        r#type: String,
        /// Optional scope of the commit.
        #[arg(short, long)]
        scope: Option<String>,
        /// The descriptive commit message.
        #[arg(short, long)]
        message: String,
        /// Mark this commit as a breaking change.
        #[arg(short, long)]
        breaking: bool,
        /// Optionally provide a description for the breaking change.
        breaking_description: Option<String>,
        /// Optionally add and push an annotated tag to this commit.
        #[arg(long)]
        tag: Option<String>,
        /// Optional flag to skip verification of the checklist.
        #[arg(long, default_value_t = false)]
        no_verify: bool,
        /// Optional flag for an issue reference.
        #[arg(long)]
        issue: Option<String>,
        /// Optional multi-line body for the commit message.
        #[arg(long)]
        body: Option<String>,
    },
    /// Creates a new short-lived feature branch from 'main'.
    #[command(after_help = "EXAMPLE:\n  \
    tbdflow feature --name \"user-profile-page\"")]
    Feature {
        /// Name of the feature (e.g. 'user-profile-page').
        #[arg(short, long)]
        name: String,
    },
    /// Creates a new short-lived release branch from 'main'.
    #[command(after_help = "EXAMPLES:\n  \
    tbdflow release --version \"2.1.0\"\n  \
    tbdflow release -v \"2.1.0\" -f \"39b68b5\"", disable_version_flag = true)]
    Release {
        /// Version for the release branch (e.g. '1.0.0').
        #[arg(short, long)]
        version: String,
        /// Optional commit hash on 'main' to branch from.
        #[arg(short, long)]
        from_commit: Option<String>,
    },
    /// Creates a new short-lived hotfix branch from 'main'.
    #[command(after_help = "EXAMPLE:\n  \
    tbdflow hotfix --name \"critical-auth-bug\"")]
    Hotfix {
        /// Name of the hotfix (e.g. 'critical-auth-bug').
        #[arg(short, long)]
        name: String,
    },
    /// Merges a short-lived branch into 'main' and deletes it.
    #[command(after_help = "EXAMPLES:\n  \
    tbdflow complete --type \"feature\" --name \"user-profile-page\"\n  \
    tbdflow complete -t \"release\" -n \"1.2.0\"")]
    Complete {
        /// Type of branch to complete ('feature', 'release', 'hotfix').
        #[arg(short, long)]
        r#type: String,
        /// Name or version of the branch to complete.
        #[arg(short, long)]
        name: String,
    },
    /// Syncs with the remote, shows recent history, and checks for stale branches.
    Sync,
    /// Shows the current git status.
    Status,
    /// Shows the current git branch name.
    #[command(name = "current-branch")]
    CurrentBranch,
    /// Checks for stale branches (older than 1 day).
    #[command(name = "check-branches")]
    CheckBranches,
    /// Generates a man page for the CLI.
    #[command(name = "generate-man-page", hide = true)] // Hidden from help
    #[command(after_help = "EXAMPLES:\n  \
    tbdflow generate-man-page > tbdflow.1\n \
    man ./tbdflow.1")]
    GenerateManPage,
    /// Generates shell completion scripts.
    #[command(name = "generate-completion", hide = true)] // Hidden from help
    Completion {
        #[arg(value_enum)]
        shell: Shell,
    },
}