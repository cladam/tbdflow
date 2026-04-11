<div align="center">
  <p align="center">
    <img src="assets/tbdflow-logo.png" alt="tbdflow logo" width="200"/>
  </p>

  <p align="center">
    <i><b>Keep your code flowing</b></i><br/>
  </p>

[![Crates.io](https://img.shields.io/crates/v/tbdflow.svg)](https://crates.io/crates/tbdflow)
[![Downloads](https://img.shields.io/crates/d/tbdflow.svg)](https://crates.io/crates/tbdflow)

</div>

## The problem

Many teams say they practise Trunk-Based Development but in day-to-day reality things deviate:

- **Commit messages become inconsistent.** Everyone formats them a little differently.
- **Branches that were meant to live for hours** stick around for days.
- **Merging back to main** turns into a manual sequence people half-remember.
- **Two people change the same file** and nobody notices until a push fails.
- **The Definition of Done exists,** but it lives in a document no one looks at during the work.

None of this breaks the build immediately. But over time, the trunk stops feeling safe to work in.

## The solution

`tbdflow` is a small CLI that **codifies your team's Trunk-Based workflow** so the safe path is always the easiest path.

```bash
cargo install tbdflow
```

It handles the ceremony (pulling, rebasing, linting, pushing) so you can stay focused on the work.

![A terminal running the command tbdflow](docs/commit-demo.gif "A demo of tbdflow running commit-to-main commands")

## What it does

| Pain point                     | How tbdflow helps                                                          |
|--------------------------------|----------------------------------------------------------------------------|
| Inconsistent commits           | `tbdflow commit` enforces Conventional Commits with built-in linting       |
| Long-lived branches            | `tbdflow branch` + `tbdflow complete` with stale-branch warnings           |
| "Did I pull before pushing?"   | `tbdflow sync` + auto-rebase before every commit to main                   |
| Pulling a broken trunk         | `tbdflow sync` pre-flight CI check warns before pulling a red build        |
| Merge conflicts you didn't see | `tbdflow radar` shows who else is touching the same files, before you push |
| "Why was this done?"           | `tbdflow task` + `tbdflow note` captures intent before it's lost           |

## Philosophy

* **Main is where the work happens.** `tbdflow commit` is your daily driver: pull, commit, push, done. Small and
  frequent beats large and delayed.
* **Branches are short-lived guests.** They're supported, but they should check out quickly.
* **Cleanup shouldn't be your job.** Completed branches get merged, tagged (for releases), and deleted automatically.
* **Commit messages should tell a story.** [Conventional Commits](https://www.conventionalcommits.org/) keep the
  history readable for humans and machines alike.
* **Collaboration should be visible.** `tbdflow radar` shows who else is touching the same files, turning silent
  conflicts into early conversations.

### Why not just use Git?

You absolutely should. `tbdflow` isn't a replacement. You'll still reach for raw `git` when rebasing, cherry-picking,
or bisecting.

Think of it as a **workflow assistant** that wraps the repeatable parts of your day:

1. **Everyone does it the same way.**
   Commits, branches, and releases follow the same steps every time. No more "how did you format that commit again?"

2. **Less to keep in your head.**
   You don't need to remember `pull --rebase` then commit then push then tag then delete branch. The CLI does.

3. **The TBD path is the easy path.**
   For 80% of your day, `tbdflow` keeps you in the groove. For the other 20%, Git is right there.

### Installation

You need [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.

#### Installing from crates.io

```bash
cargo install tbdflow
```

To update to the latest version:

```bash
tbdflow update
```

#### Building from source

Or build it yourself:

```bash
git clone https://github.com/cladam/tbdflow.git
cd tbdflow
sudo cargo install --path . --root /usr/local
```

### Monorepo Support

If you work in a monorepo, `tbdflow` understands that not every commit should touch every directory.

When you run `tbdflow commit`, `tbdflow sync` or `tbdflow status` from the repo root, only root-level files are
affected. Project subdirectories are left alone. Run the same commands from inside a project directory and they
automatically scope to that directory. (Run `tbdflow init` in each subdirectory to set this up.)

This is configured in your root `.tbdflow.yml` file:

```
# in .tbdflow.yml
monorepo:
enabled: true
  # A list of all directories that are self-contained projects.
  # These will be excluded from root-level commits and status checks.
  project_dirs:
    - "frontend"
    - "backend-api"
    - "infra"
```

For an overview and to inspect your current configuration, you can run `tbdflow info`.

#### Handling Cross-Cutting Changes

For "vertical slice" changes that intentionally touch multiple project directories, you can use the `--include-projects`
flag.
This flag overrides the default safety mechanism and stages all changes from all directories, allowing you to create a
single, cross-cutting commit.

### Interactive Wizard Mode

To make `tbdflow` even more user-friendly, the core commands (`branch`, `commit`, `complete`, `changelog`) now feature
an interactive "wizard" mode.

If you run one of these commands without providing the required flags, `tbdflow` will automatically launch a
step-by-step guide.
This is perfect for new users who are still learning the workflow, or for complex commits where you want to be sure
you've covered all the options.

For power users, the original flag-based interface is still available for a faster, scripted experience.

### Configuration

`tbdflow` is configurable via two optional files in the root of your repository. To get started quickly, run
`tbdflow init` to generate default versions of these files.

`.tbdflow.yml`
This file controls the core workflow of the tool. You can customise:

- The name of your main branch (e.g. main, trunk).
- Allowed branch types and their prefixes (e.g feat/, chore/)
- A strategy for handling issue references ("branch-name" or "commit-scope")
- The threshold for stale branch warnings.
- Automatic tagging formats.
- Commit message linting rules.

> **Note:** `main_branch_name` configures which branch is your trunk (typically `main` or `master`).
> tbdflow assumes this branch accepts direct commits. For protected branches, use short-lived feature branches with
`tbdflow branch`.

`.dod.yml`
This file controls the interactive Definition of Done checklist for the commit command.

### Features

#### The Definition of Done (DoD) Check

Most teams have a Definition of Done. Most of the time, it lives in a wiki nobody opens mid-task.

If you add a `.dod.yml` to your repo, `tbdflow commit` will surface the checklist right when it matters, before you
push. It's optional, non-blocking, and stays out of your way when you don't need it.

**Example** `.dod.yml`:

```
# .dod.yml in your project root
checklist:
  - "All relevant automated tests pass successfully."
  - "New features or fixes are covered by new tests."
  - "Security implications of this change have been considered."
  - "Relevant documentation (code comments, READMEs) is updated."
```

If you skip items, `tbdflow` offers to add a TODO list to the commit footer so the incomplete work is tracked in
Git history, not lost in a chat thread.

#### Commit Message Linting

Your `.tbdflow.yml` can include linting rules that catch issues before the commit happens: subject too long, wrong
type, missing scope. Quick feedback, no surprises in the log later.

**Default linting rules:**

```yaml
lint:
  conventional_commit_type:
    enabled: true
    allowed_types:
      - build
      - chore
      - ci
      - docs
      - feat
      - fix
      - perf
      - refactor
      - revert
      - style
      - test
  issue_key_missing:
    enabled: false
    pattern: ^[A-Z]+-\d+$
  scope:
    enabled: true
    enforce_lowercase: true
  subject_line_rules:
    max_length: 72
    enforce_lowercase: true
    no_period: true
  body_line_rules:
    max_line_length: 80
    leading_blank: true
```

#### Intent Log

You tried three approaches before settling on the final one. By the time you commit, the first two are gone. From
your memory and from the diff. A week later, a reviewer suggests one of the approaches you already rejected.

The Intent Log fixes this. While you work, you drop one-line breadcrumbs. At commit time, they're woven into the
message body automatically. Zero context-switching, full context for whoever reads the commit next.

**Start a task (optional):**

```bash
tbdflow task start "Refactor auth logic"
```

**Leave notes as you work:**

```bash
tbdflow note "tried factory pattern, felt too verbose"
tbdflow + "switching to a simple trait implementation"
tbdflow n "trait approach is cleaner, keeping it"
```

The `note` command has two shorthand aliases: `+` and `n`.

**Notes are consumed at commit time:**

When you run `tbdflow commit`, the notes are appended to the commit body automatically:

```
feat(auth): implement trait-based auth logic

Intent Log:
- tried factory pattern, felt too verbose
- switching to a simple trait implementation
- trait approach is cleaner, keeping it
```

**Other task commands:**

```bash
tbdflow task show    # Show the current task and notes
tbdflow task clear   # Discard the current intent log
```

**Branch awareness:**

The intent log tracks which branch it belongs to. If you switch branches, tbdflow warns you about the stale log so
notes from one task don't leak into another commit.

**File:** Notes are stored locally in `.tbdflow-intent.json` (git-ignored, never committed). The file is deleted
automatically after a successful push to trunk or after `tbdflow complete`.

---

## Global options

| Flag      | Description                                              | Required |
|-----------|----------------------------------------------------------|----------|
| --verbose | Prints the underlying Git commands as they are executed. | No       |
| --dry-run | Simulate the command without making any changes.         | No       |

## Commands

### 1. `commit`

This is the primary command for daily work.

Commits staged changes using a Conventional Commits message. This command is context-aware:

* **On `main`:** It runs the full TBD workflow: pulls the latest changes with rebase, commits, and pushes.
* **On any other branch:** It simply commits and pushes, allowing you to save work-in-progress.

**Usage:**

```bash
tbdflow commit [options]
```

**Options:**

| Flag | Option                 | Description                                              | Required |
|------|------------------------|----------------------------------------------------------|----------|
| -t   | --type                 | The type of commit (e.g., feat, fix, chore).             | Yes      |
| -s   | --scope                | The scope of the changes (e.g., api, ui).                | No       |
| -m   | --message              | The descriptive commit message (subject line).           | Yes      |
|      | --body                 | Optional multi-line body for the commit message.         | No       |
| -b   | --breaking             | Mark the commit as a breaking change.                    | No       |
|      | --breaking-description | Provide a description for the 'BREAKING CHANGE:' footer. | No       |
|      | --tag                  | Optionally add and push an annotated tag to this commit. | No       |
|      | --issue                | Optionally add an issue reference to the footer.         | No       |
|      | --no-verify            | Bypass the interactive DoD checklist.                    | No       |

**Example:**

```bash
# A new feature
tbdflow commit -t feat -s auth -m "add password reset endpoint"

# A bug fix with a breaking change
tbdflow commit -t fix -m "correct user permission logic" -b
tbdflow commit -t refactor -m "rename internal API" --breaking --breaking-description "The `getUser` function has been renamed to `fetchUser`."

# A bug fix with a new tag
tbdflow commit -t fix -m "correct user permission logic" --tag "v1.1.1"
```

### 2. `branch`

Creates and pushes a new, short-lived branch from the latest version of `main`. This is the primary command for starting
new work that isn't a direct commit to `main`.

**Usage:**

```bash
tbdflow branch --type <type> --name <name> [--issue <issue-id>] [--from_commit <commit hash>]
```

**Options (release):**

| Flag              | Description                                                                     | Required |
|-------------------|---------------------------------------------------------------------------------|----------|
| -t, --type        | The type of branch (e.g. feat, fix, chore). See .tbdflow.yml for allowed types. | Yes      |
| -n, --name        | A short, desriptive name for the branch.                                        | Yes      |
| --issue           | Optional issue reference to include in the branch name or commit scope.         | No       |
| -f, --from_commit | Optional commit hash on `main` to branch from.                                  | No       |

**Examples:**

```bash
# Create a simple feature branch named "feat/new-dashboard"
tbdflow branch -t feat -n "new-dashboard"

# Create a fix branch with an issue reference in the name
# (This will be named "fix/PROJ-123-login-bug" by default)
tbdflow branch -t fix -n "login-bug" --issue "PROJ-123"

# Create a release branch from a specific commit
tbdflow branch -t release -v "2.1.0" -f "39b68b5"
```

### 3. `complete`

Merges a short-lived branch back into main, then deletes the local and remote copies of the branch.

**Automatic Tagging:**

* When completing a release branch, a tag (e.g. v2.1.0) is automatically created and pushed.

**Usage:**

```bash
tbdflow complete --type <branch-type> --name <branch-name>
```

**Options:**

| Flag | Option | Description                                      | Required |
|------|--------|--------------------------------------------------|----------|
| -t   | --type | The type of branch: feature, release, or hotfix. | Yes      |
| -n   | --name | The name or version of the branch to complete.   | Yes      |

**Examples:**

```bash
# Complete a feature branch
tbdflow complete -t feat -n "user-profile-page"

# Complete a release branch (this will be tagged v2.1.0)
tbdflow complete -t release -n "2.1.0"
```

### 4. `changelog`

Generates a changelog in Markdown format from your repository's Conventional Commit history. See `tbdflow` repo for a
CHANGELOG.md generated by this command.

**Usage:**

```bash
tbdflow changelog [options]
```

**Options:**

| Option       | Description                                                               |
|--------------|---------------------------------------------------------------------------|
| --unreleased | Generate a changelog for all commits since the last tag.                  |
| --from       | Generate a changelog for commits from a specific tag.                     |
| --to         | Generate a changelog for commits up to a specific tag (defaults to HEAD). |

**Examples:**

```bash
# Generate a changelog for a new version
tbdflow changelog --from v0.12.0 --to v0.13.0

# See what will be in the next release
tbdflow changelog --unreleased
```

### 5. `review`

Manages non-blocking post-commit reviews for trunk-based development. In TBD, code is committed to trunk first and
reviewed asynchronously, this command facilitates that workflow by creating GitHub issues for review tracking.

**Philosophy:**

In Trunk-Based Development, reviews are for **course correction** and **knowledge sharing**, not gatekeeping.
Code is already in trunk; reviewers focus on Intent, Impact, and Insight.

**Usage:**

```bash
tbdflow review [sha] [options]
```

**Options:**

| Option                | Description                                                            |
|-----------------------|------------------------------------------------------------------------|
| \<sha\>               | Trigger a review for a specific commit (positional argument).          |
| --trigger             | Create a review request for the current HEAD commit.                   |
| --digest              | Generate a digest of commits needing review.                           |
| --approve \<hash\>    | Mark a commit as approved (closes issue with `review-accepted`).       |
| --concern \<hash\>    | Raise a concern on a commit (keeps issue open, adds `review-concern`). |
| --dismiss \<hash\>    | Dismiss a review (closes issue with `review-dismissed`).               |
| -m, --message         | Message for concern or dismiss (required with --concern/--dismiss).    |
| --since \<time\>      | Time range for digest (default: "1 day ago").                          |
| --reviewers \<users\> | Override default reviewers (comma-separated GitHub usernames).         |

**Examples:**

```bash
# Create a review issue for a specific commit
tbdflow review abc1234

# Create a review issue for the latest commit (HEAD)
tbdflow review --trigger

# See commits from the last 3 days that may need review
tbdflow review --digest --since "3 days ago"

# Mark a commit as reviewed (closes the associated GitHub issue)
tbdflow review --approve abc1234

# Raise a concern on a commit (keeps issue open, notifies author)
tbdflow review --concern abc1234 -m "Potential thread safety issue"

# Dismiss a review without fixing (closes issue)
tbdflow review --dismiss abc1234 -m "Won't fix, out of scope"
```

#### Review Labels (Nuanced Statuses)

`tbdflow` uses configurable labels to track review status throughout the lifecycle:

| Label              | Description                                     | Issue State |
|--------------------|-------------------------------------------------|-------------|
| `review-pending`   | Review awaiting attention (default on creation) | Open        |
| `review-concern`   | Concern raised - needs attention from author    | Open        |
| `review-accepted`  | Review approved                                 | Closed      |
| `review-dismissed` | Review dismissed (won't fix)                    | Closed      |

**Concern Workflow:**

When you raise a concern with `--concern`:

1. The issue label changes from `review-pending` to `review-concern`
2. A comment is added to the issue with the concern message
3. A checklist item is appended to the issue body: `- [ ] <concern>`
4. (Optional) A commit status is set based on `concern_blocks_status` config

This is **always non-blocking**, concerns are informational and encourage fix-forward patterns.

**Configuration:**

Enable the review system in your `.tbdflow.yml`:

```yaml
review:
  enabled: true
  strategy: github-issue  # or "github-workflow" or "log-only"
  default_reviewers:
    - teammate-username
    - another-reviewer

  # Optional: Customise label names (defaults shown)
  labels:
    pending: "review-pending"
    concern: "review-concern"
    accepted: "review-accepted"
    dismissed: "review-dismissed"

  # Optional: Set commit status to 'failure' when concern is raised
  # If false (default), status is 'pending' with description
  concern_blocks_status: false
```

**Commit Status Behaviour:**

When `concern_blocks_status` is configured:

| Setting           | Status State | Description                                   |
|-------------------|--------------|-----------------------------------------------|
| `false` (default) | `pending`    | "Awaiting fix-forward for concern: [message]" |
| `true`            | `failure`    | "Audit Concern: [message]"                    |

#### Targeted Review Rules

For teams that need specific reviewers for certain files or directories, you can configure **review rules** with glob
patterns. When rules are configured, reviews are **automatically triggered** after a commit if any changed files match
a rule pattern. The appropriate reviewers are assigned based on the matching rules.

This allows:

- **Opt-in by Default**: Without rules, `tbdflow review --trigger` is manual
- **Auto-trigger with Rules**: When rules are configured and files match, reviews are triggered automatically after
  commit
- **Smart Routing**: Database changes go to the DB expert, infrastructure changes go to DevOps, etc.

```yaml
review:
  enabled: true
  strategy: github-issue
  default_reviewers:
    - cladam

  rules:
    # Database changes get reviewed by the DB expert
    - pattern: "migrations/**"
      reviewers: [ "db-expert" ]

    # Targeted review for infrastructure changes
    - pattern: "infra/*.tf"
      reviewers: [ "devops-lead" ]

    # Targeted review for critical security modules
    - pattern: "src/auth/**"
      reviewers: [ "security-officer" ]
```

**Rule Options:**

| Field       | Description                                                              | Required |
|-------------|--------------------------------------------------------------------------|----------|
| `pattern`   | Glob pattern for files that trigger this rule (e.g., `src/auth/**`)      | Yes      |
| `reviewers` | List of reviewers specifically for these files (uses default if not set) | No       |

**Strategies:**

| Strategy          | Description                                            | Best For                             |
|-------------------|--------------------------------------------------------|--------------------------------------|
| `github-issue`    | CLI creates GitHub issues directly                     | Small teams, simple setup            |
| `github-workflow` | CLI triggers GitHub Actions for server-side management | Regulated environments, audit trails |
| `log-only`        | Local logging only, no external integration            | Offline or air-gapped environments   |

> **Note:** Both `github-issue` and `github-workflow` strategies require the [GitHub CLI (
`gh`)](https://cli.github.com/)
> to be installed and authenticated.

#### Server-Side Reviews with GitHub Actions

For teams that need **commit status gates**, **full audit trails**, or **multi-reviewer orchestration**, use the
`github-workflow` strategy. This triggers a GitHub Actions workflow that:

1. Creates review issues (even if someone bypasses the CLI)
2. Sets commit statuses (`pending` → `success`) for deploy gating
3. Handles multi-reviewer consensus automatically

To set up:

1. Copy `.github/workflows/nbr-review.yml.example` to `.github/workflows/nbr-review.yml`
2. Configure your `.tbdflow.yml`:

```yaml
review:
  enabled: true
  strategy: github-workflow
  workflow: nbr-review.yml
  default_reviewers:
    - teammate-username
```

3. Run `tbdflow review --trigger` and the workflow handles the rest

### 6. `task` and `note`

Think of these as your development scratch pad. Start a task, jot down what you're trying and why, and let the
commit pick it all up when you're ready.

**Usage:**

```bash
tbdflow task start <description>   # Start a named task
tbdflow task show                  # Show current task and notes
tbdflow task clear                 # Discard the intent log

tbdflow note <message>             # Log a note
tbdflow + <message>                # Shorthand alias
tbdflow n <message>                # Shorthand alias
```

**Options (`note`):**

| Flag   | Description                                         | Required |
|--------|-----------------------------------------------------|----------|
| --show | Show the current intent log instead of adding a note | No       |

**Examples:**

```bash
# Start a task and leave breadcrumbs
tbdflow task start "Refactor auth module"
tbdflow + "tried decorator pattern, too much boilerplate"
tbdflow + "simple middleware chain works better"

# View what you've captured
tbdflow task show

# Notes are automatically included when you commit
tbdflow commit -t refactor -s auth -m "simplify auth middleware"
# The commit body will contain:
#   Intent Log:
#   - tried decorator pattern, felt too verbose
#   - simple middleware chain works better
```

### 7. `radar`

Scans active remote branches for overlapping work that may cause merge conflicts with your local changes. This is the
**social coding safety net** for Trunk-Based Development, it makes the invisible visible by showing who else is
touching the same files before you push.

In TBD, everyone integrates frequently. The biggest fear is two people editing the same lines simultaneously. Standard
Git won't warn you until you try to push. Radar warns you *before* you commit.

**Usage:**

```bash
tbdflow radar
```

**Detection Levels** (configurable in `.tbdflow.yml`):

| Level  | What it checks                        | Speed        |
|--------|---------------------------------------|--------------|
| `file` | Same files touched (default)          | ~5ms/branch  |
| `line` | Overlapping line ranges in same files | ~50ms/branch |

**Example output:**

```
OVERLAP DETECTED with 1 active branch(es):

  feat/API-42-user-auth (by @alice, 2 commits ahead)
  ├── src/auth/handler.rs    LINE OVERLAP
  └── src/auth/middleware.rs  SAME FILE

  3 other active branch(es) have no overlap with your changes.

Hint: Coordinate with the overlapping author(s) before pushing.
```

**Integration:**

Radar is also integrated into other commands:

* **`tbdflow sync`** automatically shows a one-liner warning if overlap is detected.
* **`tbdflow commit`** optionally warns or prompts for confirmation before committing (configurable).

**Configuration:**

```yaml
radar:
  enabled: true
  level: file          # file | line
  on_sync: true        # Show warnings during tbdflow sync
  on_commit: warn      # off | warn | confirm
  ignore_patterns: # Files to exclude from overlap detection
    - "*.lock"
    - "*-lock.*"
    - "CHANGELOG.md"
```

### 8. Pre-flight CI check

When enabled, `tbdflow sync` checks the CI status of the trunk (via the `gh` CLI) **before** pulling.
If the trunk is red or pending, you get a prompt instead of blindly pulling a broken build.

**Configuration:**

```yaml
ci_check:
  enabled: true   # default: false
```

**Behaviour:**

| Trunk CI status | What happens                                            |
|-----------------|---------------------------------------------------------|
| Green           | Silent proceed, prints a brief confirmation            |
| Failed          | Warns and prompts: "Continue with sync? (y/N)"          |
| Pending         | Informs and prompts: "Pull anyway? (y/N)"               |
| Unknown         | Proceeds silently (e.g. `gh` not installed, no CI runs) |

> Requires the [GitHub CLI](https://cli.github.com/) (`gh`) to be installed and authenticated.

### 9. Utility commands

Not part of the core workflow, but handy for checking on things:

**Examples:**

```bash
# Does a pull, shows latest changes to main branch, and warns about stale branches.
# If ci_check is enabled, checks trunk CI status first.
tbdflow sync

# Inspect your current configuration
tbdflow info

# Checks the status of the working dir
tbdflow status

# Shows the current branch name
tbdflow current-branch

# Explicitly checks for local branches older than one day.
tbdflow check-branches

# Checks for a new version of tbdflow and updates it if available.
tbdflow update
```

#### `undo`

In TBD, the rule is simple: if the trunk breaks, fix it or revert it immediately. `tbdflow undo` is a smart wrapper
around `git revert` that syncs with the remote, verifies the commit is on the trunk, cleanly reverts it, and pushes,
all in one command.

**Usage:**

```bash
tbdflow undo <sha> [options]
```

**Options:**

| Flag      | Description                                       | Required |
|-----------|---------------------------------------------------|----------|
| --no-push | Create the revert commit locally without pushing. | No       |

**Examples:**

```bash
# Revert a specific commit on the trunk
tbdflow undo abc1234

# Revert locally without pushing (e.g. to inspect the result first)
tbdflow undo abc1234 --no-push

# Preview what would happen without making changes
tbdflow --dry-run undo abc1234
```

### 10. Advanced Usage

#### Shell Completion

Add tab-completion to your shell:

For Zsh (`~/.zshrc`):

```bash
eval "$(tbdflow generate-completion zsh)"
```

For Bash (`~/.bashrc`):

```bash
eval "$(tbdflow generate-completion bash)"
```

For Fish (`~/.config/fish/config.fish`):

```bash
tbdflow generate-completion fish | source
```

#### Man Page

```bash
tbdflow generate-man-page > tbdflow.1 && man tbdflow.1
```

## IDE support

`tbdflow` comes with IDE support for:

- [IntelliJ](https://github.com/cladam/tbdflow/tree/main/plugins/intellij)
- [VS Code](https://github.com/hekonsek/tbdflow-vscode-extension)

## Contributing

First off, thank you for considering contributing to `tbdflow`! ❤️

Please feel free to open an issue or submit a pull request.
