package io.github.cladam.tbdflow

import com.intellij.openapi.progress.ProgressIndicator
import com.intellij.openapi.project.Project
import com.intellij.openapi.ui.DialogWrapper
import com.intellij.ui.components.JBLabel
import com.intellij.ui.components.JBTextField
import com.intellij.util.ui.FormBuilder
import javax.swing.JComponent

import com.intellij.openapi.application.ApplicationManager
import com.intellij.openapi.progress.ProgressManager
import com.intellij.openapi.progress.Task

class CompleteDialogue(private val project: Project) : DialogWrapper(project) {
    private val typeField = JBTextField()
    private val nameField = JBTextField()

    init {
        title = "tbdflow: Complete Branch"
        setSize(400, 150)
        init()
    }

    override fun createCenterPanel(): JComponent {
        return FormBuilder.createFormBuilder()
            .addLabeledComponent(JBLabel("Type:"), typeField, true)
            .addLabeledComponent(JBLabel("Name:"), nameField, true)
            .panel
    }

    override fun doOKAction() {
        val type = typeField.text
        val name = nameField.text

        if (type.isBlank() || name.isBlank()) {
            return // Basic validation
        }

        val commandList = listOf("tbdflow", "complete", "--type", type, "--name", name)
        ProgressManager.getInstance().run(object : Task.Backgroundable(project, "Completing Branch", false) {
            override fun run(indicator: ProgressIndicator) {
                val result = runCommandAndCaptureOutput(project, commandList)
                if (result != null) {
                    ApplicationManager.getApplication().invokeLater {
                        CommandResultDialogue(project, "tbdflow complete Result", result).show()
                    }
                }
            }
        })
        super.doOKAction()
    }
}
