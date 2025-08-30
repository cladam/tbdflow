package io.github.cladam.tbdflow

import com.intellij.openapi.application.ApplicationManager
import com.intellij.openapi.progress.ProgressIndicator
import com.intellij.openapi.progress.ProgressManager
import com.intellij.openapi.progress.Task
import com.intellij.openapi.project.Project
import com.intellij.openapi.ui.DialogWrapper
import com.intellij.ui.components.JBCheckBox
import com.intellij.ui.components.JBLabel
import com.intellij.ui.components.JBTextField
import com.intellij.util.ui.FormBuilder
import javax.swing.JComponent

class ChangelogDialogue(private val project: Project) : DialogWrapper(project) {
    private val fromField = JBTextField()
    private val toField = JBTextField()
    private val unreleasedCheckBox = JBCheckBox("Unreleased changes", false)

    init {
        title = "tbdflow: Generate Changelog"
        setSize(400, 200)
        init()

        // Add a listener to enable/disable fields based on the checkbox
        unreleasedCheckBox.addActionListener {
            val isUnreleased = unreleasedCheckBox.isSelected
            fromField.isEnabled = !isUnreleased
            toField.isEnabled = !isUnreleased
        }
    }

    override fun createCenterPanel(): JComponent {
        return FormBuilder.createFormBuilder()
            .addComponent(unreleasedCheckBox)
            .addLabeledComponent(JBLabel("From (tag):"), fromField, true)
            .addLabeledComponent(JBLabel("To (tag):"), toField, true)
            .panel
    }

    override fun doOKAction() {
        val from = fromField.text
        val to = toField.text
        val isUnreleased = unreleasedCheckBox.isSelected

        val commandList = mutableListOf("tbdflow", "changelog")
        if (isUnreleased) {
            commandList.add("--unreleased")
        } else {
            if (from.isNotBlank()) commandList.addAll(listOf("--from", from))
            if (to.isNotBlank()) commandList.addAll(listOf("--to", to))
        }

        // Run the command in a background thread to avoid freezing the UI
        ProgressManager.getInstance().run(object : Task.Backgroundable(project, "Generating Changelog", false) {
            override fun run(indicator: ProgressIndicator) {
                val result = runCommandAndCaptureOutput(project, commandList)
                if (result != null) {
                    // Show the result in a new dialog on the UI thread
                    ApplicationManager.getApplication().invokeLater {
                        ChangelogResultDialogue(project, result).show()
                    }
                }
            }
        })

        super.doOKAction()
    }
}
