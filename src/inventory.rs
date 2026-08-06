use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Default, Serialize)]
pub struct Inventory {
    pub root: PathBuf,
    pub project_name: String,

    pub files: usize,
    pub directories: usize,
    pub total_size: u64,

    pub total_lines: usize,

    pub extensions: HashMap<String, usize>,

    pub languages: Vec<LanguageStat>,

    pub git: Option<GitInfo>,

    pub extension_lines: HashMap<String, usize>,

    pub rust: Option<RustProject>,

    pub files_data: Vec<FileInfo>,

    pub largest_files: Vec<FileInfo>,

    pub directories_data: Vec<DirectoryInfo>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct LanguageStat {
    pub name: String,
    pub files: usize,
    pub lines: usize,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GitInfo {
    pub branch: String,
    pub clean: bool,

    pub commits: usize,
    pub contributors: usize,

    pub remote: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct RustProject {
    pub package: String,
    pub version: String,
    pub edition: String,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileInfo {
    pub path: String,
    pub size: u64,
    pub lines: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct DirectoryInfo {
    pub path: String,
    pub files: usize,
    pub lines: usize,
    pub size: u64,
}
