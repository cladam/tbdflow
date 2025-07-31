use serde::{Deserialize, Serialize};
use std::fs;
use anyhow::Context;

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
pub struct Config {
    pub main_branch_name: String,
    pub stale_branch_threshold_days: i64,
    pub branch_prefixes: BranchPrefixes,
    pub automatic_tags: AutomaticTags,
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