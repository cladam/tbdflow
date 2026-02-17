---
name: code-analyzer
description: Code quality analysis skill for tbdflow, covering smells, maintainability, and refactoring guidance.
version: 1.0.0
author: Claes Adamsson
tags:
  - analysis
  - quality
  - refactoring
  - rust
---

# Code Analyzer Skill

## Overview

Use this skill to provide focused code quality reviews for the tbdflow codebase. The goal is to surface maintainability
risks, code smells, and refactoring options with clear, actionable guidance.

## When to Use

- Reviewing a feature, module, or pull request for code quality.
- Assessing technical debt or refactoring scope.
- Identifying maintainability, readability, or performance risks.

## When Not to Use

- Security reviews (use the security skill).
- Architectural discovery or ADRs (use the architect skill).
- Test design or acceptance coverage planning (use the atdd-developer skill).

## Instructions

### Analysis Focus

- Identify code smells and anti-patterns.
- Evaluate complexity, cohesion, and coupling.
- Check consistency with project standards and Rust/Clap CLI conventions.
- Suggest pragmatic, low-risk refactors.
- Cross-check Rust-specific findings including ownership, lifetimes, error handling (`Result`/`anyhow`), and idiomatic
  patterns.

### Rust-Specific Concerns

- **Ownership & Borrowing**: Unnecessary clones, lifetime issues, or overly complex borrow patterns.
- **Error Handling**: Proper use of `Result`, `?` operator, `anyhow` context, and `thiserror` for custom errors.
- **Option Handling**: Avoid excessive `.unwrap()`, prefer `?`, `if let`, or `map`/`and_then`.
- **Trait Usage**: Appropriate use of `Clone`, `Debug`, `Default`, `Serialize`/`Deserialize`.
- **Module Structure**: Clear separation of concerns across modules (`cli`, `config`, `git`, etc.).
- **CLI Patterns**: Proper Clap derive usage, argument conflicts, and help text quality.

### Consultation

- When proposing Rust refactors, consider idiomatic patterns from the Rust API Guidelines.
- If changes affect behaviour, recommend a test update and note any integration test impact.

### Analysis Criteria

- **Readability**: clear naming, simple flows, meaningful doc comments (`///`).
- **Maintainability**: small functions, focused modules/structs, low cyclomatic complexity.
- **Performance**: no obvious bottlenecks, wasteful allocations, or blocking in async contexts.
- **Safety**: avoid `unsafe` unless justified, handle all `Result`/`Option` properly.
- **Best Practices**: pragmatic use of patterns, DRY/KISS, predictable error handling with context.

### Code Smell Signals

- Long functions (about 50+ lines).
- Large modules (about 500+ lines without clear sub-modules).
- Duplicate or dead code.
- Complex conditionals or deeply nested `match`/`if let` chains.
- Excessive `.unwrap()` or `.expect()` without justification.
- Unnecessary `.clone()` calls (ownership issues).
- God modules or structs with too many responsibilities.
- Stringly-typed data instead of enums or newtypes.

### Code Smell Catalogue (Reference)

Use this catalogue as a reference when naming smells and explaining impact. Keep it concise in reports and only expand
when a smell is confirmed.

- **Bloaters**: Large Module, Long Function, Long Parameter List, Data Clump.
- **Change Preventers**: Shotgun Surgery, Divergent Change, Tight Coupling.
- **Couplers**: Feature Envy, Message Chain, Leaky Abstractions.
- **Dispensables**: Dead Code, Duplicate Code, Lazy Module, Unused Dependencies.
- **Rust-Specific**: Clone Abuse, Unwrap Panic Risk, Lifetime Complexity, Missing Error Context.
- **Naming/Clarity**: Uncommunicative Name, Inconsistent Names, Misleading Comments, Poor Module Docs.

### Sources

- Martin Fowler, *Refactoring* (1999/2018)
- Robert C. Martin, *Clean Code* (2008)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)
- [The Rust Book](https://doc.rust-lang.org/book/) - Idiomatic Patterns
- [Error Handling in Rust](https://blog.burntsushi.net/rust-error-handling/)

## Output Expectations

- Provide findings ordered by severity with file and line references.
- Offer concrete refactoring suggestions with minimal disruption.
- Call out positive patterns to reinforce good practice (e.g., proper error handling, clean module boundaries).
- Use a concise report format when the review is extensive.
- Note any Clippy warnings or Rust idiom improvements.
- When refactors are proposed, consider ownership implications and test coverage.

### Devlog

- Save every review in `.github/devlog/YYYY-MM-DD-review.md`.
- Record any refactoring carried out based on review findings in `.github/devlog/YYYY-MM-DD-activity.md`.
- Use this format for activity entries: `[AGENT_NAME]` -> `[ACTION_TAKEN]` -> `[RESULT/LINK]`.

### Preferred Report Shape

```markdown
## Code Quality Review

### Findings

1. [Severity] Issue summary
    - File: path/to/file.rs:line
    - Why it matters: ...
    - Suggested change: ...

### Clippy / Compiler Warnings

- ...

### Positives

- ...

### Risks / Follow-ups

- ...
```

## Notes

- Keep the tone clear, inclusive, and action-oriented.
- Prefer evidence-based observations over speculation.
- When unsure, propose a small experiment to validate the issue.
- If a refactor changes behaviour, recommend an acceptance test update or new test first.
