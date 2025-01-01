use anyhow::{Result, anyhow};
use colored::*;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

use crate::utils::display_utils::format::format_file;

pub fn get_tree(
    path: &Path,
    prefix: String,
    updated_files: &HashSet<String>,
    created_files: &HashSet<String>,
) -> Result<String> {
    let mut tree = String::new();
    collect_tree(path, prefix, &mut tree, updated_files, created_files)?;
    Ok(tree)
}

fn collect_tree(
    path: &Path,
    prefix: String,
    tree: &mut String,
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
                tree.push_str(&format!(
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

                collect_tree(&entry, new_prefix, tree, updated_files, created_files)?;
            } else if entry.extension().and_then(|ext| ext.to_str()) == Some("rs") {
                let file_name = entry
                    .file_name()
                    .ok_or_else(|| anyhow!("Missing directory name"))?
                    .to_string_lossy()
                    .to_string();
                let file_name = format_file(file_name, updated_files, created_files).bold();
                tree.push_str(&format!(
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
