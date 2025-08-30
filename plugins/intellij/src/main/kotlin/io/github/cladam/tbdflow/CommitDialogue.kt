package io.github.cladam.tbdflow

import com.intellij.openapi.application.ApplicationManager
import com.intellij.openapi.progress.ProgressIndicator
import com.intellij.openapi.progress.ProgressManager
import com.intellij.openapi.progress.Task
import com.intellij.openapi.project.Project
import com.intellij.openapi.ui.DialogWrapper
import com.intellij.ui.components.JBCheckBox
import com.intellij.ui.components.JBLabel
import com.intellij.ui.components.JBTextArea
import com.intellij.ui.components.JBTextField
import com.intellij.util.ui.FormBuilder
import java.awt.BorderLayout
import javax.swing.JComponent
import javax.swing.JPanel

class CommitDialogue(private val project: Project) : DialogWrapper(project) {
    private val typeField = JBTextField()
    private val scopeField = JBTextField()
    private val messageField = JBTextField()
    private val bodyArea = JBTextArea(5, 0)
    private val issueField = JBTextField()
    private val tagField = JBTextField()
    private val breakingChangeCheckBox = JBCheckBox("Is this a breaking change?")
    private val breakingDescriptionField = JBTextField()

    init {
        title = "tbdflow Commit"
        setSize(600, 400)
        init()
        // Initially disable the breaking description field
        breakingDescriptionField.isEnabled = false
        breakingChangeCheckBox.addActionListener {
            breakingDescriptionField.isEnabled = breakingChangeCheckBox.isSelected
        }
    }

    override fun createCenterPanel(): JComponent {
        val formPanel = FormBuilder.createFormBuilder()
            .addLabeledComponent(JBLabel("Type:"), typeField, true)
            .addLabeledComponent(JBLabel("Scope (optional):"), scopeField, true)
            .addLabeledComponent(JBLabel("Message:"), messageField, true)
            .addLabeledComponent(JBLabel("Body (optional):"), bodyArea, true)
            .addLabeledComponent(JBLabel("Issue (optional):"), issueField, true)
            .addLabeledComponent(JBLabel("Tag (optional):"), tagField, true)
            .addComponent(breakingChangeCheckBox)
            .addLabeledComponent(JBLabel("Breaking description:"), breakingDescriptionField, true)
            .panel

        val mainPanel = JPanel(BorderLayout())
        mainPanel.add(formPanel, BorderLayout.CENTER)
        return mainPanel
    }

    override fun doOKAction() {
        // This is where we will build and run the tbdflow command
        val type = typeField.text
        val scope = scopeField.text
        val message = messageField.text
        val body = bodyArea.text
        val issue = issueField.text
        val tag = tagField.text
        val isBreaking = breakingChangeCheckBox.isSelected
        val breakingDescription = breakingDescriptionField.text

        // Basic validation
        if (type.isBlank() || message.isBlank()) {
            // In a real implementation, we'd show a validation error
            return
        }

        // Build the command as a list of arguments to handle spaces correctly
        val commandList = mutableListOf("tbdflow", "commit", "--type", type, "--message", message)

        if (scope.isNotBlank()) {
            commandList.addAll(listOf("--scope", scope))
        }
        if (body.isNotBlank()) {
            commandList.addAll(listOf("--body", body))
        }
        if (issue.isNotBlank()) {
            commandList.addAll(listOf("--issue", issue))
        }
        if (tag.isNotBlank()) {
            commandList.addAll(listOf("--tag", tag))
        }
        if (isBreaking) {
            commandList.add("--breaking")
            if (breakingDescription.isNotBlank()) {
                commandList.addAll(listOf("--breaking-description", breakingDescription))
            }
        }

        ProgressManager.getInstance().run(object : Task.Backgroundable(project, "Running tbdflow commit", false) {
            override fun run(indicator: ProgressIndicator) {
                val result = runCommandAndCaptureOutput(project, commandList)
                if (result != null) {
                    ApplicationManager.getApplication().invokeLater {
                        CommandResultDialogue(project, "tbdflow commit Result", result).show()
                    }
                }
            }
        })

        super.doOKAction()
    }
}