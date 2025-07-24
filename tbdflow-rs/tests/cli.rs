use assert_cmd::Command;
use predicates::str::contains;
use serial_test::serial;
use chrono::{Duration, Utc};

mod util;
use util::setup_temp_git_repo;

/// Tests that the status command outputs the expected status message.
#[test]
fn test_status_command() {
    let mut cmd = Command::cargo_bin("tbdflow").unwrap();
    cmd.arg("status");
    cmd.assert()
        .success()
        .stdout(contains("Git Status"));
}

/// Tests that the current branch command outputs the expected branch name.
#[test]
fn test_current_branch_command() {
    let (_dir, _bare_dir, repo_path) = setup_temp_git_repo();
    std::env::set_current_dir(&repo_path).unwrap();
    let mut cmd = Command::cargo_bin("tbdflow").unwrap();
    cmd.arg("current-branch");
    cmd.assert()
        .success()
        .stdout(contains("Current branch is:"));
}

/// Tests that creating a new feature branch called "new-feature" works correctly.
#[test]
#[serial]
fn test_create_feature_branch_command() {
    let (_dir, _bare_dir, repo_path) = setup_temp_git_repo();
    std::env::set_current_dir(&repo_path).unwrap();
    let mut cmd = Command::cargo_bin("tbdflow").unwrap();
    cmd.arg("feature").arg("--name").arg("new-feature");
    cmd.assert()
        .success()
        .stdout(contains("Success! Switched to new feature branch: 'feature/new-feature'"));
}

/// Tests that creating a new release branch called "1.0.0" works correctly.
#[test]
#[serial]
fn test_create_release_branch_command() {
    let (_dir, _bare_dir, repo_path) = setup_temp_git_repo();
    std::env::set_current_dir(&repo_path).unwrap();
    let mut cmd = Command::cargo_bin("tbdflow").unwrap();
    cmd.arg("release").arg("--version").arg("1.0.0");
    cmd.assert()
        .success()
        .stdout(contains("Success! Switched to new release branch: 'release/1.0.0'"));
}

/// Tests that creating a new hotfix branch called "urgent-fix" works correctly.
#[test]
#[serial]
fn test_create_hotfix_branch_command() {
    let (_dir, _bare_dir, repo_path) = setup_temp_git_repo();
    std::env::set_current_dir(&repo_path).unwrap();
    let mut cmd = Command::cargo_bin("tbdflow").unwrap();
    cmd.arg("hotfix").arg("--name").arg("urgent-fix");
    cmd.assert()
        .success()
        .stdout(contains("Success! Switched to new hotfix branch: 'hotfix/urgent-fix'"));
}

/// Tests that adding a new file and committing it with the commit command works correctly.
#[test]
#[serial]
fn test_commit_command() {
    let (_dir, _bare_dir, repo_path) = setup_temp_git_repo();
    std::env::set_current_dir(&repo_path).unwrap();

    // Create a file to commit
    std::fs::write(repo_path.join("BUTTON.md"), "This is a new button ■").unwrap();
    // Stage the file
    Command::new("git")
        .arg("add")
        .arg("BUTTON.md")
        .current_dir(&repo_path)
        .output()
        .unwrap();

    // Wait until the working directory is clean
    let mut retries = 5;
    while retries > 0 {
        let status = Command::new("git")
            .args(&["status", "--porcelain"])
            .current_dir(&repo_path)
            .output()
            .unwrap();
        if status.stdout.is_empty() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
        retries -= 1;
    }

    let mut cmd = Command::cargo_bin("tbdflow").unwrap();
    // Run the commit command with a feature type, scope, message, and breaking change flag
    cmd.arg("commit")
        .arg("--type").arg("feat")
        .arg("--scope").arg("ui")
        .arg("--message").arg("Add new button")
        .arg("--breaking");
    cmd.assert()
        .success()
        .stdout(contains("Successfully committed and pushed changes to main."));
}

/// Tests that completing a feature branch called "new-feature" works correctly.
#[test]
#[serial]
fn test_complete_feature_branch_command() {
    let (_dir, _bare_dir, repo_path) = setup_temp_git_repo();
    std::env::set_current_dir(&repo_path).unwrap();

    // Create the feature branch first
    let mut create_cmd = Command::cargo_bin("tbdflow").unwrap();
    create_cmd.arg("feature").arg("--name").arg("new-feature");
    create_cmd.assert().success();

    let mut cmd = Command::cargo_bin("tbdflow").unwrap();
    cmd.arg("complete")
        .arg("--type").arg("feature")
        .arg("--name").arg("new-feature");
    cmd.assert()
        .success()
        .stdout(contains("Branch to complete: feature/new-feature"));
}

/// Testing the synch command to ensure it pulls changes from the remote repository
/// We will simulate a remote change by pushing to a bare repository and then running the sync command.
#[test]
#[serial]
fn test_sync_command() {
    let (_dir, bare_dir, repo_path) = setup_temp_git_repo();

    // Simulate a team member pushing changes to the remote
    let remote_repo_url = bare_dir.path().to_str().unwrap();
    let second_clone_dir = tempfile::tempdir().unwrap();
    std::process::Command::new("git")
        .args(&["clone", remote_repo_url, "."])
        .current_dir(second_clone_dir.path())
        .output()
        .unwrap();

    std::fs::write(second_clone_dir.path().join("REMOTE.md"), "remote change").unwrap();

    std::process::Command::new("git")
        .args(&["config", "user.email", "teammate@example.com"])
        .current_dir(second_clone_dir.path())
        .output()
        .unwrap();
    std::process::Command::new("git")
        .args(&["config", "user.name", "Teammate"])
        .current_dir(second_clone_dir.path())
        .output()
        .unwrap();

    std::process::Command::new("git")
        .args(&["add", "."])
        .current_dir(second_clone_dir.path())
        .output()
        .unwrap();
    std::process::Command::new("git")
        .args(&["commit", "-m", "feat: add remote file"])
        .current_dir(second_clone_dir.path())
        .output()
        .unwrap();
    std::process::Command::new("git")
        .args(&["push"])
        .current_dir(second_clone_dir.path())
        .output()
        .unwrap();

    // Now run the sync command
    std::env::set_current_dir(&repo_path).unwrap();
    let mut cmd = Command::cargo_bin("tbdflow").unwrap();
    cmd.arg("sync");
    cmd.assert()
        .success()
        .stdout(contains("Syncing with remote"));
}

/// Tests that the 'check-branches' lists and warns about branches that are stale (older than 1 day)
#[test]
#[serial]
fn test_check_branches_command() {
    let (_dir, _bare_dir, repo_path) = setup_temp_git_repo();
    std::env::set_current_dir(&repo_path).unwrap();

    // Create a feature branch that is stale
    let mut create_cmd = Command::cargo_bin("tbdflow").unwrap();
    create_cmd.arg("feature").arg("--name").arg("stale-feature");
    create_cmd.assert().success();

    let old_date = (Utc::now() - Duration::hours(25)).to_rfc3339();

    std::process::Command::new("git")
        .args(&["commit", "--allow-empty", "-m", "stale commit"])
        .env("GIT_COMMITTER_DATE", &old_date) // Set the committer date
        .current_dir(&repo_path)
        .output()
        .unwrap();

    let mut cmd = Command::cargo_bin("tbdflow").unwrap();
    cmd.arg("check-branches");
    cmd.assert()
        .success()
        .stdout(contains("Warning: The following branches may be stale:"))
        .stdout(contains("feature/stale-feature"));
}