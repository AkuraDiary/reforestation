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

    // check weither the args is containing init or not
    let mut repo_name = "";
    if args.init {
        let target_path: PathBuf = utils::resolve_path(&args.dir);
        fs::create_dir_all(&target_path).expect("Failed to create directory");
        env::set_current_dir(&target_path).expect("Failed to change directory");
        git::initialize_git(&args.repo);
    } else {
        git::clone_git(&args.repo);
        
        repo_name = args.repo.split('/').last().unwrap_or_default();
        repo_name = repo_name.strip_suffix(".git").unwrap_or(repo_name);
        let cloned_repo_path = PathBuf::from(repo_name);
        
        env::set_current_dir(&cloned_repo_path).expect("Failed to change to cloned repository directory");

    }
    
    let now: DateTime<Utc> = Utc::now();
    commit::generate_commits(&args, now);

    git::finalize_and_push(&args.init);

    println!(
        "\n🎉 GitHub commits successfully generated in: {}",
        if !repo_name.is_empty() { repo_name } else { &args.dir }
    );
}
