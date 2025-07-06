# Trunk-Based Development CLI

A simple, powerful command-line interface (CLI) to streamline your Git workflow, especially for teams practicing Trunk-Based Development (TBD).

This tool provides automated commands for both the primary "commit-to-main" workflow and the structured management of short-lived branches for features, releases, and hotfixes.

## Philosophy

This tool is designed with a specific Trunk-Based Development philosophy in mind:

* **Commit to Main is the Default:** The primary `commit` command is your day-to-day workhorse. It automates the process of pulling the latest changes, committing, and pushing directly to the `main` branch, encouraging small, frequent integrations.
* **Branches are Exceptions, Not the Rule:** Branches are supported but are treated as short-lived exceptions for specific scenarios (larger features, release stabilization, hotfixes).
* **Automated Cleanup:** The `complete` command enforces the "short-lived" nature of branches by automating the merge-and-delete process, keeping your repository clean.
* **Conventional Commits:** The tool encourages the use of Conventional Commits for clear, consistent commit messages.

## Installation

You need [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.

**Build from source:**
```bash
git clone https://github.com/ingka-group-digital/tbdflow.git
cd tbdflow
cargo install --path .
```

---

## Commands

### 1. `commit`

Commits staged changes directly to the `main` branch with a Conventional Commits message. This is the primary command for daily work.

**Usage:**
```bash
tbdflow commit [options]
```
**Options:**

| Flag | Option      | Description                                   | Required |
|------|-------------|-----------------------------------------------|----------|
| -t   | --type      | The type of commit (e.g., feat, fix, chore).  | Yes      |
| -s   | --scope     | The scope of the changes (e.g., api, ui).     | No       |
| -m   | --message   | The descriptive commit message.               | Yes      |
| -b   | --breaking  | Mark the commit as a breaking change.         | No       |

**Example:**
```bash
# A new feature
tbdflow commit -t "feat" -s "auth" -m "Add password reset endpoint"

# A bug fix with a breaking change
tbdflow commit -t "fix" -m "Correct user permission logic" -b
```

### 2.`feature` / `release` / `hotfix`

Creates a new, short-lived branch from the latest version of `main.

**Usage:**
```bash
# For features or hotfixes
tbdflow <feature|hotfix> --name <branch-name>

# For releases
tbdflow release --version <version-number> [options]
```

`release` ** Command Options:**

| Flag | Option        | Description                                    | Required |
|------|---------------|------------------------------------------------|----------|
| -f   | --from-commit | Optional commit hash on `main` to branch from. | No       |


**Examples:**

```bash
# Create a feature branch
tbdflow feature -n "user-profile-page"

# Create a release branch
tbdflow release -v "2.1.0"

# Create a hotfix branch
tbdflow hotfix -n "critical-auth-bug"
```

### 3. complete

Merges a short-lived branch back into main, then deletes the local and remote copies of the branch.

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
tbdflow complete -t "feature" -n "user-profile-page"

# Complete a release branch
tbdflow complete -t "release" -n "2.1.0"
```
