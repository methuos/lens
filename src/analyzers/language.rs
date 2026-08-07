use crate::inventory::{Inventory, LanguageStat};

pub fn detect_languages(inventory: &mut Inventory) {
    inventory.languages.clear();

    for (ext, files) in &inventory.extensions {
        let name = match ext.as_str() {
            "rs" => "Rust",
            "toml" => "TOML",
            "md" => "Markdown",
            "json" => "JSON",
            "yaml" | "yml" => "YAML",
            "py" => "Python",
            "js" => "JavaScript",
            "ts" => "TypeScript",
            "go" => "Go",
            "java" => "Java",
            "cpp" | "cc" | "cxx" => "C++",
            "c" => "C",
            "cs" => "C#",
            "zig" => "Zig",
            _ => continue,
        };

        let lines = inventory.extension_lines.get(ext).copied().unwrap_or(0);

        inventory.languages.push(LanguageStat {
            name: name.to_string(),
            files: *files,
            lines,
        });
    }

    inventory.languages.sort_by_key(|x| std::cmp::Reverse(x.lines));
}
