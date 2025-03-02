use std::env;
use std::fs;
use std::process::Command;

fn main() {
    let target_dir = "dummy-git-repo";

    // Create directory if it doesn't exist
    fs::create_dir_all(target_dir).expect("Failed to create directory");

    // Change to target directory
    env::set_current_dir(target_dir).expect("Failed to change directory");

    // Initialize Git repository if not already initialized
    if !fs::metadata(".git").is_ok() {
        Command::new("git")
            .arg("init")
            .status()
            .expect("Failed to initialize Git repository");
    }

    println!("✅ Git repository initialized in {}", target_dir);
}
