use std::borrow::Cow;
use std::process::Command as Cmd;
use std::process::Output;

pub fn initialize_git(repo_url: &str) {
    if !std::fs::metadata(".git").is_ok() {
        Cmd::new("git")
            .arg("init")
            .status()
            .expect("Failed to initialize Git");
    }
    let remote_output: Output = Cmd::new("git")
        .args(&["remote", "-v"])
        .output()
        .expect("Failed to check remote");

    let remote_str: Cow<'_, str> = String::from_utf8_lossy(&remote_output.stdout);
    // let remote_str: Cow<'_, str> = String::from_utf8_lossy(&remote_output.stdout);

    if !remote_str.contains("origin") {
        Cmd::new("git")
            .args(&["remote", "add", "origin", repo_url])
            .status()
            .expect("Failed to add remote");
    }
}

// Instead of initializing new repo let's clone it first, for saver ride
// this is for using existing repo, suppose the user is already had arepo with some files in it
pub fn clone_git(repo_url: &str) {
    if !std::fs::metadata(".git").is_ok() {
        Cmd::new("git")
            .args(&["clone", repo_url])
            .status()
            .expect("Failed to clone Git");
    }

    // let remote_output: Output = Cmd::new("git")
    //     .args(&["remote", "-v"])
    //     .output()
    //     .expect("Failed to check remote");
}

pub fn finalize_and_push(is_init: &bool) {

    if *is_init {
        Cmd::new("git")
        .args(&["branch", "-M", "main"])
        .status()
        .expect("Failed to rename branch");

    Cmd::new("git")
        .args(&["push", "-u", "origin", "main", "-f"])
        .status()
        .expect("Failed to push");
    } else{
        // assume the repo is already linked, then we just need to push it
        Cmd::new("git")
        .arg("push")
        .status()
        .expect("Failed to push");
    }
    
}



