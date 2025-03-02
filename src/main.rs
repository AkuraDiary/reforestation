mod args;
mod commit;
mod git;
mod utils;

use args::Args;
use chrono::DateTime;
use chrono::Utc;
use clap::Parser;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args: Args = Args::parse();

    let target_path: PathBuf = utils::resolve_path(&args.dir);
    fs::create_dir_all(&target_path).expect("Failed to create directory");
    env::set_current_dir(&target_path).expect("Failed to change directory");

    git::initialize_git(&args.repo);

    let now: DateTime<Utc> = Utc::now();
    commit::generate_commits(&args, now);

    git::finalize_and_push();

    println!(
        "\n🎉 GitHub commits successfully generated in: {}",
        args.dir
    );
}
