package io.github.cladam.tbdflow.settings

import com.intellij.openapi.application.ApplicationManager
import com.intellij.openapi.components.PersistentStateComponent
import com.intellij.openapi.components.State
import com.intellij.openapi.components.Storage
import com.intellij.util.xmlb.XmlSerializerUtil

@State(
    name = "io.github.cladam.tbdflow.settings.TbdflowSettingsState",
    storages = [Storage("TbdflowSettings.xml")]
)
class TbdflowSettingsState : PersistentStateComponent<TbdflowSettingsState> {

    var tbdflowExecutablePath: String = "tbdflow" // Default to assuming it's in PATH

    override fun getState(): TbdflowSettingsState {
        return this
    }

    override fun loadState(state: TbdflowSettingsState) {
        XmlSerializerUtil.copyBean(state, this)
    }

    companion object {
        val instance: TbdflowSettingsState
            get() = ApplicationManager.getApplication().getService(TbdflowSettingsState::class.java)
    }
}