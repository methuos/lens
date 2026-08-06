use anyhow::Result;
use ignore::WalkBuilder;

use super::counter::count_lines;
use super::ignore::IGNORED_DIRS;
use crate::inventory::{DirectoryInfo, FileInfo, Inventory};

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
        let size = metadata.len();

        inventory.total_size += size;

        let lines = count_lines(entry.path()).unwrap_or(0);
        inventory.total_lines += lines;

        let extension = entry
            .path()
            .extension()
            .map(|ext| ext.to_string_lossy().to_lowercase());

        let path = entry
            .path()
            .strip_prefix(&inventory.root)
            .unwrap_or(entry.path())
            .display()
            .to_string();

        inventory.files_data.push(FileInfo {
            path: path.clone(),
            size,
            lines,
        });

        let dir = std::path::Path::new(&path)
            .parent()
            .unwrap_or(std::path::Path::new(""))
            .display()
            .to_string();

        if let Some(existing) = inventory
            .directories_data
            .iter_mut()
            .find(|d| d.path == dir)
        {
            existing.files += 1;
            existing.lines += lines;
            existing.size += size;
        } else {
            inventory.directories_data.push(DirectoryInfo {
                path: dir,
                files: 1,
                lines,
                size,
            });
        }

        if let Some(ext) = extension {
            *inventory.extensions.entry(ext.clone()).or_insert(0) += 1;
            *inventory.extension_lines.entry(ext).or_insert(0) += lines;
        }
    }

    Ok(())
}
