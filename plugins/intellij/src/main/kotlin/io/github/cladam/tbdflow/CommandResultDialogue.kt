package io.github.cladam.tbdflow

import com.intellij.openapi.ide.CopyPasteManager
import com.intellij.openapi.project.Project
import com.intellij.openapi.ui.DialogWrapper
import com.intellij.ui.components.JBScrollPane
import com.intellij.ui.components.JBTextArea
import java.awt.BorderLayout
import java.awt.Dimension
import java.awt.datatransfer.StringSelection
import javax.swing.Action
import javax.swing.JComponent
import javax.swing.JPanel

class CommandResultDialogue(project: Project, title: String, output: String) : DialogWrapper(project) {
    private val textArea = JBTextArea(output)

    init {
        this.title = title
        setSize(800, 600)
        textArea.isEditable = false
        init()
    }

    override fun createCenterPanel(): JComponent {
        val panel = JPanel(BorderLayout())
        val scrollPane = JBScrollPane(textArea)
        scrollPane.preferredSize = Dimension(750, 500)
        panel.add(scrollPane, BorderLayout.CENTER)
        return panel
    }

    override fun createActions(): Array<Action> {
        val copyAction = createAction("Copy to Clipboard")
        val okAction = createAction("OK")
        okAction.putValue(DEFAULT_ACTION, true)
        return arrayOf(copyAction, okAction)
    }

    private fun createAction(name: String): Action {
        return object : DialogWrapperAction(name) {
            override fun doAction(e: java.awt.event.ActionEvent) {
                if (name == "Copy to Clipboard") {
                    CopyPasteManager.getInstance().setContents(StringSelection(textArea.text))
                }
                close(OK_EXIT_CODE)
            }
        }
    }
}
