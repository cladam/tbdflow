module Program

open CommandLine
open Git

// Helper for printing coloured text
type PrintMode =
    | Success
    | ErrorMode
    | Info
    | Warning

let printColour mode text =
    let colour =
        match mode with
        | Success -> System.ConsoleColor.Green
        | ErrorMode -> System.ConsoleColor.Red
        | Info -> System.ConsoleColor.Blue
        | Warning -> System.ConsoleColor.Yellow
    let originalColor = System.Console.ForegroundColor
    System.Console.ForegroundColor <- colour
    printfn $"%s{text}"
    System.Console.ForegroundColor <- originalColor

// Builder for the 'result' computation expression. This tells F#
// how to handle the 'do!' and 'let!' keywords for the CommandResult type.
type ResultBuilder() =
    member _.Bind(x, f) =
        match x with
        | Ok a -> f a
        | Error e -> Error e
    member _.Return(x) = Ok x
    member _.ReturnFrom(x) = x
    member _.Zero() = Ok ()
let result = ResultBuilder()

[<Verb("feature", HelpText = "Creates a new short-lived feature branch from 'main'.")>]
type FeatureOptions = {
    [<Option('n', "name", Required = true, HelpText = "Name of the feature (e.g., 'add-login-page').")>]
    Name: string
}

[<Verb("release", HelpText = "Creates a new short-lived release branch from 'main'.")>]
type ReleaseOptions = {
    [<Option('v', "version", Required = true, HelpText = "Version for the release branch (e.g., '1.0.0').")>]
    Version: string

    [<Option('f', "from-commit", HelpText = "Optional commit hash on 'main' to branch from.")>]
    FromCommit: string
}

[<Verb("hotfix", HelpText = "Creates a new short-lived hotfix branch from 'main'.")>]
type HotfixOptions = {
    [<Option('n', "name", Required = true, HelpText = "Name of the hotfix (e.g., 'fix-critical-bug').")>]
    Name: string
}

[<Verb("commit", HelpText = "Commits directly to the 'main' branch.")>]
type CommitOptions = {
    [<Option('t', "type", Required = true, HelpText = "Commit type (e.g., 'feat', 'fix', 'chore').")>]
    Type: string

    [<Option('s', "scope", HelpText = "Optional scope of the commit.")>]
    Scope: string

    [<Option('m', "message", Required = true, HelpText = "The commit message description.")>]
    Message: string

    [<Option('b', "breaking", HelpText = "Mark this commit as a breaking change.")>]
    Breaking: bool
}

[<Verb("complete", HelpText = "Merges a short-lived branch into 'main' and deletes it.")>]
type CompleteOptions = {
    [<Option('t', "type", Required = true, HelpText = "Type of branch to complete ('feature', 'release', 'hotfix').")>]
    Type: string

    [<Option('n', "name", Required = true, HelpText = "Name/version of the branch to complete.")>]
    Name: string
}

[<Verb("status", HelpText = "Shows the current git status.")>]
type StatusOptions() = class end

[<Verb("current-branch", HelpText = "Shows the current git branch name.")>]
type CurrentBranchOptions() = class end

/// Define Handlers using a Result-based workflow ///
// Handler for the 'release' command
let handleFeature (opts: FeatureOptions) =
    printfn "--- Creating feature branch ---"
    let branchName = $"feature/{opts.Name}"
    let workflow =
        result {
            let! _ = checkoutMain ()
            let! _ = pullLatestWithRebase ()
            let! _ = runGitCommand "checkout" $"-b {branchName}"
            return branchName
        }
    match workflow with
    | Ok bName -> // Give the value inside Ok a name, like bName
        printColour Success $"Feature branch '{bName}' created successfully."
        0
    | Error e ->
        printColour ErrorMode $"Error creating feature branch:\n{e}"
        1

// Handler for the 'release' command
let handleRelease (opts: ReleaseOptions) =
    printfn "--- Creating release branch ---"
    let branchName = $"release/{opts.Version}"
    let workflow =
        result {
            let! _ = checkoutMain ()
            let! _ = pullLatestWithRebase ()
            // If a commit is specified, use it. Otherwise, use the latest HEAD.
            let fromPoint = if not (System.String.IsNullOrEmpty(opts.FromCommit)) then opts.FromCommit else "HEAD"
            let! _ =  runGitCommand "checkout" $"-b {branchName} {fromPoint}"
            return branchName
        }
    match workflow with
    | Ok bName ->
        printColour Success $"Release branch '{bName}' created successfully."
        0
    | Error e ->
        printColour ErrorMode $"Error creating release branch:\n{e}"
        1

// Handler for the 'hotfix' command
let handleHotfix (opts: HotfixOptions) =
    printfn "--- Creating hotfix branch ---"
    let branchName = $"hotfix/{opts.Name}"
    let workflow =
        result {
            let! _ = checkoutMain ()
            let! _ = pullLatestWithRebase ()
            let! _ = runGitCommand "checkout" $"-b {branchName}"
            return branchName
        }
    match workflow with
    | Ok bName ->
        printColour Success $"Hotfix branch '{bName}' created successfully."
        0
    | Error e ->
        printColour ErrorMode $"Error creating hotfix branch:\n{e}"
        1

// Handler for the 'commit' command
let handleCommit (opts: CommitOptions) =
    printfn "--- Committing directly to main branch ---"
    let scopeStr = if not (System.String.IsNullOrEmpty(opts.Scope)) then $"({opts.Scope})" else ""
    let breakingStr = if opts.Breaking then "!" else ""
    let footer = if opts.Breaking then "\n\nBREAKING CHANGE: " + opts.Message else ""

    let header = $"{opts.Type}{scopeStr}{breakingStr}: {opts.Message}"
    let commitMessage = $"{header}{footer}"
    
    let workflow =
        result {
            let! _ = addAll ()
            let! _ = commit commitMessage
            let! _ = push ()
            return ()
        }
    
    match workflow with
    | Ok () ->
        printColour Success "Changes committed and pushed to main successfully."
        0
    | Error e ->
        printColour ErrorMode $"Error committing changes:\n{e}"
        1

// Handler for the 'complete' command
let handleComplete (opts: CompleteOptions) =
    printfn "--- Completing short-lived branch ---"
    
    let branchName = 
        match opts.Type.ToLower() with
        | "feature" | "hotfix" -> $"{opts.Type}/{opts.Name}"
        | "release" -> $"{opts.Type}/{opts.Name}"
        | _ -> failwith $"Invalid branch type '{opts.Type}'. Use 'feature', 'release', or 'hotfix'."

    printColour Info $"Branch to complete: {branchName}"
    
    let workflow =
        result {
            let! _ = checkoutMain()
            let! _ = pullLatestWithRebase()
            let! _ = mergeBranch branchName
            let! _ = push()
            let! _ = deleteLocalBranch branchName
            let! _ = deleteRemoteBranch branchName
            return ()
        }
        
    match workflow with
    | Ok _ ->
        printColour Success $"\nSuccess! Branch '{branchName}' was merged into main and deleted."
        0
    | Error e ->
        printColour ErrorMode $"\nWorkflow failed:\n{e}"
        1
// Handler for the 'status' command
let handleStatus (_: StatusOptions) =
    printfn "--- Git Status ---"
    match runGitCommand "status" "--porcelain" with
    | Ok output ->
        printColour Info output
        0
    | Error e ->
        printColour ErrorMode $"Error running git status:\n{e}"
        1

// Handler for the 'current-branch' command
let handleCurrentBranch (_: CurrentBranchOptions) =
    match getCurrentBranch () with
    | Ok branchName ->
        printColour Success $"Current branch is: {branchName}"
        0
    | Error e ->
        printColour ErrorMode $"Error getting current branch:\n{e}"
        1

// Main function to parse command line arguments and execute the appropriate handler
[<EntryPoint>]
let main argv =
    let parser = Parser.Default
    let result = parser.ParseArguments<FeatureOptions, ReleaseOptions, HotfixOptions, CommitOptions, CompleteOptions, StatusOptions, CurrentBranchOptions>(argv)
    
    result.MapResult(
        (fun opts -> handleFeature opts |> int),
        (fun opts -> handleRelease opts |> int),
        (fun opts -> handleHotfix opts |> int),
        (fun opts -> handleCommit opts |> int),
        (fun opts -> handleComplete opts |> int),
        (fun opts -> handleStatus opts |> int),
        (fun opts -> handleCurrentBranch opts |> int),
        (fun _ -> printColour ErrorMode "Invalid command. Use --help for usage information."; 1)
    )
