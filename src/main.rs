use chrono::DateTime;
use chrono::{Duration, Utc};
use clap::Parser;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::env;
use std::fs::OpenOptions;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command as Cmd;
use std::process::Output;

#[derive(Parser, Debug)]
#[command(version, about = "Generates dummy GitHub commit history")]
struct Args {
    /// Number of days to go back
    #[arg(long, default_value_t = 366)]
    days: i32,

    /// Number of commits per day
    #[arg(long, default_value_t = 10)]
    freq: i32,

    /// Randomize commit frequency
    #[arg(long)]
    random: bool,

    /// GitHub repository URL
    #[arg(long)]
    repo: String,

    /// Target directory for commits
    #[arg(long, default_value = "dummy-git-repo")]
    dir: String,
}

fn main() {
    let args: Args = Args::parse();

    let total_days: i32 = args.days;
    let commit_freq: i32 = args.freq;
    let randomize: bool = args.random;
    let repo_url: &String = &args.repo;
    let target_dir: &String = &args.dir;

    // Create target directory if it doesn't exist
    let target_path: PathBuf = fs::canonicalize(target_dir).unwrap_or_else(|_| target_dir.into());
    fs::create_dir_all(&target_path).expect("Failed to create directory");
    env::set_current_dir(&target_path).expect("Failed to change directory");

    // Initialize Git repository if not already initialized
    if !fs::metadata(".git").is_ok() {
        Cmd::new("git")
            .arg("init")
            .status()
            .expect("Failed to initialize Git");
    }

    // Check if remote repo is already set, otherwise add it
    let remote_output: Output = Cmd::new("git")
        .args(&["remote", "-v"])
        .output()
        .expect("Failed to check remote");
    let remote_str = String::from_utf8_lossy(&remote_output.stdout);
    if !remote_str.contains("origin") {
        Cmd::new("git")
            .args(&["remote", "add", "origin", repo_url])
            .status()
            .expect("Failed to add remote");
    }

    let now: DateTime<Utc> = Utc::now();
    let mut rng: ThreadRng = rand::rng();
    let mut commit_counter = 1;

    // Generate commits for past `total_days`
    for day in 0..total_days {
        let commit_date: DateTime<Utc> = now - Duration::days(day as i64);
        let formatted_date: String = commit_date.format("%Y-%m-%d").to_string();
        let daily_commits: i32 = if randomize {
            rng.random_range(1..=commit_freq)
        } else {
            commit_freq
        };

        for _ in 0..daily_commits {
            let hour: i32 = rng.random_range(0..24);
            let min: i32 = rng.random_range(0..60);
            let sec: i32 = rng.random_range(0..60);
            let commit_time: String = format!("{:02}:{:02}:{:02}", hour, min, sec);

            let mut file: File = OpenOptions::new()
                .append(true)
                .create(true)
                .open("commit.txt")
                .expect("Failed to open file");
            writeln!(
                file,
                "Commit {}: {} at {}",
                commit_counter, formatted_date, commit_time
            )
            .expect("Failed to write");

            Cmd::new("git")
                .args(&["add", "commit.txt"])
                .status()
                .expect("Failed to add");
            Cmd::new("git")
                .args(&[
                    "commit",
                    "--date",
                    &format!("{} {}", formatted_date, commit_time),
                    "-m",
                    &format!("Commit {}", commit_counter),
                ])
                .status()
                .expect("Failed to commit");

            println!(
                "✅ Commit {} on {} at {}",
                commit_counter, formatted_date, commit_time
            );
            commit_counter += 1;
        }
    }

    // Push commits
    Cmd::new("git")
        .args(&["branch", "-M", "main"])
        .status()
        .expect("Failed to rename branch");
    Cmd::new("git")
        .args(&["push", "-u", "origin", "main", "-f"])
        .status()
        .expect("Failed to push");

    println!(
        "\n🎉 GitHub commits successfully generated in: {}",
        target_dir
    );
}
