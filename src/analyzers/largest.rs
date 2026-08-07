use anyhow::Result;

use crate::inventory::Inventory;

pub fn analyze(inventory: &mut Inventory) -> Result<()> {
    inventory.largest_files = inventory.files_data.clone();

    inventory
        .largest_files
        .sort_by_key(|x| std::cmp::Reverse(x.lines));

    inventory.largest_files.truncate(5);

    Ok(())
}
