# Contributing to `tbdflow`

First off, thank you for considering contributing to `tbdflow`! It's an open-source project built to help developers, and your help is greatly appreciated. This document will guide you through the process.

## The Philosophy

`tbdflow` is an opinionated tool, and we try to follow our own advice. We practice Trunk-Based Development and use Conventional Commits. While we use Pull Requests to manage contributions from the community (the exact use case they were designed for!), we aim to keep them small, focused, and short-lived.

## How to Contribute

The best way to contribute is to start a conversation first.

- **Open an Issue**: Before you start writing code, please open an issue to discuss the bug you want to fix or the feature you want to add. This helps us ensure your work aligns with the project's goals and avoids duplicated effort.
- **Fork and Branch**: Once we've discussed the approach, fork the repository and create a new, short-lived branch for your work.
- **Write Code & Tests**: Make your changes and be sure to add or update tests to cover your work. We value a robust test suite!
- **Submit a Pull Request**: Push your branch and open a Pull Request against the `main` branch. Please link the issue you created in the PR description.

## Setting Up Your Development Environment

- **Install Rust**: If you don't have it, install the Rust toolchain from rustup.rs.
- **Clone the repository**:
  ```bash
  git clone https://github.com/cladam/tbdflow.git
  cd tbdflow/
  ```
- **Build the project**:
  ```bash
  cargo build
  ```

### Running Tests
We have a suite of integration tests to ensure the tool works as expected. Before submitting your changes, please make sure all tests pass.

To run the full test suite:
```bash
cargo test
```

## Coding Style & Conventions

We follow the standard Rust coding style. Please run the following commands before committing to ensure your code is formatted correctly and to catch any common issues.

Format your code:
```bash
cargo fmt
```

Run the linter:
```bash
cargo clippy -- -D warnings
```

## Commit Message Guidelines

We use `tbdflow` to develop `tbdflow`, so we follow our own rules! All commit messages must follow the Conventional Commits specification. This helps us maintain a clean history and automate our release process.

- Use the imperative mood: "add feature" not "added feature".
- Start with a type: feat, fix, docs, chore, refactor, test, etc.
- Provide a clear description.

Thank you again for your interest in contributing!

