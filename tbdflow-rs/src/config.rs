use serde::{Deserialize, Serialize};
use std::fs;
use anyhow::Context;

/// The structs and functions for handling the configuration of the TBDFlow tool.
/// This includes reading the configuration from `.tbdflow.yml` and `.dod.yml` files,
/// as well as defining the structure of the configuration data.

#[derive(Debug, Deserialize, Default)]
pub struct DodConfig {
    pub issue_reference_required: Option<bool>,
    #[serde(default)]
    pub checklist: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BranchPrefixes {
    pub feature: String,
    pub release: String,
    pub hotfix: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutomaticTags {
    pub release_prefix: String,
    pub hotfix_prefix: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConventionalCommitTypeConfig {
    pub enabled: Option<bool>,
    pub allowed_types: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueKeyConfig {
    pub enabled: Option<bool>,
    pub pattern: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubjectLineRules {
    pub subject_line_max_length: Option<usize>,
    pub subject_line_not_capitalized: Option<bool>,
    pub subject_line_no_period: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BodyLineRules {
    pub body_max_line_length: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LintConfig {
    pub conventional_commit_type: Option<ConventionalCommitTypeConfig>,
    pub issue_key_missing: Option<IssueKeyConfig>,
    pub subject_line_rules: Option<SubjectLineRules>,
    pub body_line_rules: Option<BodyLineRules>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub main_branch_name: String,
    pub stale_branch_threshold_days: i64,
    pub branch_prefixes: BranchPrefixes,
    pub automatic_tags: AutomaticTags,
    pub lint: Option<LintConfig>,
}

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
                hotfix_prefix: "hotfix_".to_string(),
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