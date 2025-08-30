package io.github.cladam.tbdflow

import com.intellij.openapi.application.ApplicationManager
import com.intellij.openapi.progress.ProgressIndicator
import com.intellij.openapi.progress.ProgressManager
import com.intellij.openapi.progress.Task
import com.intellij.openapi.project.Project
import com.intellij.openapi.ui.DialogWrapper
import com.intellij.ui.components.JBLabel
import com.intellij.ui.components.JBTextField
import com.intellij.util.ui.FormBuilder
import javax.swing.JComponent

class BranchDialogue(private val project: Project) : DialogWrapper(project) {
    private val typeField = JBTextField()
    private val nameField = JBTextField()
    private val issueField = JBTextField()

    init {
        title = "tbdflow: New Branch"
        setSize(400, 200)
        init()
    }

    override fun createCenterPanel(): JComponent {
        return FormBuilder.createFormBuilder()
            .addLabeledComponent(JBLabel("Type:"), typeField, true)
            .addLabeledComponent(JBLabel("Name:"), nameField, true)
            .addLabeledComponent(JBLabel("Issue (optional):"), issueField, true)
            .panel
    }

    override fun doOKAction() {
        val type = typeField.text
        val name = nameField.text
        val issue = issueField.text

        if (type.isBlank() || name.isBlank()) {
            return // Basic validation
        }

        val commandList = mutableListOf("tbdflow", "branch", "--type", type, "--name", name)
        if (issue.isNotBlank()) {
            commandList.addAll(listOf("--issue", issue))
        }

        ProgressManager.getInstance().run(object : Task.Backgroundable(project, "Creating Branch", false) {
            override fun run(indicator: ProgressIndicator) {
                val result = runCommandAndCaptureOutput(project, commandList)
                if (result != null) {
                    ApplicationManager.getApplication().invokeLater {
                        CommandResultDialogue(project, "tbdflow branch Result", result).show()
                    }
                }
            }
        })
        super.doOKAction()
    }
}
