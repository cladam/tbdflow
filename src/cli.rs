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
/// The main CLI structure for tbdflow.
/// This struct defines the main commands and global options for the CLI tool.
pub struct Cli {
    /// The main command to run, which can be one of the subcommands defined in `Commands`.
    #[command(subcommand)]
    pub command: Commands,
    /// Enable verbose output for debugging. Use this to troubleshoot issues or understand the flow better.
    #[arg(long)]
    pub verbose: bool,
    /// Enable dry run mode. This will simulate the command without making any changes.
    #[arg(long)]
    pub dry_run: bool,
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
    /// Checks for a new version of tbdflow and updates it if available.
    Update,
    /// Commits changes to the current branch or 'main' if no branch is checked out.
    #[command(
        after_help = "Use the imperative, present tense: \"change\" not \"changed\". Think of This commit will...\n\
    COMMON COMMIT TYPES:\n  \
    feat:     A new feature for the user.\n  \
    fix:      A bug fix for the user.\n  \
    chore:    Routine tasks, maintenance, dependency updates.\n  \
    docs:     Documentation changes.\n  \
    style:    Code style changes (formatting, etc).\n  \
    refactor: Code changes that neither fix a bug nor add a feature.\n  \
    test:     Adding or improving tests.\n\n\
    EXAMPLES:\n  \
    tbdflow commit --type feat --scope api -m \"add user endpoint\"\n  \
    tbdflow commit -t fix -m \"fix login bug\" --breaking\n  \
    tbdflow commit -t chore -m \"update dependencies\" --tag \"v0.4.0\"\n  \
    tbdflow commit -t refactor -m \"rename internal API\" --breaking --breaking-description \"The `getUser` function has been renamed to `fetchUser`.\"\n  \
    tbdflow commit -t fix -s ui -m \"fix button alignment\" --issue \"#123\""
    )]
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
        #[arg(long)]
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
        #[arg(long, default_value_t = false, hide = true)]
        /// Internal flag to do a global commit bypassing monorepo safety
        include_projects: bool,
    },
    /// Creates and pushes a new short-lived branch.
    #[command(after_help = "EXAMPLES:\n  \
    tbdflow branch --type feat --name \"user-profile-page\" --issue \"ABC-123\"\n  \
    tbdflow branch -t fix -n \"login-bug\" --issue \"CBA-456\n  \
    tbdflow branch -t chore -n \"update-dependencies\" -f \"39b68b5\"")]
    Branch {
        /// Type of branch (e.g., feat, fix, chore). See .tbdflow.yml for allowed types.
        #[arg(short, long)]
        r#type: String,
        /// A short, descriptive name for the branch.
        #[arg(short, long)]
        name: String,
        /// Optional issue reference to include in the branch name.
        #[arg(long)]
        issue: Option<String>,
        /// Optional commit hash on 'main' to branch from.
        #[arg(short, long)]
        from_commit: Option<String>,
    },
    /// Merges a short-lived branch into 'main' and deletes it.
    #[command(after_help = "EXAMPLES:\n  \
    tbdflow complete --type \"feature\" --name \"user-profile-page\"\n  \
    tbdflow complete -t \"release\" -n \"1.2.0\"")]
    Complete {
        /// Type of branch to complete, see .tbdflow.yml for allowed types.
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
    /// Generates a changelog from Conventional Commits.
    #[command(
        name = "changelog",
        after_help = "EXAMPLES:\n  \
    tbdflow changelog --from v1.0.0 --to v2.0.0\n  \
    tbdflow changelog --unreleased\n  \
    tbdflow changelog --from v1.0.0"
    )]
    Changelog {
        /// Generate from this git reference (tag or commit hash).
        #[arg(long)]
        from: Option<String>,
        /// Generate to this git reference (defaults to HEAD).
        #[arg(long)]
        to: Option<String>,
        /// Generate for all commits since the latest tag.
        #[arg(long, default_value_t = false)]
        unreleased: bool,
    },
    /// Internal commands for configuration.
    #[command(name = "config", hide = true)]
    Config {
        /// Print the DoD checklist items to stdout.
        #[arg(long)]
        get_dod: bool,
    },
}
