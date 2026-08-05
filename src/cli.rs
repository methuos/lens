use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "Lens",
    version,
    about = "Analyze a project and generate an inventory report."
)]
pub struct Cli {
    /// Path to the project
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Output Markdown report
    #[arg(long)]
    pub markdown: bool,

    /// Output JSON report
    #[arg(long)]
    pub json: bool,
}
