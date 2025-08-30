package io.github.cladam.tbdflow

import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.vcs.ProjectLevelVcsManager
import com.intellij.openapi.vcs.VcsDataKeys
import org.jetbrains.plugins.terminal.TerminalToolWindowManager

class SyncAction : AnAction() {

    override fun actionPerformed(e: AnActionEvent) {
        val project = e.project ?: return
        val basePath = project.basePath ?: return

        // Find the tbdflow executable.
        // For now, we'll assume it's in the user's PATH.
        // A more robust solution would be to make this configurable.
        val command = "tbdflow sync"

        // Get the integrated terminal and run the command
        val terminalManager = TerminalToolWindowManager.getInstance(project)
        val terminal = terminalManager.createShellWidget(basePath, "tbdflow-sync", true, false)
        terminal.sendCommandToExecute(command)
    }

    override fun update(e: AnActionEvent) {
        val project = e.project
        if (project == null) {
            e.presentation.isEnabledAndVisible = false
            return
        }
        // Check if there are any Git roots in the project
        val vcsManager = ProjectLevelVcsManager.getInstance(project)
        val isGitProject = vcsManager.allVcsRoots.any { it.vcs?.name == "Git" }
        e.presentation.isEnabledAndVisible = isGitProject
    }
}