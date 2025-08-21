use assert_cmd::Command;
use chrono::{Duration, Utc};
use predicates::str::contains;
use predicates::str::is_match;
use serial_test::serial;

mod util;
use util::setup_temp_git_repo;

/// Tests that the status command outputs the expected status message.
#[test]
#[serial]
fn test_status_command() {
    // Add the setup function to create a git repo for the test
    let (_dir, _bare_dir, repo_path) = setup_temp_git_repo();
    std::env::set_current_dir(&repo_path).unwrap();

    let mut cmd = Command::cargo_bin("tbdflow").unwrap();
    cmd.arg("--verbose").arg("status");
    cmd.assert().success().stdout(contains("Checking status"));
}

/// Tests that the current branch command outputs the expected branch name.
#[test]
#[serial]
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
    cmd.assert().success().stdout(contains(
        "Success! Switched to new feature branch: 'feature_new-feature'",
    ));
}

/// Tests that creating a new release branch called "1.0.0" works correctly.
#[test]
#[serial]
fn test_create_release_branch_command() {
    let (_dir, _bare_dir, repo_path) = setup_temp_git_repo();
    std::env::set_current_dir(&repo_path).unwrap();
    let mut cmd = Command::cargo_bin("tbdflow").unwrap();
    cmd.arg("release").arg("--version").arg("1.0.0");
    cmd.assert().success().stdout(contains(
        "Success! Switched to new release branch: 'release_1.0.0'",
    ));
}

/// Tests that creating a new hotfix branch called "urgent-fix" works correctly.
#[test]
#[serial]
fn test_create_hotfix_branch_command() {
    let (_dir, _bare_dir, repo_path) = setup_temp_git_repo();
    std::env::set_current_dir(&repo_path).unwrap();
    let mut cmd = Command::cargo_bin("tbdflow").unwrap();
    cmd.arg("hotfix").arg("--name").arg("urgent-fix");
    cmd.assert().success().stdout(contains(
        "Success! Switched to new hotfix branch: 'hotfix_urgent-fix'",
    ));
}

/// Tests that adding a new file and committing it with the commit command works correctly.
/// Skipping .dod.yml verification for simplicity in this test.
#[test]
#[serial]
fn test_commit_command() {
    let (_dir, _bare_dir, repo_path) = setup_temp_git_repo();
    std::env::set_current_dir(&repo_path).unwrap();

    // Create a file to commit
    std::fs::write(repo_path.join("BUTTON.md"), "this is a new button ■").unwrap();
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
        .arg("--type")
        .arg("feat")
        .arg("--scope")
        .arg("ui")
        .arg("--message")
        .arg("add new button")
        .arg("--body")
        .arg("This button is used for submitting forms.")
        .arg("--breaking")
        .arg("--tag")
        .arg("button-v1");
    cmd.assert().success().stdout(
        /*
        Depending on Git version used, output may vary slightly. For example:
        - Successfully committed and pushed changes to BRANCH_NAME.
        - Successfully pushed changes to 'BRANCH_NAME'.

        Also default branch name can be 'main' or 'master', depending on the Git version
        and distribution. Below is a regex that matches all those cases.
        */
        is_match(r"Successfully (?:committed and )?pushed changes to '?(?:main|master)'?\.")
            .unwrap(),
    );

    // Check that the tag exists
    let output = std::process::Command::new("git")
        .arg("tag")
        .current_dir(&repo_path)
        .output()
        .unwrap();
    let tags = String::from_utf8_lossy(&output.stdout);
    assert!(
        tags.contains("button-v1"),
        "Expected tag button-v1 not found. Tags: {}",
        tags
    );
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
        .arg("--type")
        .arg("feature")
        .arg("--name")
        .arg("new-feature");
    cmd.assert()
        .success()
        .stdout(contains("Branch to complete: feature_new-feature"));
}

/// Tests that completing a release branch called "1.0.0" works correctly.
#[test]
#[serial]
fn test_complete_release_branch_command() {
    let (_dir, _bare_dir, repo_path) = setup_temp_git_repo();
    std::env::set_current_dir(&repo_path).unwrap();

    // Create the release branch first
    let mut create_cmd = Command::cargo_bin("tbdflow").unwrap();
    create_cmd
        .arg("--verbose")
        .arg("release")
        .arg("--version")
        .arg("1.0.0");
    create_cmd.assert().success();

    let mut cmd = Command::cargo_bin("tbdflow").unwrap();
    cmd.arg("--verbose")
        .arg("complete")
        .arg("--type")
        .arg("release")
        .arg("--name")
        .arg("1.0.0");
    cmd.assert()
        .success()
        .stdout(contains("Branch to complete: release_1.0.0"));

    // Check that the tag exists
    let output = std::process::Command::new("git")
        .arg("tag")
        .current_dir(&repo_path)
        .output()
        .unwrap();
    let tags = String::from_utf8_lossy(&output.stdout);
    assert!(
        tags.contains("v1.0.0"),
        "Expected tag v1.0.0 not found. Tags: {}",
        tags
    );
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
    // Switch to main branch to ensure the stale branch is not the current one
    std::process::Command::new("git")
        .args(&["checkout", "main"])
        .current_dir(&repo_path)
        .output()
        .unwrap();

    let mut cmd = Command::cargo_bin("tbdflow").unwrap();
    cmd.arg("check-branches");
    cmd.assert()
        .success()
        .stdout(contains("Warning: The following branches may be stale:"))
        .stdout(contains("feature_stale-feature"));
}
