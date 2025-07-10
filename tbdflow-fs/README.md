## Installation & Publishing

You can run the tool directly from the source code for development or publish it as a standalone executable for easy, system-wide use.

### Running from Source
1.  **Prerequisites:** You must have the [.NET SDK](https://dotnet.microsoft.com/download) installed.
2.  **Clone the repository:** `git clone <your-repo-url>`
3.  **Run the tool:** All commands are run from the project's root directory using `dotnet run --`.

### Publishing an Executable
To create a standalone executable that you can run from anywhere:

1.  **Publish the application.** For an Apple Silicon Mac, use:
    ```bash
    dotnet publish -c Release -r osx-arm64 --self-contained true -p:PublishSingleFile=true
    ```
2.  **Locate the executable.** It will be in the `bin/Release/net8.0/osx-arm64/publish/` directory.
3.  **(Optional) Add to your PATH.** Copy the executable to a directory in your system's PATH (e.g., `/usr/local/bin`) to make it callable from any terminal session.
