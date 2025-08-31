package io.github.cladam.tbdflow.commit

import com.intellij.openapi.application.ApplicationManager
import com.intellij.openapi.progress.ProgressIndicator
import com.intellij.openapi.progress.ProgressManager
import com.intellij.openapi.progress.Task
import com.intellij.openapi.project.Project
import com.intellij.openapi.ui.Messages
import com.intellij.ui.components.JBCheckBox
import com.intellij.ui.components.JBLabel
import com.intellij.ui.components.JBScrollPane
import com.intellij.ui.components.JBTextArea
import com.intellij.ui.components.JBTextField
import com.intellij.util.ui.FormBuilder
import io.github.cladam.tbdflow.CommandResultDialogue
import io.github.cladam.tbdflow.runCommandAndCaptureOutput
import java.awt.BorderLayout
import javax.swing.BoxLayout
import javax.swing.JButton
import javax.swing.JPanel

// This class now holds the UI for our commit panel
class TbdflowCommitPanel(private val project: Project) : JPanel(BorderLayout()) {

    private val typeField = JBTextField()
    private val scopeField = JBTextField()
    private val messageField = JBTextField()
    private val bodyArea = JBTextArea(5, 0)
    private val issueField = JBTextField()
    private val tagField = JBTextField()
    private val breakingChangeCheckBox = JBCheckBox("Is this a breaking change?")
    private val breakingDescriptionField = JBTextField()
    private val checklistPanel = JPanel().apply {
        layout = BoxLayout(this, BoxLayout.Y_AXIS)
    }
    private val checkBoxes = mutableListOf<JBCheckBox>()
    private val commitButton = JButton("Commit")

    init {
        breakingDescriptionField.isEnabled = false
        breakingChangeCheckBox.addActionListener {
            breakingDescriptionField.isEnabled = breakingChangeCheckBox.isSelected
        }

        val formPanel = FormBuilder.createFormBuilder()
            .addLabeledComponent(JBLabel("Type:"), typeField, true)
            .addLabeledComponent(JBLabel("Scope (optional):"), scopeField, true)
            .addLabeledComponent(JBLabel("Message:"), messageField, true)
            .addLabeledComponent(JBLabel("Body (optional):"), JBScrollPane(bodyArea), true)
            .addLabeledComponent(JBLabel("Issue (optional):"), issueField, true)
            .addLabeledComponent(JBLabel("Tag (optional):"), tagField, true)
            .addComponent(breakingChangeCheckBox)
            .addLabeledComponent(JBLabel("Breaking Description:"), breakingDescriptionField, true)
            .addSeparator()
            .addComponent(JBLabel("Definition of Done Checklist:"))
            .addComponent(checklistPanel)
            .panel

        add(JBScrollPane(formPanel), BorderLayout.CENTER)
        add(commitButton, BorderLayout.SOUTH)

        commitButton.addActionListener {
            performCommit()
        }

        loadChecklistItems()
    }

    private fun loadChecklistItems() {
        ProgressManager.getInstance().run(object : Task.Backgroundable(project, "Loading DoD Checklist", false) {
            override fun run(indicator: ProgressIndicator) {
                val command = listOf("tbdflow", "config", "--get-dod")
                val dodItems = runCommandAndCaptureOutput(project, command)?.lines()?.filter { it.isNotBlank() }

                ApplicationManager.getApplication().invokeLater {
                    checklistPanel.removeAll()
                    checkBoxes.clear()
                    if (!dodItems.isNullOrEmpty()) {
                        dodItems.forEach { item ->
                            val checkBox = JBCheckBox(item)
                            checkBoxes.add(checkBox)
                            checklistPanel.add(checkBox)
                        }
                    }
                    checklistPanel.revalidate()
                    checklistPanel.repaint()
                }
            }
        })
    }

    private fun performCommit() {
        val allChecked = checkBoxes.all { it.isSelected }
        if (!allChecked && checkBoxes.isNotEmpty()) {
            Messages.showErrorDialog(
                project,
                "Please complete all items in the Definition of Done checklist.",
                "DoD Not Complete"
            )
            return
        }

        val commandList = mutableListOf("tbdflow", "commit", "--type", typeField.text, "--message", messageField.text, "--no-verify")

        if (scopeField.text.isNotBlank()) {
            commandList.addAll(listOf("--scope", scopeField.text))
        }
        if (bodyArea.text.isNotBlank()) {
            commandList.addAll(listOf("--body", bodyArea.text))
        }
        if (issueField.text.isNotBlank()) {
            commandList.addAll(listOf("--issue", issueField.text))
        }
        if (tagField.text.isNotBlank()) {
            commandList.addAll(listOf("--tag", tagField.text))
        }
        if (breakingChangeCheckBox.isSelected) {
            commandList.add("--breaking")
            if (breakingDescriptionField.text.isNotBlank()) {
                commandList.addAll(listOf("--breaking-description", breakingDescriptionField.text))
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
    }
}
