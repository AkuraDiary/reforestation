use crate::args::Args;
use chrono::{DateTime, Duration, Utc};
use rand::rngs::ThreadRng;
use rand::Rng;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command as Cmd;

pub fn generate_commits(args: &Args, now: DateTime<Utc>) {
    let mut rng: ThreadRng = rand::rng();
    let mut commit_counter: i32 = 1;

    for day in 0..args.days {
        let commit_date: DateTime<Utc> = now - Duration::days(day as i64);
        let formatted_date: String = commit_date.format("%Y-%m-%d").to_string();
        let daily_commits: i32 = if args.random {
            rng.random_range(1..=args.freq)
        } else {
            args.freq
        };

        for _ in 0..daily_commits {
            let commit_time: String = random_time(&mut rng);

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
}

fn random_time(rng: &mut ThreadRng) -> String {
    let hour: i32 = rng.random_range(0..24);
    let min: i32 = rng.random_range(0..60);
    let sec: i32 = rng.random_range(0..60);
    format!("{:02}:{:02}:{:02}", hour, min, sec)
}
