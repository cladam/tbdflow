use crate::config::Config;
use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

// Struct to hold the results from the commit wizard
// All of these are flags that can be passed to the `commit` command
#[derive(Debug, Clone)]
pub struct CommitWizardResult {
    pub r#type: String,
    pub scope: Option<String>,
    pub message: String,
    pub body: Option<String>,
    pub breaking: bool,
    pub breaking_description: Option<String>,
    pub tag: Option<String>,
    pub issue: Option<String>,
}

// Struct to hold the results from the branch wizard
// All of these are flags that can be passed to the `branch` command
#[derive(Debug, Clone)]
pub struct BranchWizardResult {
    pub branch_type: String,
    pub name: String,
    pub issue: Option<String>,
}

// Struct to hold the results from the complete wizard
// All of these are flags that can be passed to the `complete` command
#[derive(Debug, Clone)]
pub struct CompleteWizardResult {
    pub branch_type: String,
    pub name: String,
}

// Struct to hold the results from the changelog wizard
// All of these are flags that can be passed to the `changelog` command
#[derive(Debug, Clone)]
pub struct ChangeLogWizardResult {
    pub from: String,
    pub to: String,
    pub unreleased: bool,
}

// Function to run the commit wizard
// Function to run the commit wizard
pub fn run_commit_wizard(config: &Config) -> Result<CommitWizardResult> {
    let theme = ColorfulTheme::default();
    println!("Welcome to the Commit Wizard!");
    println!("This wizard will guide you through creating a well-structured commit message.");
    println!("You can press Ctrl+C at any time to exit the wizard.\n");

    // Load commit types from config or use defaults
    let allowed_types = config
        .lint
        .as_ref()
        .and_then(|l| l.conventional_commit_type.as_ref())
        .and_then(|cct| cct.allowed_types.as_ref())
        .map(|types| types.clone())
        .unwrap_or_else(|| {
            vec![
                "feat".to_string(),
                "fix".to_string(),
                "chore".to_string(),
                "docs".to_string(),
                "style".to_string(),
                "refactor".to_string(),
                "perf".to_string(),
                "test".to_string(),
                "build".to_string(),
                "ci".to_string(),
                "revert".to_string(),
                "wip".to_string(),
            ]
        });

    let type_selection = Select::with_theme(&theme)
        .with_prompt("Select the type of change")
        .items(&allowed_types)
        .default(0)
        .interact()?;
    let r#type = allowed_types[type_selection].clone();

    // Helper function to convert empty strings from dialoguer to None
    fn to_option(s: String) -> Option<String> {
        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    }

    let scope: Option<String> = to_option(
        Input::<String>::with_theme(&theme)
            .with_prompt("Enter the scope of this change (optional)")
            .allow_empty(true)
            .interact_text()?,
    );

    let message: String = Input::with_theme(&theme)
        .with_prompt("Write a short, imperative tense description of the change")
        .interact_text()?;

    let body: Option<String> = to_option(
        Input::<String>::with_theme(&theme)
            .with_prompt("Provide a longer description of the change (optional)")
            .allow_empty(true)
            .interact_text()?,
    );

    let breaking = Confirm::with_theme(&theme)
        .with_prompt("Is this a breaking change?")
        .default(false)
        .interact()?;

    let breaking_description: Option<String> = if breaking {
        Some(
            Input::<String>::with_theme(&theme)
                .with_prompt("Describe the breaking change")
                .interact_text()?,
        )
    } else {
        None
    };

    let issue: Option<String> = to_option(
        Input::<String>::with_theme(&theme)
            .with_prompt("Enter an issue reference (e.g., PROJ-123) (optional)")
            .allow_empty(true)
            .interact_text()?,
    );

    let tag: Option<String> = to_option(
        Input::<String>::with_theme(&theme)
            .with_prompt("Enter a tag for this commit (optional)")
            .allow_empty(true)
            .interact_text()?,
    );

    Ok(CommitWizardResult {
        r#type,
        scope,
        message,
        body,
        breaking,
        breaking_description,
        tag,
        issue,
    })
}
