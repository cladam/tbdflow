use crate::git;
use anyhow::{anyhow, Context};
/// The structs and functions for handling the configuration of the TBDFlow tool.
/// This includes reading the configuration from `.tbdflow.yml` and `.dod.yml` files,
/// as well as defining the structure of the configuration data.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Represents the Definition of Done (DoD) configuration.
#[derive(Debug, Deserialize, Default)]
pub struct DodConfig {
    #[serde(default)]
    pub checklist: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MonorepoConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub project_dirs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IssueHandlingStrategy {
    BranchName,
    CommitScope,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueHandling {
    pub strategy: IssueHandlingStrategy,
}

impl Default for IssueHandling {
    fn default() -> Self {
        Self {
            strategy: IssueHandlingStrategy::BranchName,
        }
    }
}

/// Represents the automatic tagging configuration for releases and hotfixes.
#[derive(Debug, Serialize, Deserialize)]
pub struct AutomaticTags {
    pub release_prefix: String,
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

/// Represents the configuration for scopes in commit messages.
#[derive(Debug, Serialize, Deserialize)]
pub struct ScopeConfig {
    pub enabled: Option<bool>,
    pub enforce_lowercase: Option<bool>,
}

/// Represents the rules for validating the subject line of commit messages.
/// This includes checks for maximum length, capitalization, and whether it ends with a period.
/// The subject line rules help ensure that commit messages are clear and follow a consistent format.
#[derive(Debug, Serialize, Deserialize)]
pub struct SubjectLineRules {
    pub max_length: Option<usize>,
    pub enforce_lowercase: Option<bool>,
    pub no_period: Option<bool>,
}

/// Represents the rules for validating the body lines of commit messages.
#[derive(Debug, Serialize, Deserialize)]
pub struct BodyLineRules {
    pub max_line_length: Option<usize>,
    pub leading_blank: Option<bool>,
}

/// Represents the configuration for linting commit messages.
/// This includes configurations for conventional commit types, issue key validation,
/// subject line rules, and body line rules.
/// The lint configuration helps enforce best practices in commit messages,
#[derive(Debug, Serialize, Deserialize)]
pub struct LintConfig {
    pub conventional_commit_type: Option<ConventionalCommitTypeConfig>,
    pub issue_key_missing: Option<IssueKeyConfig>,
    pub scope: Option<ScopeConfig>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_root: Option<String>,
    pub release_url_template: Option<String>,
    pub stale_branch_threshold_days: i64,
    #[serde(default)]
    pub monorepo: MonorepoConfig,
    #[serde(default)]
    pub issue_handling: IssueHandling,
    pub branch_types: HashMap<String, String>,
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
        let mut branch_types = HashMap::new();
        branch_types.insert("feat".to_string(), "feat/".to_string());
        branch_types.insert("fix".to_string(), "fix/".to_string());
        branch_types.insert("chore".to_string(), "chore/".to_string());
        branch_types.insert("docs".to_string(), "docs/".to_string());
        branch_types.insert("refactor".to_string(), "refactor/".to_string());
        branch_types.insert("ci".to_string(), "ci/".to_string());
        branch_types.insert("release".to_string(), "release_".to_string());
        // Adding feature and hotfix branch types for backward compatibility
        branch_types.insert("feature".to_string(), "feature_".to_string());
        branch_types.insert("hotfix".to_string(), "hotfix_".to_string());
        Config {
            main_branch_name: "main".to_string(),
            project_root: None,
            release_url_template: Some(
                "https://github.com/owner/repository/releases/tag/{{version}}".to_string(),
            ),
            stale_branch_threshold_days: 1,
            monorepo: MonorepoConfig::default(),
            issue_handling: IssueHandling::default(),
            branch_types,
            automatic_tags: AutomaticTags {
                release_prefix: "v".to_string(),
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
                scope: Some(ScopeConfig {
                    enabled: Some(true),
                    enforce_lowercase: Some(true),
                }),
                subject_line_rules: Some(SubjectLineRules {
                    max_length: Some(72),
                    enforce_lowercase: Some(true),
                    no_period: Some(true),
                }),
                body_line_rules: Some(BodyLineRules {
                    max_line_length: Some(80),
                    leading_blank: Option::from(true),
                }),
            }),
        }
    }
}

// Merges a child config (from a subdirectory) into a parent config.
fn merge_configs(parent: &mut Config, child: Config) {
    // Fields that are typically project-specific
    if child.project_root.is_some() {
        parent.project_root = child.project_root;
    }

    // Merge branch_types: child can add or override parent's
    for (key, value) in child.branch_types {
        parent.branch_types.insert(key, value);
    }

    // Overwrite issue handling strategy if specified in child
    parent.issue_handling = child.issue_handling;

    // Overwrite linting configuration if specified in child
    if child.lint.is_some() {
        parent.lint = child.lint;
    }

    // Fields that are generally global and should not be merged
    // - main_branch_name
    // - release_url_template
    // - stale_branch_threshold_days
    // - monorepo
    // - branch_prefixes
    // - automatic_tags
}

/// Loads the configuration from the `.tbdflow.yml` file in the current directory (root of the git repository).
pub fn load_tbdflow_config() -> Result<Config, anyhow::Error> {
    // Use a dummy verbose/dry_run setting for this internal operation.
    let verbose = false;
    let dry_run = false;

    // Find the root of the git repository.
    let git_root = match git::get_git_root(verbose, dry_run) {
        Ok(path) => path,
        Err(_) => {
            // Not in a git repo, so we can't find the config.
            // Return default config silently as before.
            return Ok(Config::default());
        }
    };
    // Load base config from git root, or use default.
    let root_config_path = Path::new(&git_root).join(".tbdflow.yml");
    let mut base_config = if root_config_path.exists() {
        let config_str = fs::read_to_string(root_config_path)?;
        serde_yaml::from_str(&config_str)
            .map_err(|e| anyhow!("Failed to parse root .tbdflow.yml: {}", e))?
    } else {
        Config::default()
    };

    // Check if we are in a subdirectory and if a local config exists.
    let current_dir = std::env::current_dir()?;
    if current_dir != Path::new(&git_root) {
        let local_config_path = current_dir.join(".tbdflow.yml");
        if local_config_path.exists() {
            // 3. Load local config and merge it into the base config.
            let local_config_str = fs::read_to_string(local_config_path)?;
            let local_config: Config = serde_yaml::from_str(&local_config_str)
                .map_err(|e| anyhow!("Failed to parse local .tbdflow.yml: {}", e))?;
            merge_configs(&mut base_config, local_config);
        }
    }

    Ok(base_config)
}

/// Reads the DoD configuration from the `.dod.yml` file in the current directory (root of the git repository).
pub fn load_dod_config() -> anyhow::Result<DodConfig> {
    let content = std::fs::read_to_string(".dod.yml").context("Failed to read .dod.yml")?;
    let config: DodConfig = serde_yaml::from_str(&content).context("Failed to parse .dod.yml")?;
    Ok(config)
}

/// Checks if the current context is the root of a configured monorepo.
pub fn is_monorepo_root(config: &Config, current_dir: &Path, git_root: &Path) -> bool {
    current_dir == git_root && config.monorepo.enabled && !config.monorepo.project_dirs.is_empty()
}

// New function to find the root of the current sub-project.
pub fn find_project_root() -> Result<Option<PathBuf>, anyhow::Error> {
    let mut current_dir = std::env::current_dir()?;
    let git_root = PathBuf::from(git::get_git_root(false, false)?); // Use non-verbose for internal check

    loop {
        let config_path = current_dir.join(".tbdflow.yml");
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: Config = serde_yaml::from_str(&content)?;
            if config.project_root.is_some() {
                return Ok(Some(current_dir));
            }
        }

        if current_dir == git_root || current_dir.parent().is_none() {
            break;
        }
        current_dir = current_dir.parent().unwrap().to_path_buf();
    }

    Ok(None)
}
