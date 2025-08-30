package io.github.cladam.tbdflow

import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.vcs.VcsDataKeys
import com.intellij.openapi.vcs.changes.ChangeListManager

class CommitAction : AnAction() {
    override fun actionPerformed(e: AnActionEvent) {
        val project = e.project ?: return
        // Show our custom dialog
        val dialog = CommitDialogue(project)
        dialog.show()
    }

    override fun update(e: AnActionEvent) {
        val project = e.project
        val changes = project?.let { ChangeListManager.getInstance(it).allChanges }
        // Enable the action only if there are changes to commit
        e.presentation.isEnabled = project != null && !changes.isNullOrEmpty()
    }
}