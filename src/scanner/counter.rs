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
