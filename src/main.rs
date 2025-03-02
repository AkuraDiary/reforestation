use clap::Parser;
use std::env;
use std::fs;
use std::process::Command;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let target_dir = "dummy-git-repo";
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }

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
