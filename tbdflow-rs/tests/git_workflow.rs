use tempfile::{tempdir, TempDir};
use std::fs::{File, write};
use std::env;
use std::process::Command;
use tbdflow::git;

fn setup_temp_git_repo() -> (TempDir, std::path::PathBuf) {
    let dir = tempdir().expect("create temp dir");
    let repo_path = dir.path().to_path_buf();
    Command::new("git").arg("init").current_dir(&repo_path).output().unwrap();
    Command::new("git").args(&["config", "user.email", "test@example.com"]).current_dir(&repo_path).output().unwrap();
    Command::new("git").args(&["config", "user.name", "Test"]).current_dir(&repo_path).output().unwrap();
    let file_path = repo_path.join("README.md");
    write(&file_path, "test").unwrap();
    Command::new("git").args(&["add", "."]).current_dir(&repo_path).output().unwrap();
    Command::new("git").args(&["commit", "-m", "init"]).current_dir(&repo_path).output().unwrap();
    (dir, repo_path)
}

#[test]
fn test_clean_working_directory() {
    let (_dir, repo_path) = setup_temp_git_repo();
    let old_dir = env::current_dir().unwrap();
    env::set_current_dir(&repo_path).unwrap();

    let result = git::is_working_directory_clean();
    assert!(result.is_ok(), "Expected Ok, got {:?}", result);

    env::set_current_dir(old_dir).unwrap();
}

#[test]
fn test_dirty_working_directory() {
    let (_dir, repo_path) = setup_temp_git_repo();
    let old_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&repo_path).unwrap();

    let file_path = repo_path.join("README.md");
    std::fs::write(&file_path, "changed").unwrap();

    let result = git::is_working_directory_clean();
    assert!(result.is_err(), "Expected Err, got {:?}", result);

    std::env::set_current_dir(old_dir).unwrap();
}