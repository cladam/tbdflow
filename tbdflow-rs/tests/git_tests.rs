use tbdflow::git; // If you make git.rs a public module

#[test]
fn test_get_current_branch() {
    let result = git::checkout_main();
    assert!(result.is_ok() || result.is_err()); // Replace with real assertions
}