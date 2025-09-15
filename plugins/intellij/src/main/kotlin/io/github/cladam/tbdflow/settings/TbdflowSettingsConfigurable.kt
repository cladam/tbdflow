package io.github.cladam.tbdflow.settings

import com.intellij.openapi.fileChooser.FileChooser
import com.intellij.openapi.fileChooser.FileChooserDescriptor
import com.intellij.openapi.options.Configurable
import com.intellij.openapi.ui.TextFieldWithBrowseButton
import com.intellij.openapi.vfs.VirtualFile
import com.intellij.util.ui.FormBuilder
import java.awt.Component
import javax.swing.JComponent
import javax.swing.JPanel

class TbdflowSettingsConfigurable : Configurable {

    private var settingsPanel: JPanel? = null
    private val executablePathField = TextFieldWithBrowseButton()

    override fun getDisplayName(): String {
        return "tbdflow"
    }

    override fun createComponent(): JComponent? {
        executablePathField.addActionListener {
            val descriptor = FileChooserDescriptor(true, false, false, false, false, false)
                .withTitle("Select tbdflow Executable")
            val parent: Component = executablePathField
            val project = null // or provide your Project instance if available

            FileChooser.chooseFile(descriptor, project, null) { file: VirtualFile? ->
                if (file != null) {
                    executablePathField.text = file.path
                }
            }
        }

        settingsPanel = FormBuilder.createFormBuilder()
            .addLabeledComponent("tbdflow executable path:", executablePathField, 1, false)
            .addComponentFillVertically(JPanel(), 0)
            .panel
        return settingsPanel
    }

    override fun isModified(): Boolean {
        val settings = TbdflowSettingsState.instance
        return executablePathField.text != settings.tbdflowExecutablePath
    }

    override fun apply() {
        val settings = TbdflowSettingsState.instance
        settings.tbdflowExecutablePath = executablePathField.text
    }

    override fun reset() {
        val settings = TbdflowSettingsState.instance
        executablePathField.text = settings.tbdflowExecutablePath
    }

    override fun disposeUIResources() {
        settingsPanel = null
    }
}