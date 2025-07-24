use tempfile::{tempdir, TempDir};
use std::fs::write;
use std::process::Command;

/// Sets up a temporary Git repository for testing purposes.
pub fn setup_temp_git_repo() -> (TempDir, TempDir, std::path::PathBuf) {
    let dir = tempdir().expect("create temp dir");
    let repo_path = dir.path().to_path_buf();

    // Create a bare repo to act as 'origin'
    let bare_dir = tempdir().expect("create bare repo");
    let bare_repo_path = bare_dir.path().to_path_buf();
    Command::new("git").arg("init").arg("--bare").current_dir(&bare_repo_path).output().unwrap();

    Command::new("git").arg("init").current_dir(&repo_path).output().unwrap();
    Command::new("git").args(&["config", "user.email", "test@example.com"]).current_dir(&repo_path).output().unwrap();
    Command::new("git").args(&["config", "user.name", "Test"]).current_dir(&repo_path).output().unwrap();
    let file_path = repo_path.join("README.md");
    write(&file_path, "test").unwrap();
    Command::new("git").args(&["add", "."]).current_dir(&repo_path).output().unwrap();
    Command::new("git").args(&["commit", "-m", "init"]).current_dir(&repo_path).output().unwrap();

    // Add local bare repo as remote
    Command::new("git")
        .args(&["remote", "add", "origin", bare_repo_path.to_str().unwrap()])
        .current_dir(&repo_path)
        .output()
        .unwrap();

    // Push main to origin to set up tracking
    Command::new("git")
        .args(&["push", "-u", "origin", "main"])
        .current_dir(&repo_path)
        .output()
        .unwrap();

    let status = Command::new("git")
        .args(&["status", "--porcelain"])
        .current_dir(&repo_path)
        .output()
        .unwrap();
    assert!(status.stdout.is_empty(), "Repo not clean after setup: {:?}", String::from_utf8_lossy(&status.stdout));

    (dir, bare_dir, repo_path)
}