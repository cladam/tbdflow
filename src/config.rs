/// The structs and functions for handling the configuration of the TBDFlow tool.
/// This includes reading the configuration from `.tbdflow.yml` and `.dod.yml` files,
/// as well as defining the structure of the configuration data.
use serde::{Deserialize, Serialize};
use std::fs;
use anyhow::Context;

/// Represents the Definition of Done (DoD) configuration.
#[derive(Debug, Deserialize, Default)]
pub struct DodConfig {
    pub issue_reference_required: Option<bool>,
    #[serde(default)]
    pub checklist: Vec<String>,
}

/// The thre main prefixes for different types of branches in the project.
/// These prefixes are used to categorize branches as features, releases, or hotfixes.
#[derive(Debug, Serialize, Deserialize)]
pub struct BranchPrefixes {
    pub feature: String,
    pub release: String,
    pub hotfix: String,
}

/// Represents the automatic tagging configuration for releases and hotfixes.
#[derive(Debug, Serialize, Deserialize)]
pub struct AutomaticTags {
    pub release_prefix: String,
    pub hotfix_prefix: String,
}

/// Represents the configuration for linting commit messages.
/// This includes rules for conventional commit types, issue keys, subject line rules, and body line
#[derive(Debug, Serialize, Deserialize)]
pub struct ConventionalCommitTypeConfig {
    pub enabled: Option<bool>,
    pub allowed_types: Option<Vec<String>>,
}

/// Represents the configuration for issue keys in commit messages.
/// This includes whether the issue key linting is enabled and the pattern to match issue keys.
/// The pattern is typically a regex that matches issue keys in formats like "PROJECT-123".
#[derive(Debug, Serialize, Deserialize)]
pub struct IssueKeyConfig {
    pub enabled: Option<bool>,
    pub pattern: Option<String>,
}

/// Represents the rules for validating the subject line of commit messages.
/// This includes checks for maximum length, capitalization, and whether it ends with a period.
/// The subject line rules help ensure that commit messages are clear and follow a consistent format.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubjectLineRules {
    pub subject_line_max_length: Option<usize>,
    pub subject_line_not_capitalized: Option<bool>,
    pub subject_line_no_period: Option<bool>,
}

/// Represents the rules for validating the body lines of commit messages.
#[derive(Debug, Serialize, Deserialize)]
pub struct BodyLineRules {
    pub body_max_line_length: Option<usize>,
}

/// Represents the configuration for linting commit messages.
/// This includes configurations for conventional commit types, issue key validation,
/// subject line rules, and body line rules.
/// The lint configuration helps enforce best practices in commit messages,
#[derive(Debug, Serialize, Deserialize)]
pub struct LintConfig {
    pub conventional_commit_type: Option<ConventionalCommitTypeConfig>,
    pub issue_key_missing: Option<IssueKeyConfig>,
    pub subject_line_rules: Option<SubjectLineRules>,
    pub body_line_rules: Option<BodyLineRules>,
}

/// Represents the main configuration for the TBDFlow tool.
/// This includes the main branch name, stale branch threshold, branch prefixes,
/// automatic tagging configuration, and optional linting configuration.
/// The configuration is loaded from a YAML file and provides defaults for various settings.
/// The `Config` struct is used to manage the settings for the TBDFlow tool,
/// ensuring that the tool operates according to the project's requirements.
/// The configuration is typically loaded from a `.tbdflow.yml` file in the root of the git repository.
/// It includes settings for the main branch name, stale branch threshold, branch prefixes,
/// automatic tagging, and optional linting rules for commit messages.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub main_branch_name: String,
    pub stale_branch_threshold_days: i64,
    pub branch_prefixes: BranchPrefixes,
    pub automatic_tags: AutomaticTags,
    pub lint: Option<LintConfig>,
}

// Implementing Default for Config to provide default values for the configuration.
// This allows the application to use a default configuration if no `.tbdflow.yml` file
// is found or if the file is empty or malformed.
// The default values are set to ensure that the tool can operate with sensible defaults,
// even if the user has not provided a custom configuration file.
impl Default for Config {
    fn default() -> Self {
        Config {
            main_branch_name: "main".to_string(),
            stale_branch_threshold_days: 1,
            branch_prefixes: BranchPrefixes {
                feature: "feature_".to_string(),
                release: "release_".to_string(),
                hotfix: "hotfix_".to_string(),
            },
            automatic_tags: AutomaticTags {
                release_prefix: "v".to_string(),
                hotfix_prefix: "hotfix-tag_".to_string(),
            },
            // Add default lint configuration
            lint: Some(LintConfig {
                conventional_commit_type: Some(ConventionalCommitTypeConfig {
                    enabled: Some(true),
                    allowed_types: Some(vec![
                        "build".to_string(),
                        "chore".to_string(),
                        "ci".to_string(),
                        "docs".to_string(),
                        "feat".to_string(),
                        "fix".to_string(),
                        "perf".to_string(),
                        "refactor".to_string(),
                        "revert".to_string(),
                        "style".to_string(),
                        "test".to_string(),
                    ]),
                }),
                issue_key_missing: Some(IssueKeyConfig {
                    enabled: Some(false),
                    pattern: Some(r"^[A-Z]+-\d+$".to_string()), // Example pattern for Jira issue keys
                }),
                subject_line_rules: Some(SubjectLineRules {
                    subject_line_max_length: Some(72),
                    subject_line_not_capitalized: Some(true),
                    subject_line_no_period: Some(true),
                }),
                body_line_rules: Some(BodyLineRules {
                    body_max_line_length: Some(80),
                }),
            }),
        }
    }
}

/// Loads the configuration from the `.tbdflow.yml` file in the current directory (root of the git repository).
pub fn load_tbdflow_config() -> Result<Config, anyhow::Error> {
    // Attempt to read the configuration file
    if let Ok(content) = fs::read_to_string(".tbdflow.yml") {
        serde_yaml::from_str(&content).context("Failed to parse .tbdflow.yml")
    } else {
        Ok(Config::default())
    }
}

/// Reads the DoD configuration from the `.dod.yml` file in the current directory (root of the git repository).
pub fn load_dod_config() -> anyhow::Result<DodConfig> {
    let content = std::fs::read_to_string(".dod.yml")
        .context("Failed to read .dod.yml")?;
    let config: DodConfig = serde_yaml::from_str(&content)
        .context("Failed to parse .dod.yml")?;
    Ok(config)
}