mod counter;
mod ignore;
mod stats;
mod walker;

use anyhow::Result;
use std::path::{Path, PathBuf};

use crate::inventory::Inventory;

pub fn scan(path: &Path) -> Result<Inventory> {
    let root = resolve_root(path)?;
    let mut inventory = create_inventory(root);

    walker::collect_files(&mut inventory)?;

    Ok(inventory)
}

fn resolve_root(path: &Path) -> Result<PathBuf> {
    Ok(path.canonicalize()?)
}

fn create_inventory(root: PathBuf) -> Inventory {
    let project_name = root
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();

    Inventory {
        root,
        project_name,
        ..Default::default()
    }
}
