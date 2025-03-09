use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    version,
    author = "Ahsan Azizan, contact@ahsanzizan.xyz",
    about = "Generates dummy GitHub commit history",
    version = "1.1.0",
    help_template = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
"
)]
pub struct Args {
    /// Number of days to go back
    #[arg(long, default_value_t = 366)]
    pub days: i32,

    /// Number of commits per day
    #[arg(long, default_value_t = 10)]
    pub freq: i32,

    /// Randomize commit frequency
    #[arg(long)]
    pub random: bool,

    /// GitHub repository URL
    #[arg(long)]
    pub repo: String,

    /// Target directory for commits
    #[arg(long, default_value = "dummy-git-repo")]
    pub dir: String,

    // Additional Argument to initalise the repo, 
    // I changed the default --repo args to call the function to clone repo instead of initializing it
    /// Initialize new repository from repository link provided instead of cloning
    #[arg(long)]
    pub init : bool
}
