use anyhow::{Result, anyhow};
use colored::*;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
fn format_file(
    file: &str,
    updated_files: &HashSet<String>,
    created_files: &HashSet<String>,
) -> String {
    if updated_files.contains(file) {
        let file_str: String = format!(" {} ", file);
        format!(
            "{}{}",
            file_str.bold().white().on_truecolor(180, 77, 0),
            " [Updated] ".bold().truecolor(232, 97, 0)
        )
    } else if created_files.contains(file) {
        let file_str: String = format!(" {} ", file);
        format!(
            "{}{}",
            file_str.bold().on_truecolor(0, 150, 0),
            " [Created] ".bold().green()
        )
    } else {
        (file.yellow()).to_string()
    }
}

pub fn get_tree(
    path: &Path,
    prefix: String,
    updated_files: &HashSet<String>,
    created_files: &HashSet<String>,
) -> Result<String> {
    let mut output = String::new();
    collect_tree(path, prefix, &mut output, updated_files, created_files)?;

    Ok(output)
}

fn collect_tree(
    path: &Path,
    prefix: String,
    output: &mut String,
    updated_files: &HashSet<String>,
    created_files: &HashSet<String>,
) -> Result<()> {
    if path.is_dir() {
        let entries: Vec<_> = fs::read_dir(path)?.collect();
        let len = entries.len();

        for (i, entry) in entries.into_iter().enumerate() {
            let entry = entry?.path();
            let is_last = i == len - 1;

            if entry.is_dir() {
                let dir_name = entry
                    .file_name()
                    .ok_or_else(|| anyhow!("Missing directory name"))?
                    .to_string_lossy()
                    .blue()
                    .bold();
                output.push_str(&format!(
                    "{}{} {}\n",
                    prefix,
                    if is_last { "└──" } else { "├──" },
                    dir_name
                ));

                let new_prefix = if is_last {
                    format!("{}    ", prefix)
                } else {
                    format!("{}│   ", prefix)
                };

                collect_tree(&entry, new_prefix, output, updated_files, created_files)?;
            } else if entry.extension().and_then(|ext| ext.to_str()) == Some("rs") {
                let file_name = entry
                    .file_name()
                    .ok_or_else(|| anyhow!("Missing directory name"))?
                    .to_string_lossy();
                let file_name = format_file(&file_name, updated_files, created_files)
                    .bold()
                    .to_string();
                output.push_str(&format!(
                    "{}{} {}\n",
                    prefix,
                    if is_last { "└──" } else { "├──" },
                    file_name
                ));
            }
        }
    }
    Ok(())
}
