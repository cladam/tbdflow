---
name: tbdflow
description: Manage Trunk-Based Development workflows using the tbdflow CLI. Use this skill to create short-lived branches, make standardised commits, sync with trunk, merge completed work, and generate changelogs.
version: 1.0.0
author: Claes Adamsson
tags:
  - git
  - trunk-based-development
  - devops
  - cli
---

# tbdflow Skill

## Overview

This skill enables an AI agent to manage a **Trunk-Based Development (TBD)** workflow using the `tbdflow` CLI (v0.18.2).

The skill exists to:

- Enforce short-lived branches
- Standardise commits
- Reduce Git decision-making
- Maintain a fast, safe path back to trunk (`main`)

`tbdflow` is the **only interface** the agent should use for Git workflow actions covered by this skill.

---

## When to Use

Use this skill when the user wants to:

- Start work on a task, ticket, or feature
- Commit staged changes
- Sync with trunk or check repository status
- Merge completed work back to `main`
- See what has changed since the last release
- Log a design decision, failed attempt, or reasoning pivot

Typical trigger phrases include:

- "Start working on…"
- "Commit this"
- "Merge my work"
- "Sync me up"
- "What's new?"
- "Note that…" / "Log this decision"

---

## When *Not* to Use

Do **not** use this skill to:

- Create or manage long-lived branches
- Perform manual Git commands outside `tbdflow`
- Rewrite commit history
- Perform interactive rebases
- Merge without explicit user intent

If an action cannot be performed via `tbdflow`, explain the limitation instead of falling back to raw Git commands.

---

## Prerequisites

Before invoking any `tbdflow` command, the agent **must** verify that the CLI is installed and accessible.

### Installation Check

Run the following to confirm availability:

```bash
command -v tbdflow && tbdflow --version
```

If `tbdflow` is **not found**, the agent must attempt to install it automatically using one of the strategies below.

### Installation Strategies (in order of preference)

**1. Cargo Install (requires Rust toolchain)**

If `cargo` is available on the system:

```bash
cargo install tbdflow
```

This downloads the latest release from [crates.io](https://crates.io/crates/tbdflow) and compiles it locally.

**2. GitHub Releases (pre-built binary)**

If `cargo` is not available but `curl` is:

```bash
curl -fsSL https://github.com/cladam/tbdflow/releases/latest/download/tbdflow-$(uname -m)-$(uname -s | tr '[:upper:]' '[:lower:]') -o /usr/local/bin/tbdflow
chmod +x /usr/local/bin/tbdflow
```

Adjust the binary path if `/usr/local/bin` is not writable (e.g. use `~/.local/bin`).

**3. Manual Prompt**

If neither strategy is viable, inform the user:

> `tbdflow` is not installed. Please install it using one of:
> - `cargo install tbdflow`
> - Download a binary from https://github.com/cladam/tbdflow/releases
>
> See the README for details.

### Post-Install Verification

After installation, always confirm:

```bash
tbdflow --version
```

If the version is outdated, suggest:

```bash
tbdflow update
```

---

## Instructions

Follow the instructions below exactly. Each capability defines intent, constraints, and decision rules.

---

### 1. Standardised Committing

**Intent**
Create a structured, conventional commit on trunk or a short-lived branch.

**Staging Behaviour**

* `tbdflow` automatically stages the relevant changes when committing
* The agent must **not** run `git add` or any raw Git staging commands
* No explicit staging step is required from the user or the agent

**Preconditions**

* The working tree contains changes intended for commit
* No unresolved merge conflicts are present

**Command**

```bash
tbdflow commit -t <type> [-s <scope>] -m "<message>" [--issue <issue>] [-b]
```

**Decision Rules**

* Allowed commit types:

    * `feat`, `fix`, `chore`, `docs`, `refactor`, `test`, `build`, `ci`, `perf`, `revert`, `style`
* Never invent new commit types
* If no type is specified:

    * Default to `chore` unless behaviour changes
* DoD Checklist: If a `.dod.yml` file exists in the project root and `--no-verify` is not passed, an interactive
  checklist will appear. Unchecked items will result in a `TODO:` footer being appended to the commit message.
* Use `-b` / `--breaking` if the change introduces breaking behaviour
* Use `--issue` when the user references a ticket ID (JIRA, GitHub, etc.)

**Use This When**

* The user says “commit”, “save this”, or “check this in”
* The user describes completed work ready to be recorded

---

### 2. Creating Short-Lived Branches

**Intent**
Start a new unit of work in a short-lived branch that will be merged back to trunk quickly.

**Preconditions**

* Working tree is clean or safely stashed
* The task is not exploratory or long-running

**Command**

```bash
tbdflow branch -t <type> -n <name> [--issue <issue>] [-f <from_commit>]
```

**Decision Rules**

* Branch naming follows:

    * `<type>/<name>` or
    * `<type>/<issue>-<name>`
* If the user provides a task description:

    * Slugify it for the `-n` parameter
* Use `--issue` when a ticket ID is available
* Use `-f` only if the user explicitly asks to branch from a non-HEAD commit

**Use This When**

* The user says “start working on…”
* A task requires isolation before merging to `main`

---

### 3. Workflow Completion & Integration

**Intent**
Safely merge completed work back into trunk and clean up the branch.

**Preconditions**

* All intended commits are complete
* The branch is ready to be merged

**Command**

```bash
tbdflow complete -t <type> -n <name>
```

**Decision Rules**

* Infer `<type>` and `<name>` from:

    1. The current Git branch name
    2. Fallback: the most recent `tbdflow branch` invocation
* The merge is performed using `--no-ff`
* Both local and remote branch copies are deleted after merge

**Use This When**

* The user says “I’m done”, “merge my work”, or “ship this”

---

### 4. Syncing & Status

**Intent**
Keep the local workspace aligned with trunk and provide situational awareness.

**Commands**

```bash
tbdflow sync
tbdflow status
```

**Decision Rules**

* Use `sync` to:

    * Pull and rebase from remote
    * Inspect recent history
    * Identify stale branches
* Use `status` to:

    * Show context-aware Git status, In monorepos, this excludes sub-project directories when at the root.
    * Handle monorepos correctly

**Use This When**

* The user says "sync", "catch me up", or "what's happening"
* Before committing, merging, or starting new work

---

### 5. Radar — Overlap Detection

**Intent**
Proactively detect potential merge conflicts by scanning active remote branches for overlapping work with local changes.
The social coding safety net for TBD.

**Commands**

```bash
tbdflow radar
```

**Decision Rules**

* Use `radar` to:

    * Scan all active (unmerged) remote branches
    * Compare their diffs against local uncommitted/staged changes
    * Show who is working on overlapping files (and optionally overlapping lines)
    * Provide actionable social coordination hints

* Radar is also integrated into:

    * `tbdflow sync` — shows a one-liner warning if overlap is detected
    * `tbdflow commit` — optionally warns or prompts for confirmation

**Detection Levels**

| Level  | What it checks                        | Speed        |
|--------|---------------------------------------|--------------|
| `file` | Same files touched (default)          | ~5ms/branch  |
| `line` | Overlapping line ranges in same files | ~50ms/branch |

**Configuration (`.tbdflow.yml`)**

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

**Use This When**

* The user says "anyone else working on this?", "check for conflicts", or "radar"
* Before pushing to avoid merge hell
* When collaborating closely with teammates on trunk

---

### 6. Undo — The Panic Button

**Intent**
Immediately revert a broken commit on trunk, restoring it to a green state. In TBD, if trunk breaks, you fix it or
revert it — there is no middle ground.

**Command**

```bash
tbdflow undo <sha> [--no-push]
```

**Preconditions**

* The working tree is clean (no uncommitted changes)
* The commit SHA exists and is on the main branch

**Decision Rules**

* `undo` will:

    * Sync with remote (fast-forward only) before reverting
    * Verify the commit exists and is on trunk
    * Create a revert commit using `git revert --no-edit`
    * Push the revert to the remote (unless `--no-push`)
* Use `--no-push` when the user wants to inspect the revert locally before pushing
* The reverted changes remain in Git history and can be re-applied later
* This command only works on commits that are on the main branch

**Use This When**

* The user says "revert this", "undo that commit", or "trunk is broken"
* A commit on trunk caused a build failure, test regression, or production incident
* The fastest path to green is reverting rather than fixing forward

---

### Pre-Commit Workflow

**Always run `tbdflow sync` before `tbdflow commit`.**

The `sync` command:

* Pulls and rebases from remote
* Shows current status (wraps `git status`)
* Ensures the workspace is aligned with trunk

This prevents conflicts and ensures commits are based on the latest trunk state.

---

## Validation & Linting Behaviour

`tbdflow` enforces workflow correctness using an internal linter. The agent must understand and respect these rules.

---

### Commit Message Rules

#### Subject Line (`-m` message)

| Rule           | Requirement                                                                                                  | Example                              |
|----------------|--------------------------------------------------------------------------------------------------------------|--------------------------------------|
| Max Length     | 72 characters                                                                                                | `"add user profile"` ✓               |
| Capitalisation | Must not start with a capital letter                                                                         | `"add feature"` ✓, `"Add feature"` ✗ |
| Punctuation    | Must not end with a period                                                                                   | `"fix bug"` ✓, `"fix bug."` ✗        |
| Type           | Must be one of: `feat`, `fix`, `chore`, `docs`, `refactor`, `test`, `build`, `ci`, `perf`, `revert`, `style` | `feat` ✓, `feature` ✗                |
| Scope          | Optional, lowercase, no spaces                                                                               | `-s login` ✓, `-s "user login"` ✗    |
| Message        | Required, non-empty, imperative mood                                                                         | `"add user profile"` ✓, `""` ✗       |
| Breaking       | Must use `-b` flag if breaking change                                                                        | `-b` for breaking ✓                  |

#### Commit Body (Optional)

| Rule        | Requirement                                    |
|-------------|------------------------------------------------|
| Line Length | Each line must not exceed 80 characters        |
| Separation  | Must be separated from subject by a blank line |

#### Issue Key (`--issue`)

| Rule   | Requirement                         | Example                    |
|--------|-------------------------------------|----------------------------|
| Format | Uppercase project key, dash, number | `PROJ-123` ✓, `proj-123` ✗ |

---

### Branch Name Rules

| Rule  | Requirement                            | Example                                    |
|-------|----------------------------------------|--------------------------------------------|
| Type  | Must match commit types                | `feat/` ✓, `feature/` ✗                    |
| Name  | Lowercase, hyphen-separated, no spaces | `add-user-profile` ✓, `Add User Profile` ✗ |
| Issue | Optional, prefixed to name             | `feat/API-456-add-user` ✓                  |

---

### Error Handling

If `tbdflow` rejects input:

1. Read the error message carefully
2. Correct the input based on the rules above
3. Retry with valid parameters
4. Do **not** fall back to raw Git commands

The agent should prefer generating valid inputs over relying on linter errors.

---

### 7. Breadcrumbs — Low-Friction Intent Logging

**Intent**
Capture architectural decisions, failed attempts, or logic pivots during the development process before the final
commit.

Breadcrumbs provide a lightweight way to document the *why* behind code changes — the reasoning that would otherwise be
lost between keystrokes.

**Command**

```bash
tbdflow + "<breadcrumb_message>"
```

**Decision Rules**

* Use this whenever you pivot your approach (e.g. switching from a Factory to a Trait)
* Use this to document a failed attempt that explains why the final code looks the way it does
* Use this to explain complex logic, patterns, or non-obvious trade-offs
* Do **not** use this for trivial changes (typos, formatting)
* These notes are automatically appended to the next `tbdflow commit` body
* Multiple breadcrumbs accumulate and are included in order

**Storage & Safety**

Breadcrumbs are stored locally in `.tbdflow-intent.json` at the repository root. This file is **never committed** — it
is consumed and deleted after the next `tbdflow commit`.

`tbdflow` automatically ensures `.tbdflow-intent.json` is listed in `.gitignore` the first time a breadcrumb is saved.
This prevents the `git add .` staging pattern from accidentally committing raw intent data.

If `.gitignore` does not exist, it will be created with the entry. If it already contains the entry, no changes are
made.

**Decision Rules: The Intent Log**

Breadcrumbs are not optional decoration — they are the agent's **audit trail**.

* **Log your struggle**: if you try an approach and it fails or feels wrong, log it with `tbdflow +`.
  This gives the human auditor context on what you *didn't* do and why.
* **Vibe-coding transparency**: do not just push a final "perfect" diff.
  Use breadcrumbs to show the evolution of the logic — the dead ends, the trade-offs, the pivots.
* **Minimum expectation**: before committing a complex refactor or non-trivial feature, there should be
  at least 1–2 breadcrumbs explaining the major structural decisions.
* **Observability over perfection**: a clean diff with zero context is harder to review than a
  slightly messier one with a clear reasoning trail.

> *"Velocity is nothing without observability. The trunk is live, the struggle is logged, the audit is parallel."*

**Agent Behaviour**

* **Think out loud**: use `tbdflow +` to document your chain of thought during development
* Prefer clarification over assumptions for destructive actions
* Default to the safest operation that preserves trunk integrity
* Treat breadcrumbs as a first-class part of the workflow, not an afterthought

**Examples**

| Scenario                         | Command                                                                                          |
|----------------------------------|--------------------------------------------------------------------------------------------------|
| Switching architectural pattern  | `tbdflow + "switched from Factory to Trait: Factory felt over-engineered for this scope"`        |
| Documenting a rejected edge-case | `tbdflow + "decided against async here: the overhead outweighs the benefits for this sync task"` |
| Explaining complex regex/logic   | `tbdflow + "using lookahead in regex to handle nested brackets without recursion"`               |

**Use This When**

* The agent or user makes a non-trivial design decision mid-flight
* A failed approach informs the final implementation
* The commit message alone would not explain the reasoning

---

### 8. Changelog Generation

**Intent**
Summarise changes using structured commit history.

**Command**

```bash
tbdflow changelog [--unreleased] [--from <ref>]
```

**Decision Rules**

* Use `--unreleased` when the user asks "What's new?"
* Use `--from <ref>` when comparing against a specific tag or version

**Use This When**

* The user asks for release notes
* The user wants a summary of recent changes

---

## Output Format

* Commands should be executed directly
* Explanations should be concise and factual
* Avoid narrating Git internals unless asked
* Prefer showing the command being run before or alongside results

---

## Examples

| User Input                                    | Action                                                                                    |
|-----------------------------------------------|-------------------------------------------------------------------------------------------|
| "Commit this as a bug fix for login."         | `tbdflow commit -t fix -s login -m "resolve timeout issue"`                               |
| "Start working on API-456: Add user profile." | `tbdflow branch -t feat -n add-user-profile --issue API-456`                              |
| "Merge my current work back to main."         | `tbdflow complete -t <current_type> -n <current_name>`                                    |
| "Sync me up."                                 | `tbdflow sync`                                                                            |
| "Anyone else working on this file?"           | `tbdflow radar`                                                                           |
| "Revert commit abc1234, it broke the build."  | `tbdflow undo abc1234`                                                                    |
| "I switched from Factory to Trait."           | `tbdflow + "switched from Factory to Trait: Factory felt over-engineered for this scope"` |
| "What changed since the last version?"        | `tbdflow changelog --unreleased`                                                          |

---

## Notes

* Treat `main` (trunk) as sacred
* Prefer safety and clarity over cleverness
* Ask for clarification only when an action could be destructive

