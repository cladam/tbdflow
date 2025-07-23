use assert_cmd::Command;
use predicates::str::contains;
use serial_test::serial;

mod util;
use util::setup_temp_git_repo;

#[test]
fn test_status_command() {
    let mut cmd = Command::cargo_bin("tbdflow").unwrap();
    cmd.arg("status");
    cmd.assert()
        .success()
        .stdout(contains("Git Status"));
}

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
    cmd.arg("commit")
        .arg("--type").arg("feat")
        .arg("--scope").arg("ui")
        .arg("--message").arg("Add new button")
        .arg("--breaking");
    cmd.assert()
        .success()
        .stdout(contains("Successfully committed and pushed changes to main."));
}

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
        .stdout(contains("Syncing branches with remote"));
}