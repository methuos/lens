mod analyzers;
mod inventory;
mod output;
mod scanner;
mod utils;

use anyhow::Result;
use std::path::Path;

fn main() -> Result<()> {
    let inventory = scanner::scan(Path::new("."))?;

    output::terminal::print(&inventory);

    Ok(())
}
