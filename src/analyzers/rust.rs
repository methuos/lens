use std::collections::BTreeMap;
use std::fs;

use anyhow::Result;
use serde::Deserialize;

use crate::inventory::{Inventory, RustProject};

#[derive(Deserialize)]
struct CargoToml {
    package: Option<Package>,
    dependencies: Option<BTreeMap<String, toml::Value>>,
}

#[derive(Deserialize)]
struct Package {
    name: String,
    version: String,
    edition: String,
}

pub fn analyze(inventory: &mut Inventory) -> Result<()> {
    let cargo = inventory.root.join("Cargo.toml");

    if !cargo.exists() {
        return Ok(());
    }

    let text = fs::read_to_string(cargo)?;
    let manifest: CargoToml = toml::from_str(&text)?;

    let Some(package) = manifest.package else {
        return Ok(());
    };

    let dependencies = manifest
        .dependencies
        .unwrap_or_default()
        .keys()
        .cloned()
        .collect();

    inventory.rust = Some(RustProject {
        package: package.name,
        version: package.version,
        edition: package.edition,
        dependencies,
    });

    Ok(())
}
