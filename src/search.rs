use anyhow::Result;
use std::fs;

use crate::cli::OutputFormat;
use crate::inventory::{Inventory, SearchMatch};

pub fn run(inventory: &Inventory, query: &str, format: OutputFormat) -> Result<()> {
    let query = query.to_lowercase();

    let mut results = Vec::<SearchMatch>::new();
    let mut matches = 0;

    for file in &inventory.files_data {
        let full_path = inventory.root.join(&file.path);

        let mut printed_header = false;

        if let Ok(content) = fs::read_to_string(&full_path) {
            for (index, line) in content.lines().enumerate() {
                if line.to_lowercase().contains(&query) {
                    results.push(SearchMatch {
                        path: file.path.clone(),
                        line: index + 1,
                        text: line.trim().to_string(),
                    });

                    if matches!(format, OutputFormat::Terminal) {
                        if !printed_header {
                            println!();
                            println!("{}", file.path);
                            printed_header = true;
                        }

                        println!("  {:>4} | {}", index + 1, line.trim());
                    }

                    matches += 1;
                }
            }
        }

        if !printed_header
            && file.path.to_lowercase().contains(&query)
            && matches!(format, OutputFormat::Terminal)
        {
            println!();
            println!("{}", file.path);
            matches += 1;
        }
    }

        match format {
        OutputFormat::Terminal => {
            println!();
            println!("{} match(es)", matches);
        }

        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&results)?);
        }
    }

    Ok(())
}
