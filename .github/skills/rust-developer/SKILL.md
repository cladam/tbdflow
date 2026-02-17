---
name: rust-developer
description: Lead Rust developer guidance for Rust 2021+, Clap CLI applications, error handling with anyhow/thiserror, and idiomatic patterns.
version: 1.0.0
author: Claes Adamsson
tags:
  - rust
  - cli
  - systems
  - error-handling
  - architecture
---

# Rust Developer Skill

## Overview

Use this skill when acting as a Lead Rust Developer on the tbdflow codebase. It targets Rust 2021 edition, Clap 4.x for
CLI argument parsing, anyhow/thiserror for error handling, and serde for serialization. The goal is to ensure idiomatic
Rust, safe and ergonomic APIs, and pragmatic application code that stays aligned with the TBD workflow.

## When to Use

- Implementing or reviewing Rust CLI features, commands, or module structure.
- Designing error handling strategies, Result chains, or custom error types.
- Producing configuration structs, validators, or parsing constructs in Rust.
- Guiding architectural or style decisions for CLI tools within this repo.

## When Not to Use

- Web frontend, WASM UI, or non-Rust services.
- Infrastructure-as-code, Terraform, or pure documentation tasks.
- Situations where another specialised skill (e.g., `tbdflow` workflow) already governs the process.

## Instructions

### General Guidance

- Always read `Cargo.toml` before suggesting dependencies or versions.
- Use idiomatic Rust: ownership, borrowing, lifetimes, pattern matching, and iterators.
- Default to immutable bindings (`let`) and leverage the type system for safety.
- Avoid `.unwrap()` and `.expect()` in library code; use `?` operator and proper error propagation.
- Prefer `Result<T, E>` over panics; reserve panics for truly unrecoverable states.
- Treat tests as first-class: unit tests in modules, integration tests in `tests/`.

### Preferred Practices

- Follow Clap derive conventions; keep command handlers thin and delegate to domain modules.
- Use the module system effectively: `mod.rs` or inline modules, clear public API boundaries.
- Leverage iterators and combinators (`.map()`, `.filter()`, `.collect()`) over manual loops.
- Keep functions/files concise; split modules when responsibilities grow complex.
- Use `cargo clippy` and `cargo fmt` for linting and formatting.

### Patterns to Follow

- **Error Handling**: Use `thiserror` for custom error types in libraries, `anyhow` for applications.
  ```rust
  // Library error with thiserror
  #[derive(Error, Debug)]
  pub enum GitError {
      #[error("Git command failed: {0}")]
      CommandFailed(String),
      #[error("Branch '{0}' not found")]
      BranchNotFound(String),
  }
  
  // Application code with anyhow
  fn main() -> anyhow::Result<()> {
      let config = load_config().context("Failed to load configuration")?;
      Ok(())
  }
  ```

- **Structs & Enums**: Use `#[derive]` liberally for common traits.
  ```rust
  #[derive(Debug, Clone, Serialize, Deserialize, Default)]
  pub struct Config {
      pub name: String,
      #[serde(default)]
      pub enabled: bool,
  }
  ```

- **Option/Result Handling**: Prefer combinators and `?` over explicit matching.
  ```rust
  // Preferred
  let name = config.name.as_ref().map(|s| s.trim()).unwrap_or("default");
  
  // Also good with ?
  let value = some_option.ok_or_else(|| anyhow!("Missing value"))?;
  ```

- **CLI Structure**: Use Clap derive with clear subcommands and help text.
  ```rust
  #[derive(Parser)]
  #[command(name = "mytool", about = "Does useful things")]
  struct Cli {
      #[command(subcommand)]
      command: Commands,
      #[arg(long, global = true)]
      verbose: bool,
  }
  ```

- **Testing**: Use `#[cfg(test)]` modules and descriptive test names.
  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;
      
      #[test]
      fn parse_config_with_defaults() {
          let config: Config = serde_yaml::from_str("name: test").unwrap();
          assert!(!config.enabled);
      }
  }
  ```

### Patterns to Avoid

- **Unwrap Abuse**: Avoid `.unwrap()` except in tests or when invariants are proven.
- **Clone Overuse**: Don't `.clone()` to satisfy the borrow checker without understanding why.
- **Stringly Typed**: Use enums and newtypes instead of raw strings for domain concepts.
- **God Modules**: Avoid modules with 500+ lines; split into focused submodules.
- **Ignoring Clippy**: Address warnings; they often catch real issues.
- **Blocking in Async**: If using async, don't call blocking APIs without `spawn_blocking`.

### Tooling & Dependency Checks

- Confirm crate versions from `Cargo.toml` and `Cargo.lock` before adding dependencies.
- Prefer well-maintained crates from the ecosystem (check crates.io downloads, recent updates).
- Use `cargo test` for unit/integration tests; `cargo test --doc` for doc tests.
- Maintain test coverage on critical logic; structure tests with Arrange-Act-Assert.
- Run `cargo clippy -- -W clippy::pedantic` periodically for deeper analysis.
- Use `cargo audit` to check for security vulnerabilities in dependencies.

### Output Expectations

- Document public APIs with doc comments (`///`) focusing on usage and rationale.
- Include examples in doc comments for complex functions.
- Keep guidance inclusive, action-oriented, and practical.
- When suggesting new dependencies, justify the addition and check for existing alternatives.

### Module Organization (tbdflow example)

```
src/
├── lib.rs          # Public module exports
├── main.rs         # CLI entry point, command dispatch
├── cli.rs          # Clap definitions (Commands enum)
├── config.rs       # Configuration structs and loading
├── git.rs          # Git operations (shell out to git)
├── commit.rs       # Commit workflow logic
├── branch.rs       # Branch management
├── review.rs       # Review feature
└── wizard.rs       # Interactive prompts
```

- Keep `main.rs` thin: parse args, load config, dispatch to handlers.
- Domain logic lives in dedicated modules (`commit.rs`, `branch.rs`).
- Shared utilities go in `misc.rs` or a `utils/` submodule.

## Notes

- This skill complements, but does not override, the `tbdflow` Git workflow skill—use both where applicable.
- If unsure about a pattern or crate, design a small experiment first (write a test) before implementing.
- Consult [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) for naming and design decisions.
- Reference [The Rust Book](https://doc.rust-lang.org/book/) for fundamentals.
