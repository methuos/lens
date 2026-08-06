mod analyzers;
mod cli;
mod inventory;
mod output;
mod scanner;
mod utils;

use crate::cli::{Cli, Commands};
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let inventory = scanner::scan(&cli.path)?;

    match cli.command {
        None | Some(Commands::Summary) => {
            output::terminal::print(&inventory);
        }

        Some(Commands::Files {
            sort,
            reverse,
            format,
        }) => {
            output::files::print(&inventory, sort, reverse, format);
        }

        Some(Commands::Dirs {
    sort,
    reverse,
    format,
}) => {
    output::dirs::print(&inventory, sort, reverse, format);
}

        Some(Commands::Tree) => {
            output::tree::print(&inventory);
        }

        Some(Commands::Git) => {
            output::terminal::print_git(&inventory);
        }

        Some(Commands::Rust) => {
            output::terminal::print_rust(&inventory);
        }

        Some(Commands::Languages) => {
            output::terminal::print_languages(&inventory);
        }

        Some(Commands::Largest) => {
            output::terminal::print_largest(&inventory);
        }
    }

    Ok(())
}
