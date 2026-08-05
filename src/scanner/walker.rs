use anyhow::Result;
use ignore::WalkBuilder;

use super::counter::count_lines;
use super::ignore::IGNORED_DIRS;
use crate::inventory::Inventory;

pub fn collect_files(inventory: &mut Inventory) -> Result<()> {
    let walker = WalkBuilder::new(&inventory.root)
        .hidden(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .filter_entry(|entry| {
            let name = entry.file_name().to_string_lossy();
            !IGNORED_DIRS.iter().any(|dir| *dir == name)
        })
        .build();

    for entry in walker {
        let entry = entry?;

        let Some(file_type) = entry.file_type() else {
            continue;
        };

        if file_type.is_dir() {
            inventory.directories += 1;
            continue;
        }

        if !file_type.is_file() {
            continue;
        }

        inventory.files += 1;

        let metadata = entry.metadata()?;
        inventory.total_size += metadata.len();

        if let Ok(lines) = count_lines(entry.path()) {
            inventory.total_lines += lines;
        }

        if let Some(ext) = entry.path().extension() {
            let ext = ext.to_string_lossy().to_lowercase();
            *inventory.extensions.entry(ext).or_insert(0) += 1;
        }
    }

    Ok(())
}
