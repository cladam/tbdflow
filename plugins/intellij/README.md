## `tbdflow` IntelliJ Plugin
This plugin integrates `tbdflow` into your JetBrains IDE as a wrapper, providing a guided workflow for Trunk-Based Development.

It enhances the IDE's built-in Git support with the unique "guardrail" features of `tbdflow`, helping you and your team maintain a high-quality, consistent, and fast-flowing development process.

### Features
- **Integrated Commit Tool Window**: A dedicated, non-blocking side panel for crafting perfect Conventional Commits.
- **Native Definition of Done (DoD) Checklist**: The commit panel automatically reads your project's `.dod.yml` file and displays an interactive checklist. Committing is blocked until the team's quality standards are met.
- **Integrated Workflow Actions**: Access all core `tbdflow` commands (`branch`, `complete`, `sync`, etc.) directly from the "Tools" -> "tbdflow" menu in your IDE.
- **Background Execution & Notifications**: Commands are run in the background, with clear success and error notifications, so your workflow is never interrupted.
- **Configurable Executable Path**: A dedicated settings page allows you to specify the path to your tbdflow binary, ensuring the plugin works in any environment.

### Installation & Setup

**Step 1: Install the tbdflow CLI**
Before you can use this plugin, you must have the `tbdflow` command-line tool installed and available in your system's PATH.

You can install it by running:
```bash
cargo install tbdflow
```

To make your repo compatible with `tbdflow`, initialise it by running:
```bash
tbdflow init
```

**Step 2: Install the plugin**
Install the plugin from the JetBrains Marketplace. Go to Settings/Preferences -> Plugins, search for "tbdflow", and click "Install".

**Step 3: Configure the executable path (optional)**
After installing, you must tell the plugin where to find the tbdflow executable.

Go to Settings/Preferences -> Tools -> tbdflow.

In the "tbdflow executable path" field, enter the full path to your tbdflow binary. (You can find this by running `which tbdflow` in your terminal).

### Usage
Once configured, the plugin is ready to use:

To Commit: Open the "tbdflow Commit" tool window from the left-hand sidebar (or the Tools menu) to craft your commit.

Other Commands: Access all other commands from the Tools -> tbdflow sub-menu.

#### Caveat
`tbdflow` is a commandline tool, and a wrapper can only do so much, please explore `tbdflow`in the terminal for the full experience.
