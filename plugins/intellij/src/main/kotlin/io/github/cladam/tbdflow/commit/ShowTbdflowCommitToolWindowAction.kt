package io.github.cladam.tbdflow.commit

import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.project.DumbAware
import com.intellij.openapi.wm.ToolWindowManager

class ShowTbdflowCommitToolWindowAction : AnAction("Show tbdflow Commit Tool Window"), DumbAware {
    override fun actionPerformed(e: AnActionEvent) {
        val project = e.project ?: return
        ToolWindowManager.getInstance(project)
            .getToolWindow("tbdflow Commit")
            ?.show()
    }
}
