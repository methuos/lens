use anyhow::Result;

use crate::inventory::Inventory;

pub fn analyze(inventory: &mut Inventory) -> Result<()> {
    inventory.largest_files = inventory.files_data.clone();

    inventory
        .largest_files
        .sort_by(|a, b| b.lines.cmp(&a.lines));

    inventory.largest_files.truncate(5);

    Ok(())
}
