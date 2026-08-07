use crate::cli::OutputFormat;
use crate::cli::SortBy;
use crate::inventory::Inventory;
use crate::utils::format_size;
use serde_json;

pub(crate) fn sort_dirs(
    mut dirs: Vec<crate::inventory::DirectoryInfo>,
    sort: SortBy,
    reverse: bool,
) -> Vec<crate::inventory::DirectoryInfo> {
    match sort {
        SortBy::Path => dirs.sort_by(|a, b| a.path.cmp(&b.path)),
        SortBy::Size => dirs.sort_by_key(|x| std::cmp::Reverse(x.size)),
        SortBy::Lines => dirs.sort_by_key(|x| std::cmp::Reverse(x.lines)),
    }

    if reverse {
        dirs.reverse();
    }

    dirs
}

pub fn print(inventory: &Inventory, sort: SortBy, reverse: bool, format: OutputFormat) {
    let dirs = sort_dirs(
    inventory.directories_data.clone(),
    sort,
    reverse,
);

    match format {
        OutputFormat::Terminal => {}

        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&dirs).unwrap());
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inventory::DirectoryInfo;

    #[test]
    fn sorts_by_size_descending() {
        let dirs = vec![
            DirectoryInfo {
                path: "a".into(),
                files: 1,
                lines: 10,
                size: 10,
            },
            DirectoryInfo {
                path: "b".into(),
                files: 1,
                lines: 20,
                size: 50,
            },
            DirectoryInfo {
                path: "c".into(),
                files: 1,
                lines: 15,
                size: 30,
            },
        ];

        let dirs = sort_dirs(dirs, SortBy::Size, false);

        assert_eq!(dirs[0].path, "b");
        assert_eq!(dirs[1].path, "c");
        assert_eq!(dirs[2].path, "a");
    }
}