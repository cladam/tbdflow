package io.github.cladam.tbdflow

import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.vcs.VcsDataKeys
import com.intellij.openapi.vcs.changes.ChangeListManager
import com.intellij.openapi.vcs.ProjectLevelVcsManager

class CommitAction : AnAction() {
    override fun actionPerformed(e: AnActionEvent) {
        val project = e.project ?: return
        // Show our custom dialog
        val dialog = CommitDialogue(project)
        dialog.show()
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