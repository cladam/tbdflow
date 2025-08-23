<div align="center">

# tbdflow

[![Crates.io](https://img.shields.io/crates/v/tbdflow.svg)](https://crates.io/crates/tbdflow)
[![Downloads](https://img.shields.io/crates/d/tbdflow.svg)](https://crates.io/crates/tbdflow)

</div>

## tbdflow, a Trunk-Based Development CLI

`tbdflow` is a lightweight command-line tool that helps you (and your team) stay in flow with Trunk-Based Development (TBD).

This CLI supports both the default commit-to-main workflow and the structured handling of short-lived branches for features, releases, and hotfixes.

![A terminal running the command tbdflowlow](docs/commit-demo.gif "A demo of tbdflow running commit-to-main commands")

## Philosophy

This tool is built around a specific philosophy of Trunk-Based Development:

* **Main is the default.** The `commit` command is your everyday go-to. It automates pulling the latest changes, committing, and pushing directly to `main`, promoting small, frequent integrations.
* **Branches are the exception.** While branches are supported, they’re treated as short-lived exceptions and not the norm.
* **Cleanup is automatic.** The complete command enforces branch short-livedness by merging and automatically tagging (release) and deleting completed branches, helping keep your repo tidy.
* **Conventional Commits encouraged.** Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/) for clarity and consistency.

### Why not just use Git?

This CLI isn’t a replacement for Git. You’ll still reach for raw `git` when doing advanced work like rebasing, cherry-picking, or running `git bisect`.

This tool is as a **workflow assistant**, `tbdflow` encapsulates a repeatable, opinionated process to support your day-to-day development.

It offers three main benefits:

1. **Consistency across the team**
   Everyone follows the same steps for common tasks. Commits, branches, and releases are handled the same way every time, keeping your Git history clean and predictable.

2. **Less to remember**
   No need to recall the exact flags or sequences (like `pull --rebase`, `merge --no-ff`, or commit message formats). The CLI handles that, so you can stay focused on writing code.

3. **It supports "the TBD way"**
   This tool makes the preferred approach easy by providing a smooth, safe, and efficient path for 80% of everyday tasks. For the other 20%, you can always use Git directly.

### Installation

You need [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.

#### Installing from crates.io

The easiest way to install `tbdflow` is to download it from [crates.io](https://crates.io/crates/tbdflow). You can do it using the following command:

```bash
cargo install tbdflow
```

If you want to update `tbdflow` to the latest version, execute the following command:

```bash
tbdflow update
```

#### Building from source

Alternatively you can build `tbdflow` from source using Cargo:

```bash
git clone https://github.com/cladam/tbdflow.git
cd tbdflow
sudo cargo install --path . --root /usr/local
```

### Configuration
`tbdflow` is configurable via two optional files in the root of your repository. To get started quickly, run `tbdflow init` to generate default versions of these files.

`.tbdflow.yml`
This file controls the core workflow of the tool. You can customise:
- The name of your main branch (e.g. main, trunk).
- Allowed branch types and their prefixes (e.g feat/, chore/)
- A strategy for handling issue references ("branch-name" or "commit-scope")
- The threshold for stale branch warnings.
- Automatic tagging formats.
- Commit message linting rules.

`.dod.yml`
This file controls the interactive Definition of Done checklist for the commit command. 

### Features

#### The Definition of Done (DoD) Check
To move beyond just automating process, `tbdflow` integrates an optional pre-commit quality check. If a `.dod.yml` file is present in your repository, the commit command will present an interactive checklist to ensure your work meets the team's agreed-upon standards.

**Example** `.dod.yml`:

```
# .dod.yml in your project root
checklist:
  - "All relevant automated tests pass successfully."
  - "New features or fixes are covered by new tests."
  - "Security implications of this change have been considered."
  - "Relevant documentation (code comments, READMEs) is updated."
```

If you try to proceed without checking all items, the tool will offer to add a TODO list to your commit message footer, ensuring the incomplete work is tracked directly in your Git history.

#### Commit Message Linting

If a `.tbdflow.yml` file is present and contains a lint section, the commit command will automatically validate your commit message against the configured rules before the DoD check. This provides immediate feedback on stylistic and structural conventions.

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
---

## Global options

| Flag        | Description                                              | Required |
|-------------|----------------------------------------------------------|----------|
| --verbose   | Prints the underlying Git commands as they are executed. | No       |
| --dry-run   | Simulate the command without making any changes.         | No       |

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

| Flag | Option                  | Description                                              | Required |
|------|-------------------------|----------------------------------------------------------|----------|
| -t   | --type                  | The type of commit (e.g., feat, fix, chore).             | Yes      |
| -s   | --scope                 | The scope of the changes (e.g., api, ui).                | No       |
| -m   | --message               | The descriptive commit message (subject line).           | Yes      |
|      | --body                  | Optional multi-line body for the commit message.         | No       |
| -b   | --breaking              | Mark the commit as a breaking change.                    | No       |
|      | --breaking-description  | Provide a description for the 'BREAKING CHANGE:' footer. | No       |
|      | --tag                   | Optionally add and push an annotated tag to this commit. | No       |
|      | --issue                 | Optionally add an issue reference to the footer.         | No       |
|      | --no-verify             | Bypass the interactive DoD checklist.                    | No       |

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

Creates and pushes a new, short-lived branch from the latest version of `main`. This is the primary command for starting new work that isn't a direct commit to `main`.

**Usage:**

```bash
tbdflow branch --type <type> --name <name> [--issue <issue-id>]
```

**Options (release):**

| Flag       | Description                                                                     | Required |
|------------|---------------------------------------------------------------------------------|----------|
| -t, --type | The type of branch (e.g. feat, fix, chore). See .tbdflow.yml for allowed types. | Yes      |
| -n, --name | A short, desriptive name for the branch.                                        | Yes      |
| --issue    | Optional issue reference to include in the branch name or commit scope.         | No       |


**Examples:**

```bash
# Create a simple feature branch named "feat/new-dashboard"
tbdflow branch -t feat -n "new-dashboard"

# Create a fix branch with an issue reference in the name
# (This will be named "fix/PROJ-123-login-bug" by default)
tbdflow branch -t fix -n "login-bug" --issue "PROJ-123"
```

### 3. `complete`

Merges a short-lived branch back into main, then deletes the local and remote copies of the branch.

**Automatic Tagging:**

* When completing a release branch, a tag (e.g. v1.2.0) is automatically created and pushed.

**Usage:**

```bash
tbdflow complete --type <branch-type> --name <branch-name>
```

**Options:**

| Flag | Option   | Description                                             | Required |
|------|----------|---------------------------------------------------------|----------|
| -t   | --type   | The type of branch: feature, release, or hotfix.        | Yes      |
| -n   | --name   | The name or version of the branch to complete.          | Yes      |

**Examples:**

```bash
# Complete a feature branch
tbdflow complete -t feature -n "user-profile-page"

# Complete a release branch (this will be tagged v2.1.0)
tbdflow complete -t release -n "2.1.0"
```

### 4. `changelog`

Generates a changelog in Markdown format from your repository's Conventional Commit history. See `tbdflow` repo for a CHANGELOG.md generated by this command.

**Usage:**

```bash
tbdflow changelog [options]
```

**Options:**

| Option        | Description                                                               |
|---------------|---------------------------------------------------------------------------|
| --unreleased  | Generate a changelog for all commits since the last tag.                  |
| --from        | Generate a changelog for commits from a specific tag.                     |
| --to          | Generate a changelog for commits up to a specific tag (defaults to HEAD). |

**Examples:**

```bash
# Generate a changelog for a new version
tbdflow changelog --from v0.12.0 --to v0.13.0

# See what will be in the next release
tbdflow changelog --unreleased
```

### 5. Utility commands

`tbdflow` has a couple of commands that can be beneficial to use but they are not part of the workflow, they are for inspecting the state of the repository. 

**Examples:**

```bash
# Does a pull, shows latest changes to main branch, and warns about stale branches.
tbdflow sync

# Checks the status of the working dir
tbdflow status

# Shows the current branch name
tbdflow current-branch

# Explicitly checks for local branches older than one day.
tbdflow check-branches

# Checks for a new version of tbdflow and updates it if available.
tbdflow update
```

### 6. Advanced Usage

#### Shell Completion

To make `tbdflow` even faster to use, you can enable shell completion. Add one of the following lines to your shell's configuration file.

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

#### Man Page Generation

You can generate a man page for `tbdflow` by running the following command:

```bash
tbdflow generate-man-page > tbdflow.1
```

### Deprecated Commands

The following commands are deprecated and will be removed in a future version. Please use the new `branch` command instead.

- **`feature`**
- **`release`**
- **`hotfix`**


