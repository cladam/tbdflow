<div align="center">

# tbdflow

[![Crates.io](https://img.shields.io/crates/v/tbdflow.svg)](https://crates.io/crates/tbdflow)
[![Downloads](https://img.shields.io/crates/d/tbdflow.svg)](https://crates.io/crates/tbdflow)

</div>

## tbdflow, a Trunk-Based Development CLI

`tbdflow` is a lightweight command-line tool that helps you (and your team) stay in flow with Trunk-Based Development (TBD).

This CLI supports both the default commit-to-main workflow and the structured handling of short-lived branches for features, releases, and hotfixes.

## Status & history

This project is the result of an iterative development journey. It began as an F# application (`tbdflow-fs`) which was a great learning exercise in functional programming.

The current and actively developed version is the Rust implementation (`tbdflow-rs`). It was ported to Rust to create a leaner, faster, and more portable single-binary executable, making it easier for others to use and contribute to. The F# version is no longer maintained but remains in the repository as a functional prototype.

## Philosophy

This tool is built around a specific philosophy of Trunk-Based Development:

* **Main is the default.** The `commit` command is your everyday go-to. It automates pulling the latest changes, committing, and pushing directly to `main`, promoting small, frequent integrations.
* **Branches are the exception.** While feature, release, and hotfix branches are supported, they’re treated as short-lived exceptions and not the norm.
* **Cleanup is automatic.** The complete command enforces branch short-livedness by merging and automatically tagging (release/hotfix) and deleting completed branches, helping keep your repo tidy.
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

### Configuration
`tbdflow` is configurable via two optional files in the root of your repository. To get started quickly, run `tbdflow init` to generate default versions of these files.

`.tbdflow.yml`
This file controls the core workflow of the tool. You can customize:
* The name of your main branch (e.g., main, trunk).
* Branch name prefixes (e.g. feat- instead of feature_).
* The threshold for stale branch warnings.
* Automatic tagging formats.

`.dod.yml`
This file controls the interactive Definition of Done checklist for the commit command. 

### The Definition of Done (DoD) Check
To move beyond just automating process, `tbdflow` integrates an optional pre-commit quality check. If a `.dod.yml` file is present in your repository, the commit command will present an interactive checklist to ensure your work meets the team's agreed-upon standards.

**Example** `.dod.yml`:

```
# .dod.yml in your project root

# If true, requires an --issue <ID> flag on the commit command.
issue_reference_required: true

checklist:
  - "All relevant automated tests pass successfully."
  - "New features or fixes are covered by new tests."
  - "Security implications of this change have been considered."
  - "Relevant documentation (code comments, READMEs) is updated."
```

If you try to proceed without checking all items, the tool will offer to add a TODO list to your commit message footer, ensuring the incomplete work is tracked directly in your Git history.

---

## Global options

| Flag        | Description                                              | Required |
|-------------|----------------------------------------------------------|----------|
| --verbose   | Prints the underlying Git commands as they are executed. | No       |

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
| -m   | --message               | The descriptive commit message.                          | Yes      |
| -b   | --breaking              | Mark the commit as a breaking change.                    | No       |
|      | --breaking-description  | Provide a description for the BREAKING CHANGE: footer.   | No       |
|      | --tag                   | Optionally add and push an annotated tag to this commit. | No       |
|      | --no-verify             | Bypass the interactive DoD checklist.                    | No       |

**Example:**

```bash
# A new feature
tbdflow commit -t "feat" -s "auth" -m "Add password reset endpoint"

# A bug fix with a breaking change
tbdflow commit -t "fix" -m "Correct user permission logic" -b
tbdflow commit -t "refactor" -m "Rename internal API" --breaking --breaking-description "The `getUser` function has been renamed to `fetchUser`."

# A bug fix with a new tag
tbdflow commit -t "fix" -m "Correct user permission logic" --tag "v1.1.1"
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

**Options (release):**

| Flag | Option        | Description                                    | Required |
|------|---------------|------------------------------------------------|----------|
| -f   | --from-commit | Optional commit hash on `main` to branch from. | No       |


**Examples:**

```bash
# Create a feature branch
tbdflow feature -n "user-profile-page"

# Create a release branch
tbdflow release -v "2.1.0"

# Create a release branch from a specific commit
tbdflow release -v "2.1.0" -f "39b68b5"

# Create a hotfix branch
tbdflow hotfix -n "critical-auth-bug"
```

### 3. complete

Merges a short-lived branch back into main, then deletes the local and remote copies of the branch.

**Automatic Tagging:**

* When completing a release branch, a tag (e.g., v1.2.0) is automatically created and pushed.
* When completing a hotfix branch, a tag (e.g., hotfix/name-of-fix) is automatically created and pushed.

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

# Complete a release branch (this will be tagged v2.1.0)
tbdflow complete -t "release" -n "2.1.0"
```

### 4. Misc commands

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
```

