// ===============================================================
// FILE: Git.fs
// ===============================================================
// This module contains all the logic for interacting with Git.
module Git

open System.Diagnostics

// A discriminated union to handle success or failure of a command.
// This is a common functional pattern for error handling.
type CommandResult<'Success, 'Error> =
    | Ok of 'Success
    | Error of 'Error
    
// A helper function to run an external process like "git".
let private runProcess (command: string, args: string)  =
    let startInfo = ProcessStartInfo(command, args)
    startInfo.UseShellExecute <- false
    startInfo.RedirectStandardOutput <- true
    startInfo.RedirectStandardError <- true
    use proc = Process.Start(startInfo)
    proc.WaitForExit()

    let stdout = proc.StandardOutput.ReadToEnd().Trim()
    let stderr = proc.StandardError.ReadToEnd().Trim()
    
    (proc.ExitCode, stdout, stderr)
        
// A helper function to run a Git command and handle the result.
let runGitCommand (command: string) (args: string) =
    printfn $"[RUNNING] git {command} {args}"
    // Correctly call runProcess with two arguments
    let exitCode, stdout, stderr = runProcess ("git", $"{command} {args}")
    
    // This 'match' expression is the key. It takes the output from runProcess
    // and correctly wraps it in the CommandResult type. This is what was missing.
    match exitCode with
    | 0 -> Ok stdout // Success
    | _ -> Error stderr // Failure
    

let checkoutMain () : CommandResult<string, string> =
    runGitCommand "checkout" "main"

let pullLatestWithRebase () : CommandResult<string, string> =
    runGitCommand "pull" "--rebase"

let addAll () : CommandResult<string, string> =
    runGitCommand "add" "."

let commit (message: string) : CommandResult<string, string> =
    runGitCommand "commit" $"-m \"{message}\""

let push () : CommandResult<string, string> =
    runGitCommand "push" ""

let mergeBranch (branchName: string) =
    runGitCommand "merge" $"--no-ff {branchName}"

let deleteLocalBranch (branchName: string) =
    runGitCommand "branch" $"-d {branchName}"

let deleteRemoteBranch (branchName: string) =
    runGitCommand "push" $"origin --delete {branchName}"