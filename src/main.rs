mod arguments;
mod utils;

use anyhow::Result;
use arguments::args;
use clap::Parser;
use colored::Colorize;
use std::collections::HashSet;
use std::path::PathBuf;
use utils::{mod_utils, parse_process_result, tree};
/// This program is a simple tree maintenance tool that creates and updates mod
/// files in a directory tree for a Rust project.
/// It uses the newer style of mod files, which are named after the directory
/// they represent but exist in the parent directory of it, and contain
/// appropriate `mod` statements for each .rs file in the directory they represent.
///
/// This does not support:
/// - the older style of mod files, which are named `mod.rs` and exist in the
///     directory they represent.
/// - adding `mod` statements to the root's `lib.rs` or `main.rs` file.
/// - removing mod statements from any file.
/// - removing any file.
///
/// This partly supports:
/// - renaming a file/directory
///     - if a file/directory is renamed, a mod file will be created if needed, and a `mod`
///        statment will be added where needed, but no files or `mod` statements
///        will be removed.
/// - moving a file/directory
///    - if a file/directory is moved, a mod file will be created if needed, and a `mod`
///      statement will be added where needed, but no files or `mod` statements
///      will be removed.
///
/// This fully supports:
/// - new file
///    - if a new file is added, a mod file will be created if needed, and a `mod`
///       statement will be added where needed.
/// - nested directories
///   - if a new directory is added, a mod file will be created if needed (the
///         directory has .rs files in it), and a `mod` statement will be added where needed.
///
fn main() -> Result<()> {
    let args: args::Args = args::Args::parse();
    let source: PathBuf = args.source;
    let source_name: colored::ColoredString = source.to_string_lossy().red().bold();
    // This is a mutable reference to a string in order to be able to pass it to the recursive function
    // and collect the output of the process_directory function
    let output: &mut String = &mut String::new();

    // The output variable will be filled with the result of the process_directory function
    mod_utils::process_directory(&source, &source, output)?;

    // These exist to control the appearance of the printed tree
    // We use the output variable that has been filled by the process_directory function
    let updated_files: HashSet<String> = parse_process_result::parse_process_result(output, "U");
    let created_files: HashSet<String> = parse_process_result::parse_process_result(output, "C");

    let tree = tree::get_tree(&source, String::new(), &updated_files, &created_files)?;

    println!("{}\n{}", source_name, tree);
    Ok(())
}
