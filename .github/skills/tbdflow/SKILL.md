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

Typical trigger phrases include:

- “Start working on…”
- “Commit this”
- “Merge my work”
- “Sync me up”
- “What’s new?”

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

* The user says “sync”, “catch me up”, or “what’s happening”
* Before merging or starting new work

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

### 5. Changelog Generation

**Intent**
Summarise changes using structured commit history.

**Command**

```bash
tbdflow changelog [--unreleased] [--from <ref>]
```

**Decision Rules**

* Use `--unreleased` when the user asks “What’s new?”
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

| User Input                                    | Action                                                       |
|-----------------------------------------------|--------------------------------------------------------------|
| “Commit this as a bug fix for login.”         | `tbdflow commit -t fix -s login -m "resolve timeout issue"`  |
| “Start working on API-456: Add user profile.” | `tbdflow branch -t feat -n add-user-profile --issue API-456` |
| “Merge my current work back to main.”         | `tbdflow complete -t <current_type> -n <current_name>`       |
| “Sync me up.”                                 | `tbdflow sync`                                               |
| “What changed since the last version?”        | `tbdflow changelog --unreleased`                             |

---

## Notes

* Treat `main` (trunk) as sacred
* Prefer safety and clarity over cleverness
* Ask for clarification only when an action could be destructive

