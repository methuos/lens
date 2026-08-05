use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Default)]
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
}

#[derive(Debug, Default)]
pub struct LanguageStat {
    pub name: String,
    pub files: usize,
    pub lines: usize,
}

#[derive(Debug, Default)]
pub struct GitInfo {
    pub branch: String,
    pub clean: bool,
}
