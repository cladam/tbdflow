package io.github.cladam.tbdflow.commit

import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.vcs.ProjectLevelVcsManager
import com.intellij.openapi.project.DumbAware
import com.intellij.openapi.wm.ToolWindowManager

class CommitAction : AnAction(), DumbAware {
    override fun actionPerformed(e: AnActionEvent) {
        val project = e.project ?: return
        // Find the tool window by its ID and activate it.
        val toolWindow = ToolWindowManager.getInstance(project).getToolWindow("tbdflow Commit")
        toolWindow?.activate(null)
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