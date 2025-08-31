## `tbdflow` IntelliJ Plugin
This plugin integrates `tbdflow` into your JetBrains IDE as a wrapper, providing a guided workflow for Trunk-Based Development.

It enhances the IDE's built-in Git support with the unique "guardrail" features of `tbdflow`, helping you and your team maintain a high-quality, consistent, and fast-flowing development process.

### Features
- **Integrated Workflow Actions**: Access all core `tbdflow` commands (`commit`, `branch`, `complete`, `sync`, etc.) directly from the "Tools" -> "tbdflow" menu in your IDE.
- **Guided Commit Dialogue**: A custom commit dialogue that helps you build perfect Conventional Commit messages with fields for type, scope, body, and breaking changes.
- **Background Execution & Notifications**: Commands are run in the background, with clear success and error notifications, so your workflow is never interrupted.

### Requirements
Before you can use this plugin, you must have the `tbdflow` command-line tool installed and available in your system's PATH.

You can install it by running:
```bash
cargo install tbdflow
```

To make your repo compatible with `tbdflow`, initialise it by running:
```bash
tbdflow init
```

### Usage
Once installed, the plugin adds a new "tbdflow" sub-menu to your main "Tools" menu. From here, you can trigger all the core workflow commands.

#### Caveat
`tbdflow` is a commandline tool, and a wrapper can only do so much. 
