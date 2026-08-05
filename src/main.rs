mod inventory;
mod scanner;

use anyhow::Result;
use std::path::Path;

fn main() -> Result<()> {
    let inventory = scanner::scan(Path::new("."))?;

    println!("Project      : {}", inventory.project_name);
    println!("Files        : {}", inventory.files);
    println!("Directories  : {}", inventory.directories);
    println!("Total Size   : {} bytes", inventory.total_size);
    println!("Total Lines  : {}", inventory.total_lines);

    println!("\nExtensions");

    let mut extensions: Vec<_> = inventory.extensions.iter().collect();
    extensions.sort_by(|a, b| a.0.cmp(b.0));

    for (ext, count) in extensions {
        println!("{:<10} {}", ext, count);
    }

    // Read language statistics (will be populated later)
    if !inventory.languages.is_empty() {
        println!("\nLanguages");

        for language in &inventory.languages {
            println!(
                "{:<12} files: {:<4} lines: {}",
                language.name,
                language.files,
                language.lines
            );
        }
    }

    // Read git information (will be populated later)
    if let Some(git) = &inventory.git {
        println!("\nGit");
        println!("Branch : {}", git.branch);
        println!("Clean  : {}", git.clean);
    }

    Ok(())
}