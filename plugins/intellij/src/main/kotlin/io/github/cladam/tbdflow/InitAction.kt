package io.github.cladam.tbdflow

import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.project.DumbAware
import org.jetbrains.plugins.terminal.TerminalToolWindowManager

class InitAction : AnAction(), DumbAware {

    override fun actionPerformed(e: AnActionEvent) {
        val project = e.project ?: return

        // Run the command in the terminal
        runCommandInTerminal(project, listOf("tbdflow", "init"))
    }

    override fun update(e: AnActionEvent) {
        // This action should be available as long as a project is open.
        e.presentation.isEnabledAndVisible = e.project != null
    }
}