mod modules;
mod utils;
use anyhow::Result;
use clap::Parser;
use clearscreen::clear;
use colored::Colorize;
use modules::arguments::args;
use modules::display::tree::get_tree;
use modules::processor::process_directory::process_directory;
use std::collections::HashSet;
use std::path::PathBuf;
use utils::display_utils::parse::parse_paths;
/// This program is a simple tree maintenance tool that creates and updates mod
/// files in a directory tree for a Rust project. It uses the newer style of mod
/// files, where mod files are named after their directory and exist in the parent directory.
///
/// This does not support:
/// - the older style of mod files, which are named `mod.rs` and exist in the directory they represent.
/// - modifying files that are not modules (e.g., `lib.rs`, `main.rs`).
/// - removing mod statements or files.
///
/// This partly supports:
/// - renaming files/directories (creates new mod entries but doesn't remove old ones).
/// - moving files/directories (creates new mod entries but doesn't remove old ones).
///
/// This fully supports:
/// - creating mod files for new Rust source files.
/// - handling nested directories (will ignore directories that contain no Rust source files).
/// - adding appropriate `mod` statements for new files in existing mod directories.
fn main() -> Result<()> {
    let args: args::Args = args::Args::parse();
    let source: PathBuf = args.source;
    let source_name: colored::ColoredString = source.to_string_lossy().red().bold();
    // This is a mutable reference to a string in order to be able to pass it to the recursive function
    // and collect the output of the process_directory function
    let output: &mut String = &mut String::new();

    // The output variable will be filled with the result of the process_directory function
    process_directory(&source, &source, output)?;

    // These exist to control the appearance of the printed tree
    // We use the output variable that has been filled by the process_directory function
    let updated_files: HashSet<String> = parse_paths(output, "U");
    let created_files: HashSet<String> = parse_paths(output, "C");

    if updated_files.is_empty() && created_files.is_empty() {
        return Ok(());
    }
    clear()?;
    let tree = get_tree(&source, String::new(), &updated_files, &created_files)?;

    println!("{}\n{}", source_name, tree);
    Ok(())
}
