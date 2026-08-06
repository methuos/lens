pub mod git;
pub mod language;
pub mod largest;
pub mod rust;

use anyhow::Result;

use crate::inventory::Inventory;

pub fn run(inventory: &mut Inventory) -> Result<()> {
    language::detect_languages(inventory);
    rust::analyze(inventory)?;
    largest::analyze(inventory)?;
    git::analyze(inventory)?;

    Ok(())
}
