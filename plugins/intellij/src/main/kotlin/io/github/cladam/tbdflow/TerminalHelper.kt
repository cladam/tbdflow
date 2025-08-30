package io.github.cladam.tbdflow

import com.intellij.notification.NotificationGroupManager
import com.intellij.notification.NotificationType
import com.intellij.openapi.project.Project
import java.io.BufferedReader
import java.io.InputStreamReader

fun runCommandAndCaptureOutput(project: Project, command: List<String>): String? {
    val notificationGroup = NotificationGroupManager.getInstance()
        .getNotificationGroup("tbdflow Notifications")

    try {
        println("Running command: ${command.joinToString(" ")}")
        val processBuilder = ProcessBuilder(command)
            .directory(project.basePath?.let { java.io.File(it) })
            .redirectErrorStream(true) // Combine stdout and stderr

        val process = processBuilder.start()

        val output = StringBuilder()
        val reader = BufferedReader(InputStreamReader(process.inputStream))
        var line: String?
        while (reader.readLine().also { line = it } != null) {
            output.append(line).append("\n")
        }

        val exitCode = process.waitFor()

        if (exitCode == 0) {
            // Let the caller decide how to display success
            return output.toString()
        } else {
            notificationGroup.createNotification(
                "tbdflow Command Failed",
                output.toString(),
                NotificationType.ERROR
            ).notify(project)
            return null
        }

    } catch (e: Exception) {
        notificationGroup.createNotification(
            "tbdflow Execution Error",
            e.message ?: "An unknown error occurred.",
            NotificationType.ERROR
        ).notify(project)
        return null
    }
}

