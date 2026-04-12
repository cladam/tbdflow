use crate::git;
use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum ReviewStrategy {
    /// Create GitHub issues for review tracking (requires `gh` CLI).
    #[default]
    GithubIssue,
    /// Trigger a GitHub Actions workflow for server-side review management.
    GithubWorkflow,
    /// Log reviews locally without external integration.
    LogOnly,
}

/// Maps file glob patterns to specific reviewers.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ReviewRule {
    /// e.g. "src/auth/**", "infra/*.tf"
    pub pattern: String,
    /// Falls back to `default_reviewers` if empty.
    #[serde(default)]
    pub reviewers: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReviewLabelsConfig {
    #[serde(default = "ReviewLabelsConfig::default_pending")]
    pub pending: String,
    #[serde(default = "ReviewLabelsConfig::default_concern")]
    pub concern: String,
    #[serde(default = "ReviewLabelsConfig::default_accepted")]
    pub accepted: String,
    #[serde(default = "ReviewLabelsConfig::default_dismissed")]
    pub dismissed: String,
}

impl Default for ReviewLabelsConfig {
    fn default() -> Self {
        Self {
            pending: Self::default_pending(),
            concern: Self::default_concern(),
            accepted: Self::default_accepted(),
            dismissed: Self::default_dismissed(),
        }
    }
}

impl ReviewLabelsConfig {
    fn default_pending() -> String {
        "review-pending".to_string()
    }
    fn default_concern() -> String {
        "review-concern".to_string()
    }
    fn default_accepted() -> String {
        "review-accepted".to_string()
    }
    fn default_dismissed() -> String {
        "review-dismissed".to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum RadarLevel {
    #[default]
    File,
    Line,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum RadarOnCommit {
    #[default]
    Off,
    Warn,
    Confirm,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RadarConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub level: RadarLevel,
    #[serde(default = "RadarConfig::default_on_sync")]
    pub on_sync: bool,
    #[serde(default)]
    pub on_commit: RadarOnCommit,
    #[serde(default)]
    pub ignore_patterns: Vec<String>,
}

impl Default for RadarConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            level: RadarLevel::File,
            on_sync: true,
            on_commit: RadarOnCommit::Off,
            ignore_patterns: vec![
                "*.lock".to_string(),
                "*-lock.*".to_string(),
                "CHANGELOG.md".to_string(),
            ],
        }
    }
}

impl RadarConfig {
    fn default_on_sync() -> bool {
        true
    }
}

/// Pre-flight CI status check via `gh` CLI during `tbdflow sync`.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CiCheckConfig {
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ReviewConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub default_reviewers: Vec<String>,
    #[serde(default)]
    pub strategy: ReviewStrategy,
    /// Workflow filename for `github-workflow` strategy (e.g. "nbr-review.yml").
    #[serde(default)]
    pub workflow: Option<String>,
    #[serde(default)]
    pub rules: Vec<ReviewRule>,
    #[serde(default)]
    pub labels: ReviewLabelsConfig,
    /// If true, a concern sets commit status to 'failure' instead of 'pending'.
    #[serde(default)]
    pub concern_blocks_status: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IssueHandlingStrategy {
    BranchName,
    CommitScope,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AutomaticTags {
    pub release_prefix: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConventionalCommitTypeConfig {
    pub enabled: Option<bool>,
    pub allowed_types: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IssueKeyConfig {
    pub enabled: Option<bool>,
    pub pattern: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScopeConfig {
    pub enabled: Option<bool>,
    pub enforce_lowercase: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubjectLineRules {
    pub max_length: Option<usize>,
    pub enforce_lowercase: Option<bool>,
    pub no_period: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BodyLineRules {
    pub max_line_length: Option<usize>,
    pub leading_blank: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LintConfig {
    pub conventional_commit_type: Option<ConventionalCommitTypeConfig>,
    pub issue_key_missing: Option<IssueKeyConfig>,
    pub scope: Option<ScopeConfig>,
    pub subject_line_rules: Option<SubjectLineRules>,
    pub body_line_rules: Option<BodyLineRules>,
}

/// Loaded from `.tbdflow.yml` at the git root, with optional per-project overrides.
#[derive(Debug, Serialize, Deserialize, Clone)]
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
    #[serde(default)]
    pub review: ReviewConfig,
    #[serde(default)]
    pub radar: RadarConfig,
    #[serde(default)]
    pub ci_check: CiCheckConfig,
    pub branch_types: HashMap<String, String>,
    pub automatic_tags: AutomaticTags,
    pub lint: Option<LintConfig>,
}

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
        Config {
            main_branch_name: "main".to_string(),
            project_root: None,
            release_url_template: Some(
                "https://github.com/owner/repository/releases/tag/{{version}}".to_string(),
            ),
            stale_branch_threshold_days: 1,
            monorepo: MonorepoConfig::default(),
            issue_handling: IssueHandling::default(),
            review: ReviewConfig::default(),
            radar: RadarConfig::default(),
            ci_check: CiCheckConfig::default(),
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
                    pattern: Some(r"^[A-Z]+-\d+$".to_string()),
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

fn merge_configs(parent: &mut Config, child: Config) {
    if child.project_root.is_some() {
        parent.project_root = child.project_root;
    }

    for (key, value) in child.branch_types {
        parent.branch_types.insert(key, value);
    }

    parent.issue_handling = child.issue_handling;

    if child.lint.is_some() {
        parent.lint = child.lint;
    }

    // Global fields intentionally not merged:
    // main_branch_name, release_url_template, stale_branch_threshold_days,
    // monorepo, automatic_tags
}

pub fn load_tbdflow_config() -> Result<Config, anyhow::Error> {
    let verbose = false;
    let dry_run = false;

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
        yaml_serde::from_str(&config_str)
            .map_err(|e| anyhow!("Failed to parse root .tbdflow.yml: {}", e))?
    } else {
        Config::default()
    };

    // Check if we are in a subdirectory and if a local config exists.
    let current_dir = std::env::current_dir()?;
    if current_dir != Path::new(&git_root) {
        let local_config_path = current_dir.join(".tbdflow.yml");
        if local_config_path.exists() {
            let local_config_str = fs::read_to_string(local_config_path)?;
            let local_config: Config = yaml_serde::from_str(&local_config_str)
                .map_err(|e| anyhow!("Failed to parse local .tbdflow.yml: {}", e))?;
            merge_configs(&mut base_config, local_config);
        }
    }

    Ok(base_config)
}

pub fn load_dod_config() -> anyhow::Result<DodConfig> {
    let content = fs::read_to_string(".dod.yml").context("Failed to read .dod.yml")?;
    let config: DodConfig = yaml_serde::from_str(&content).context("Failed to parse .dod.yml")?;
    Ok(config)
}

pub fn is_monorepo_root(config: &Config, current_dir: &Path, git_root: &Path) -> bool {
    current_dir == git_root && config.monorepo.enabled && !config.monorepo.project_dirs.is_empty()
}

pub fn find_project_root() -> Result<Option<PathBuf>, anyhow::Error> {
    let mut current_dir = std::env::current_dir()?;
    let git_root = PathBuf::from(git::get_git_root(false, false)?);

    loop {
        let config_path = current_dir.join(".tbdflow.yml");
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: Config = yaml_serde::from_str(&content)?;
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
