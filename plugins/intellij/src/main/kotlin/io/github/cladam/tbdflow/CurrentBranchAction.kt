package io.github.cladam.tbdflow

import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.progress.ProgressIndicator
import com.intellij.openapi.progress.ProgressManager
import com.intellij.openapi.application.ApplicationManager
import com.intellij.openapi.vcs.VcsDataKeys
import com.intellij.openapi.progress.Task
import com.intellij.openapi.project.DumbAware
import com.intellij.openapi.vcs.ProjectLevelVcsManager

class CurrentBranchAction : AnAction(), DumbAware {

    override fun actionPerformed(e: AnActionEvent) {
        val project = e.project ?: return
        val command = listOf("tbdflow", "current-branch")

        ProgressManager.getInstance().run(object : Task.Backgroundable(project, "Getting Current Branch", false) {
            override fun run(indicator: ProgressIndicator) {
                val result = runCommandAndCaptureOutput(project, command)
                if (result != null) {
                    ApplicationManager.getApplication().invokeLater {
                        CommandResultDialogue(project, "Current Branch", result).show()
                    }
                }
            }
        })
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