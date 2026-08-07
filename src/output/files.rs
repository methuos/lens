use crate::cli::{OutputFormat, SortBy};
use crate::inventory::{FileInfo, Inventory};
use crate::utils::format_size;

pub(crate) fn sort_files(mut files: Vec<FileInfo>, sort: SortBy, reverse: bool) -> Vec<FileInfo> {
    match sort {
        SortBy::Path => files.sort_by(|a, b| a.path.cmp(&b.path)),
        SortBy::Size => files.sort_by_key(|x| std::cmp::Reverse(x.size)),
        SortBy::Lines => files.sort_by_key(|x| std::cmp::Reverse(x.lines)),
    }

    if reverse {
        files.reverse();
    }

    files
}

pub fn print(inventory: &Inventory, sort: SortBy, reverse: bool, format: OutputFormat) {
    let files = sort_files(inventory.files_data.clone(), sort, reverse);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inventory::FileInfo;

    #[test]
    fn sorts_by_size_descending() {
        let files = vec![
            FileInfo {
                path: "a".into(),
                size: 10,
                lines: 1,
            },
            FileInfo {
                path: "b".into(),
                size: 50,
                lines: 2,
            },
            FileInfo {
                path: "c".into(),
                size: 30,
                lines: 3,
            },
        ];

        let files = sort_files(files, SortBy::Size, false);
        assert_eq!(files[0].path, "b");
        assert_eq!(files[1].path, "c");
        assert_eq!(files[2].path, "a");
    }

    #[test]
    fn sorts_by_size_reverse() {
        let files = vec![
            FileInfo {
                path: "a".into(),
                size: 10,
                lines: 1,
            },
            FileInfo {
                path: "b".into(),
                size: 50,
                lines: 2,
            },
            FileInfo {
                path: "c".into(),
                size: 30,
                lines: 3,
            },
        ];

        let files = sort_files(files, SortBy::Size, true);

        assert_eq!(files[0].path, "a");
        assert_eq!(files[1].path, "c");
        assert_eq!(files[2].path, "b");
    }
}
