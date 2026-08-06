use crate::cli::SortBy;
use crate::inventory::Inventory;
use crate::utils::format_size;
use crate::cli::OutputFormat;
use serde_json;


pub fn print(
    inventory: &Inventory,
    sort: SortBy,
    reverse: bool,
    format: OutputFormat,) {
    let mut dirs = inventory.directories_data.clone();

    match sort {
        SortBy::Path => dirs.sort_by(|a, b| a.path.cmp(&b.path)),
        SortBy::Size => dirs.sort_by(|a, b| b.size.cmp(&a.size)),
        SortBy::Lines => dirs.sort_by(|a, b| b.lines.cmp(&a.lines)),
    }

    if reverse {
        dirs.reverse();
    }

    match format {
    OutputFormat::Terminal => {}

    OutputFormat::Json => {
        println!(
            "{}",
            serde_json::to_string_pretty(&dirs).unwrap()
        );
        return;
    }
}

    println!("Directories ({})", dirs.len());
    println!();

    println!(
        "{:<40} {:>8} {:>10} {:>14}",
        "Path", "Files", "Lines", "Size",
    );

    println!("{}", "-".repeat(74));

    for dir in dirs {
        println!(
            "{:<40} {:>8} {:>10} {:>10}",
            if dir.path.is_empty() { "." } else { &dir.path },
            dir.files,
            dir.lines,
            format_size(dir.size),
        );
    }
}
