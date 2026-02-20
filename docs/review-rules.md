# Review Rules

## How This Works for the Team

**Low Friction:** Most commits still just create a general review for the team.

**High Accountability:** If a junior developer touches a database migration, tbdflow automatically tags the db-expert on
the review issue.

**Mandatory Audits:** The `mandatory: true` flag ensures that high-risk files cannot be "silenced" by a developer using
the `$noreview` tag.

This directly addresses the **Safety vs. Throughput trade-off**. You are not blocking the merge, but you are ensuring
the right eyes see the change immediately after it hits the trunk.

## Configuration Example

```yaml
review:
  enabled: true
  strategy: github-issue
  default_reviewers:
    - cladam

  rules:
    # Always review database changes, even if $noreview is used.
    - pattern: "migrations/**"
      reviewers: [ "db-expert" ]
      mandatory: true

    # Targeted review for infrastructure changes
    - pattern: "infra/*.tf"
      reviewers: [ "devops-lead" ]

    # Targeted review for critical security modules
    - pattern: "src/auth/**"
      reviewers: [ "security-officer" ]
```

## Implementation

### Triggering a Non-blocking Review

The `trigger_review` function handles the review process based on configuration and rules:

```rust
/// Triggers a non-blocking review for a commit based on configuration and rules.
pub fn trigger_review(
    config: &Config,
    reviewers_override: Option<&[String]>,
    commit_hash: &str,
    message: &str,
    author: &str,
    verbose: bool,
    dry_run: bool,
) -> Result<()> {
    if !config.review.enabled {
        return Ok(());
    }

    // 1. Identify which rules apply based on touched files
    let touched_files = git::get_changed_files(commit_hash, verbose, dry_run)?;
    let mut applicable_reviewers: Vec<String> = Vec::new();
    let mut is_targeted = false;
    let mut is_mandatory = false;

    for rule in &config.review.rules {
        if let Ok(pattern) = Pattern::new(&rule.pattern) {
            let matched = touched_files.iter().any(|f| pattern.matches(f));
            if matched {
                if verbose {
                    println!("{} File match for rule: {}", "[RULE]".magenta(), rule.pattern.dimmed());
                }
                is_targeted = true;
                if rule.mandatory {
                    is_mandatory = true;
                }
                if let Some(rule_reviewers) = &rule.reviewers {
                    applicable_reviewers.extend(rule_reviewers.clone());
                }
            }
        }
    }

    // 2. Decide if we should skip
    let has_skip_tag = message.contains("$noreview");
    if has_skip_tag && !is_mandatory {
        if verbose {
            println!("{}", "Skipping review due to $noreview tag.".dimmed());
        }
        return Ok(());
    }

    // 3. Aggregate reviewers
    let mut final_reviewers = if let Some(ovr) = reviewers_override {
        ovr.to_vec()
    } else if !applicable_reviewers.is_empty() {
        applicable_reviewers
    } else {
        config.review.default_reviewers.clone()
    };

    final_reviewers.sort();
    final_reviewers.dedup();

    // 4. Trigger the review
    println!("{}", "--- Triggering Non-blocking Review ---".blue());
    if is_targeted {
        println!("{} Review triggered by targeted file rules.", "🎯".yellow());
    }

    let short = short_hash(commit_hash);
    println!(
        "{} {} ({})",
        "Review requested for:".green(),
        message.bold(),
        short.dimmed()
    );

    if dry_run {
        println!("{}", "[DRY RUN] Would create review request".yellow());
        return Ok(());
    }

    match &config.review.strategy {
        ReviewStrategy::GithubIssue => {
            create_github_issue(&final_reviewers, commit_hash, message, author, verbose)?;
        }
        ReviewStrategy::GithubWorkflow => {
            trigger_github_workflow(&final_reviewers, commit_hash, message, author, verbose)?;
        }
        ReviewStrategy::LogOnly => {
            println!("{}", "Review logged (no external system integration)".dimmed());
        }
    }

    Ok(())
}
```

### Data Structures

```rust
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ReviewRule {
    /// Glob pattern for files that trigger this rule (e.g., "src/auth/**", "infra/*.tf")
    pub pattern: String,
    /// Optional list of reviewers specifically for these files. 
    /// If empty, it uses the default_reviewers.
    pub reviewers: Option<Vec<String>>,
    /// If true, a review is always triggered for these files even if the commit message 
    /// contains $noreview.
    #[serde(default)]
    pub mandatory: bool,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ReviewConfig {
    pub enabled: bool,
    pub strategy: ReviewStrategy,
    pub default_reviewers: Vec<String>,
    #[serde(default)]
    pub rules: Vec<ReviewRule>,
}
```
