use crate::cli::OutputFormat;
use crate::cli::SortBy;
use crate::inventory::Inventory;
use crate::utils::format_size;

pub fn print(inventory: &Inventory, sort: SortBy, reverse: bool, format: OutputFormat) {
    let mut files = inventory.files_data.clone();

    match sort {
        SortBy::Path => files.sort_by(|a, b| a.path.cmp(&b.path)),
        SortBy::Size => files.sort_by(|a, b| b.size.cmp(&a.size)),
        SortBy::Lines => files.sort_by(|a, b| b.lines.cmp(&a.lines)),
    }

    if reverse {
        files.reverse();
    }

    match format {
        OutputFormat::Terminal => {}

        OutputFormat::Json => {
            crate::output::json::print(&files);
            return;
        }
    }
    println!("Files ({})", files.len());
    println!();

    println!("{:<60} {:>10} {:>8}", "Path", "Size", "Lines");
    println!("{}", "-".repeat(82));

    for file in files {
        println!(
            "{:<60} {:>10} {:>8}",
            file.path,
            format_size(file.size),
            file.lines,
        );
    }
}
