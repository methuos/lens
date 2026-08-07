use crate::inventory::Inventory;
use crate::utils::format_size;

pub fn print(inventory: &Inventory) {
    print_summary(inventory);
    print_extensions(inventory);
    print_languages(inventory);
    print_rust(inventory);
    print_largest(inventory);
    print_git(inventory);
}

pub fn print_summary(inventory: &Inventory) {
    println!("Project Summary");
    println!("===============");
    println!("Name          {}", inventory.project_name);
    println!("Files         {}", inventory.files);
    println!("Directories   {}", inventory.directories);
    println!("Size          {}", format_size(inventory.total_size));
    println!("Lines         {}", inventory.total_lines);
}

pub fn print_extensions(inventory: &Inventory) {
    if inventory.extensions.is_empty() {
        return;
    }

    println!();
    println!("Extensions");
    println!("----------");

    let mut extensions: Vec<_> = inventory.extensions.iter().collect();
    extensions.sort_by(|a, b| a.0.cmp(b.0));

    for (ext, count) in extensions {
        println!("{:<12} {}", ext, count);
    }
}

pub fn print_languages(inventory: &Inventory) {
    if inventory.languages.is_empty() {
        return;
    }

    println!();
    println!("Languages");
    println!("---------");

    for language in &inventory.languages {
        let file_label = if language.files == 1 { "file" } else { "files" };
        let line_label = if language.lines == 1 { "line" } else { "lines" };

        println!(
            "{:<12} {:>4} {:<5} {:>8} {}",
            language.name, language.files, file_label, language.lines, line_label
        );
    }
}

pub fn print_rust(inventory: &Inventory) {
    let Some(rust) = &inventory.rust else {
        return;
    };

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

pub fn print_largest(inventory: &Inventory) {
    if inventory.largest_files.is_empty() {
        return;
    }

    println!();
    println!("Largest Files");
    println!("-------------");

    for file in &inventory.largest_files {
        println!(
            "{:<40} {:>8} {:>6} lines",
            file.path,
            format_size(file.size),
            file.lines
        );
    }
}

pub fn print_git(inventory: &Inventory) {
    let Some(git) = &inventory.git else {
        return;
    };

    println!();
    println!("Git");
    println!("===");

    println!("Branch        {}", git.branch);
    println!(
        "Status        {}",
        if git.clean { "Clean" } else { "Modified" }
    );
    println!("Commits       {}", git.commits);
    println!("Contributors  {}", git.contributors);

    if let Some(remote) = &git.remote {
        println!();
        println!("Remote");
        println!("------");
        println!("{}", remote);
    }
}
