package io.github.cladam.tbdflow.commit

import com.intellij.openapi.project.DumbAware
import com.intellij.openapi.project.Project
import com.intellij.openapi.wm.ToolWindow
import com.intellij.openapi.wm.ToolWindowFactory
import com.intellij.ui.content.ContentFactory

class TbdflowCommitToolWindowFactory : ToolWindowFactory, DumbAware {
    override fun createToolWindowContent(project: Project, toolWindow: ToolWindow) {
        // Create an instance of our commit panel UI
        val commitPanel = TbdflowCommitPanel(project)
        val contentFactory = ContentFactory.getInstance()
        // Create the content for the tool window
        val content = contentFactory.createContent(commitPanel, "", false)
        // Add the content to the tool window
        toolWindow.contentManager.addContent(content)
    }
}