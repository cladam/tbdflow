package io.github.cladam.tbdflow

import com.intellij.notification.NotificationGroupManager
import com.intellij.notification.NotificationType
import com.intellij.openapi.application.ApplicationManager
import com.intellij.openapi.project.Project
import java.io.BufferedReader
import java.io.InputStreamReader
import io.github.cladam.tbdflow.settings.TbdflowSettingsState
import java.util.concurrent.TimeUnit

fun runCommandAndCaptureOutput(project: Project, command: List<String>): String? {
    val settings = TbdflowSettingsState.instance
    val executablePath = settings.tbdflowExecutablePath.ifBlank { "tbdflow" }

    // The first element of the 'command' list is the placeholder 'tbdflow'.
    // We replace it with the actual executable path from settings.
    val fullCommand = mutableListOf(executablePath)
    fullCommand.addAll(command.drop(1))

    try {
        val processBuilder = ProcessBuilder(fullCommand)
        processBuilder.directory(project.basePath?.let { java.io.File(it) })
        val process = processBuilder.start()

        val stdout = BufferedReader(InputStreamReader(process.inputStream)).readText()
        val stderr = BufferedReader(InputStreamReader(process.errorStream)).readText()

        process.waitFor(60, TimeUnit.SECONDS)

        return if (process.exitValue() == 0) {
            stdout.ifBlank { "Command executed successfully." }
        } else {
            ApplicationManager.getApplication().invokeLater {
                NotificationGroupManager.getInstance()
                    .getNotificationGroup("tbdflow Notifications")
                    .createNotification("tbdflow Command Failed", stderr, NotificationType.ERROR)
                    .notify(project)
            }
            return null
        }
    } catch (e: Exception) {
        ApplicationManager.getApplication().invokeLater {
            val errorMessage = """
                Failed to run command: ${e.message}
                Please ensure the path to the 'tbdflow' executable is configured correctly in Settings/Preferences -> Tools -> tbdflow.
            """.trimIndent()
            NotificationGroupManager.getInstance()
                .getNotificationGroup("tbdflow Notifications")
                .createNotification("tbdflow Execution Error", errorMessage, NotificationType.ERROR)
                .notify(project)
        }
        return null
    }
}

