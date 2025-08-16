use serial_test::serial;
use std::fs::write;
use std::env;
use tbdflow::git;
mod util;
use util::setup_temp_git_repo;

#[test]
#[serial]
fn test_clean_working_directory() {
    let verbose = true;
    let (_dir, _bare_dir, repo_path) = setup_temp_git_repo();
    let old_dir = env::current_dir().unwrap();
    env::set_current_dir(&repo_path).unwrap();

    let result = git::is_working_directory_clean(verbose);
    assert!(result.is_ok(), "Expected Ok, got {:?}", result);

    env::set_current_dir(old_dir).unwrap();
}

#[test]
#[serial]
fn test_dirty_working_directory() {
    let verbose = true;
    let (_dir, _bare_dir, repo_path) = setup_temp_git_repo();
    let old_dir = env::current_dir().unwrap();
    env::set_current_dir(&repo_path).unwrap();

    let file_path = repo_path.join("README.md");
    write(&file_path, "changed").unwrap();

    // print contents of README.md to verify change
    let contents = std::fs::read_to_string(&file_path).unwrap();
    println!("Contents of README.md: {}", contents);

    let result = git::is_working_directory_clean(verbose);
    assert!(result.is_err(), "Expected Err, got {:?}", result);

    env::set_current_dir(old_dir).unwrap();
}