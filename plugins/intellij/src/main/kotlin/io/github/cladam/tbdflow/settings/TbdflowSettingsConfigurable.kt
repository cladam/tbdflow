package io.github.cladam.tbdflow.settings

import com.intellij.openapi.fileChooser.FileChooserDescriptorFactory
import com.intellij.openapi.options.Configurable
import com.intellij.openapi.ui.TextFieldWithBrowseButton
import com.intellij.ui.components.JBLabel
import com.intellij.util.ui.FormBuilder
import javax.swing.JComponent
import javax.swing.JPanel

class TbdflowSettingsConfigurable : Configurable {

    private var settingsPanel: JPanel? = null
    private val tbdflowPathField = TextFieldWithBrowseButton()

    override fun getDisplayName(): String {
        return "tbdflow"
    }

    override fun createComponent(): JComponent {
        tbdflowPathField.addBrowseFolderListener(
            "Select tbdflow Executable",
            "Please select the path to your tbdflow executable",
            null,
            FileChooserDescriptorFactory.createSingleFileDescriptor()
        )

        settingsPanel = FormBuilder.createFormBuilder()
            .addLabeledComponent(JBLabel("tbdflow executable path:"), tbdflowPathField, 1, true)
            .addComponentFillVertically(JPanel(), 0)
            .panel

        return settingsPanel!!
    }

    override fun isModified(): Boolean {
        val settings = TbdflowSettingsState.instance
        return tbdflowPathField.text != settings.tbdflowExecutablePath
    }

    override fun apply() {
        val settings = TbdflowSettingsState.instance
        settings.tbdflowExecutablePath = tbdflowPathField.text
    }

    override fun reset() {
        val settings = TbdflowSettingsState.instance
        tbdflowPathField.text = settings.tbdflowExecutablePath
    }

    override fun disposeUIResources() {
        settingsPanel = null
    }
}

