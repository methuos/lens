use crate::inventory::Inventory;
use crate::utils::format_size;

pub fn print(inventory: &Inventory) {
    println!("Project Summary");
    println!("===============");
    println!("Name          {}", inventory.project_name);
    println!("Files         {}", inventory.files);
    println!("Directories   {}", inventory.directories);
    println!("Size          {}", format_size(inventory.total_size));
    println!("Lines         {}", inventory.total_lines);

    println!();
    println!("Extensions");
    println!("----------");

    let mut extensions: Vec<_> = inventory.extensions.iter().collect();
    extensions.sort_by(|a, b| a.0.cmp(b.0));

    for (ext, count) in extensions {
        println!("{:<12} {}", ext, count);
    }

    if !inventory.languages.is_empty() {
        println!();
        println!("Languages");
        println!("---------");

        for language in &inventory.languages {
            let file_label = if language.files == 1 { "file" } else { "files" };
            let line_label = if language.lines == 1 { "line" } else { "lines" };

            println!(
                "{:<12} {:>4} {:<5} {:>8} {}",
                language.name, language.files, file_label, language.lines, line_label,
            );
        }
    }

    if let Some(rust) = &inventory.rust {
        println!();
        println!("Rust Project");
        println!("============");
        println!("Package      {}", rust.package);
        println!("Version      {}", rust.version);
        println!("Edition      {}", rust.edition);

        if !rust.dependencies.is_empty() {
            println!();
            println!("Dependencies");
            println!("------------");

            for dep in &rust.dependencies {
                println!("{}", dep);
            }
        }
    }

    if let Some(git) = &inventory.git {
        println!();
        println!("Git");
        println!("---");
        println!("Branch      {}", git.branch);
        println!("Clean       {}", git.clean);
    }
}
