use clap::{Parser, Subcommand, ValueEnum};

#[derive(Clone, ValueEnum)]
pub enum SortBy {
    Path,
    Size,
    Lines,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum OutputFormat {
    Terminal,
    Json,
}

#[derive(Subcommand)]
pub enum Commands {
    Summary,
    Git,
    Rust,
    Languages,
    Largest,
    Tree,
    Search {
        #[arg()]
        query: String,

        #[arg(long, value_enum, default_value = "terminal")]
        format: OutputFormat,
    },

    Files {
        #[arg(long, value_enum, default_value = "path")]
        sort: SortBy,

        #[arg(long)]
        reverse: bool,

        #[arg(long, value_enum, default_value = "terminal")]
        format: OutputFormat,
    },
    Dirs {
        #[arg(long, value_enum, default_value = "path")]
        sort: SortBy,

        #[arg(long)]
        reverse: bool,

        #[arg(long, value_enum, default_value = "terminal")]
        format: OutputFormat,
    },
}

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(default_value = ".")]
    pub path: std::path::PathBuf,

    #[command(subcommand)]
    pub command: Option<Commands>,
}
