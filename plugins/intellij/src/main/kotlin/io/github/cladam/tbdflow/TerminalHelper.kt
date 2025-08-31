package io.github.cladam.tbdflow

import com.intellij.notification.NotificationGroupManager
import com.intellij.notification.NotificationType
import com.intellij.openapi.project.Project
import java.io.BufferedReader
import java.io.InputStreamReader
import io.github.cladam.tbdflow.settings.TbdflowSettingsState
import java.util.concurrent.TimeUnit

fun runCommandAndCaptureOutput(project: Project, command: List<String>): String? {
    val settings = TbdflowSettingsState.instance
    val executable = settings.tbdflowExecutablePath

    // Replace the placeholder "tbdflow" with the configured path
    val fullCommand = command.toMutableList()
    if (fullCommand.isNotEmpty() && fullCommand[0] == "tbdflow") {
        fullCommand[0] = executable
    }

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
            val errorMessage = stderr.ifBlank { stdout.ifBlank { "An unknown error occurred." } }
            NotificationGroupManager.getInstance()
                .getNotificationGroup("tbdflow Notifications")
                .createNotification("tbdflow Command Failed", errorMessage, NotificationType.ERROR)
                .notify(project)
            null
        }
    } catch (e: Exception) {
        NotificationGroupManager.getInstance()
            .getNotificationGroup("tbdflow Notifications")
            .createNotification(
                "tbdflow Execution Error",
                "Failed to run command: ${e.message}\n" +
                        "Please ensure the path to the 'tbdflow' executable is configured correctly in Settings/Preferences -> Tools -> tbdflow.",
                NotificationType.ERROR
            )
            .notify(project)
        return null
    }
}

