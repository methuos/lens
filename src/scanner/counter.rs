use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn count_lines(path: &Path) -> Result<usize> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut lines = 0;

    for line in reader.lines() {
        line?;
        lines += 1;
    }

    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::count_lines;
    use std::fs;
    use std::path::PathBuf;

    fn temp_file(name: &str) -> PathBuf {
        std::env::temp_dir().join(name)
    }

    #[test]
    fn counts_three_lines() {
        let path = temp_file("lens_counter_three_lines.txt");

        fs::write(&path, "one\ntwo\nthree\n").unwrap();

        let count = count_lines(&path).unwrap();

        assert_eq!(count, 3);

        let _ = fs::remove_file(path);
    }

    #[test]
    fn counts_empty_file() {
        let path = temp_file("lens_counter_empty.txt");

        fs::write(&path, "").unwrap();

        let count = count_lines(&path).unwrap();

        assert_eq!(count, 0);

        let _ = fs::remove_file(path);
    }

    #[test]
    fn missing_file_returns_error() {
        let path = temp_file("lens_file_does_not_exist.txt");

        assert!(count_lines(&path).is_err());
    }
}
