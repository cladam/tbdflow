package io.github.cladam.tbdflow

import com.intellij.execution.configurations.GeneralCommandLine
import com.intellij.execution.process.OSProcessHandler
import com.intellij.execution.process.ProcessAdapter
import com.intellij.execution.process.ProcessEvent
import com.intellij.execution.process.ProcessOutputType
import com.intellij.notification.NotificationGroupManager
import com.intellij.notification.NotificationType
import com.intellij.openapi.project.Project
import com.intellij.openapi.util.Key

const val NOTIFICATION_GROUP_ID = "tbdflow Notifications"

fun runCommandInTerminal(project: Project, command: List<String>) {
    val commandLine = GeneralCommandLine(command)
    commandLine.setWorkDirectory(project.basePath)

    val processHandler = OSProcessHandler(commandLine)
    val output = StringBuilder()
    val errorOutput = StringBuilder()

    processHandler.addProcessListener(object : ProcessAdapter() {
        override fun onTextAvailable(event: ProcessEvent, outputType: Key<*>) {
            if (ProcessOutputType.isStderr(outputType)) {
                errorOutput.append(event.text)
            } else {
                output.append(event.text)
            }
        }

        override fun processTerminated(event: ProcessEvent) {
            // Ensure notifications are shown on the UI thread
            com.intellij.openapi.application.ApplicationManager.getApplication().invokeLater {
                if (event.exitCode == 0) {
                    // Success: Show the actual output from the command
                    NotificationGroupManager.getInstance()
                        .getNotificationGroup(NOTIFICATION_GROUP_ID)
                        .createNotification("tbdflow Success", output.toString().trim(), NotificationType.INFORMATION)
                        .notify(project)
                } else {
                    // Failure: Show the error output
                    NotificationGroupManager.getInstance()
                        .getNotificationGroup(NOTIFICATION_GROUP_ID)
                        .createNotification(
                            "tbdflow Command Failed",
                            // Use stderr, but fall back to stdout if stderr is empty
                            if (errorOutput.isNotBlank()) errorOutput.toString().trim() else output.toString().trim(),
                            NotificationType.ERROR
                        )
                        .notify(project)
                }
            }
        }
    })

    processHandler.startNotify()
}
/*
// Alternative version with custom success message
fun runCommandInTerminal(project: Project, command: String, successMessage: String) {
    val commandLine = GeneralCommandLine(command.split(" "))
    commandLine.setWorkDirectory(project.basePath)

    val processHandler = OSProcessHandler(commandLine)
    val output = StringBuilder()
    val errorOutput = StringBuilder()

    processHandler.addProcessListener(object : ProcessAdapter() {
        override fun onTextAvailable(event: ProcessEvent, outputType: Key<*>) {
            if (ProcessOutputType.isStderr(outputType)) {
                errorOutput.append(event.text)
            } else {
                output.append(event.text)
            }
        }

        override fun processTerminated(event: ProcessEvent) {
            // Ensure notifications are shown on the UI thread
            com.intellij.openapi.application.ApplicationManager.getApplication().invokeLater {
                if (event.exitCode == 0) {
                    // Success
                    NotificationGroupManager.getInstance()
                        .getNotificationGroup(NOTIFICATION_GROUP_ID)
                        .createNotification("tbdflow Success", successMessage, NotificationType.INFORMATION)
                        .notify(project)
                } else {
                    // Failure
                    NotificationGroupManager.getInstance()
                        .getNotificationGroup(NOTIFICATION_GROUP_ID)
                        .createNotification(
                            "tbdflow Command Failed",
                            // Use stderr, but fall back to stdout if stderr is empty
                            if (errorOutput.isNotBlank()) errorOutput.toString() else output.toString(),
                            NotificationType.ERROR
                        )
                        .notify(project)
                }
            }
        }
    })

    processHandler.startNotify()
}
*/
