// ===============================================================
// FILE: src/main.rs
// ===============================================================
// Project: tbdflow-rs - Trunk-Based Development Git CLI
// Description: Entry point for the CLI application.
// Author: Claes Adamsson @cladam
// ===============================================================

use colored::Colorize;

/// A helper to print the result of a workflow.
fn print_workflow_result(result: Result<String, String>, success_message: String) {
    match result {
        Ok(_) => println!("\n{}", success_message.green()),
        Err(e) => println!("\n{}", format!("Workflow failed:\n{}", e).red()),
    }
}

fn main() {
    println!("Hello, world!");
}
