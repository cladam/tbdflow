package io.github.cladam.tbdflow

import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.application.ApplicationManager
import com.intellij.openapi.progress.ProgressIndicator
import com.intellij.openapi.progress.ProgressManager
import com.intellij.openapi.progress.Task
import com.intellij.openapi.vcs.VcsDataKeys

class CheckBranchesAction : AnAction() {
    override fun actionPerformed(e: AnActionEvent) {
        val project = e.project ?: return
        val command = listOf("tbdflow", "check-branches")

        ProgressManager.getInstance().run(object : Task.Backgroundable(project, "Checking Stale Branches", false) {
            override fun run(indicator: ProgressIndicator) {
                val result = runCommandAndCaptureOutput(project, command)
                if (result != null) {
                    ApplicationManager.getApplication().invokeLater {
                        CommandResultDialogue(project, "Stale Branch Check Result", result).show()
                    }
                }
            }
        })
    }

    override fun update(e: AnActionEvent) {
        val project = e.project
        e.presentation.isEnabled = project != null && e.getData(VcsDataKeys.VCS)?.name == "Git"
    }
}