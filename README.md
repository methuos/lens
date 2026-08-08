# Lens

**Lens** is a fast, offline-first CLI tool for inspecting and understanding software projects.

It scans a project directory and provides a compact overview of its files, directories, languages, dependencies, Git state, largest files, and searchable source content.

## Features

* Project summary
* File and directory analysis
* Human-readable file sizes
* Lines-of-code statistics
* File extension statistics
* Programming language statistics
* Largest files
* Directory statistics
* Project tree
* Source-code search
* Rust project analysis
* Git repository information
* Sorting by path, size, or line count
* JSON output for supported commands
* Offline operation
* No configuration required

## Installation

Build Lens from source:

```bash
git clone https://github.com/methuos/lens
cd lens
cargo build --release
```

The release binary will be available at:

```bash
target/release/lens
```

You can optionally install it locally:

```bash
cargo install --path .
```

Then verify:

```bash
lens --help
```

## Usage

Run Lens against the current directory:

```bash
lens
```

Or specify a project path:

```bash
lens /path/to/project
```

The default command displays a project summary.

## Commands

### Summary

```bash
lens
```

or:

```bash
lens summary
```

Displays:

* Project name
* File count
* Directory count
* Total size
* Total lines
* File extensions
* Programming languages
* Rust project information
* Largest files
* Git information

### Files

List project files:

```bash
lens files
```

Sort by size:

```bash
lens files --sort size
```

Sort by lines:

```bash
lens files --sort lines
```

Reverse the result:

```bash
lens files --sort size --reverse
```

Available sorting modes:

```text
path
size
lines
```

JSON output:

```bash
lens files --format json
```

### Directories

List analyzed directories:

```bash
lens dirs
```

Sort by size:

```bash
lens dirs --sort size
```

Sort by lines:

```bash
lens dirs --sort lines
```

Reverse the result:

```bash
lens dirs --sort size --reverse
```

JSON output:

```bash
lens dirs --format json
```

### Largest Files

Show the largest files in the project:

```bash
lens largest
```

### Tree

Display the project file tree:

```bash
lens tree
```

### Search

Search source files for a query:

```bash
lens search Cargo
```

Search is case-insensitive.

JSON output:

```bash
lens search Cargo --format json
```

### Languages

Display detected programming languages:

```bash
lens languages
```

### Rust

Display Rust project information:

```bash
lens rust
```

For Rust projects, Lens can report:

* Package name
* Version
* Edition
* Dependencies

### Git

Display Git repository information:

```bash
lens git
```

The Git section reports information such as:

* Current branch
* Working tree status
* Commit count
* Contributor count
* Remote information when available

## JSON Output

Commands that support JSON output can be used with:

```bash
--format json
```

For example:

```bash
lens files --format json
```

This makes Lens useful as a data source for scripts and other developer tools.

Example:

```bash
lens files --format json | jq
```

## Example

Running:

```bash
lens
```

can produce output similar to:

```text
Name          Lens
Files         25
Directories   5
Size          36.29 KB
Lines         1553

## Extensions

lock         1
rs           21
toml         1

## Languages

Rust           21 files     1079 lines
TOML            1 file        13 lines

# Rust Project

Package      lens
Version      0.1.0
Edition      2024

## Dependencies

anyhow
clap
ignore
serde
serde_json
toml
walkdir
```

## Development

Run formatting checks:

```bash
cargo fmt --check
```

Run the compiler checks:

```bash
cargo check
```

Run tests:

```bash
cargo test
```

Build an optimized release:

```bash
cargo build --release
```

## Project Structure

```text
Lens/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── analyzers/
│   ├── output/
│   ├── scanner/
│   ├── cli.rs
│   ├── inventory.rs
│   ├── main.rs
│   ├── search.rs
│   └── utils.rs
└── target/
```

### Architecture

Lens is divided into a few focused areas:

* `scanner` — discovers project files and collects basic statistics.
* `analyzers` — analyzes languages, Git, Rust projects, and other project properties.
* `output` — renders terminal and JSON results.
* `search` — searches project file contents.
* `inventory` — stores collected project information.
* `cli` — defines the command-line interface.

## Testing

Lens currently includes unit tests covering file sorting, directory sorting, line counting, and human-readable size formatting.

Run:

```bash
cargo test
```

Expected result:

```text
11 passed; 0 failed
```

## Design Goals

Lens is designed around a few principles:

* **Fast** — scan projects quickly.
* **Offline-first** — no network connection is required.
* **Simple** — useful information without unnecessary configuration.
* **Scriptable** — structured JSON output where supported.
* **Developer-focused** — provide information that helps understand an unfamiliar project quickly.

## Roadmap

Planned improvements for future versions may include:

* Tree depth control
* Improved search output
* More structured search results
* Additional project analyzers
* More JSON output modes
* Better filtering and exclusion controls
* Additional language detection
* Performance improvements for very large projects

## License

Lens is open-source software licensed under the Apache License, Version 2.0.
