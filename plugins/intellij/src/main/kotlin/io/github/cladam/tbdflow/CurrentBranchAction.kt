package io.github.cladam.tbdflow

import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.project.DumbAware
import com.intellij.openapi.vcs.ProjectLevelVcsManager
import org.jetbrains.plugins.terminal.TerminalToolWindowManager

class CurrentBranchAction : AnAction(), DumbAware {

    override fun actionPerformed(e: AnActionEvent) {
        val project = e.project ?: return
        val command = "tbdflow current-branch"

        // Get the integrated terminal and run the command
        val terminalManager = TerminalToolWindowManager.getInstance(project)
        val terminal = terminalManager.createShellWidget(project.basePath, "tbdflow", true, false)
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