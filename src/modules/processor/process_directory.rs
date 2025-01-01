use crate::utils::file_utils::create::create_and_write_mod_file;
use crate::utils::file_utils::find::HasRsFiles;
use crate::utils::file_utils::update::update;
use anyhow::Result;
use std::fs::{self};
use std::io::{self};
use std::path::Path;
pub fn ensure_mod_lines(path: &Path, source: &Path) -> Result<String, io::Error> {
    if !path.has_rs_files() {
        return Ok(String::from("No .rs files found"));
    }

    let mut creation_result = String::new();
    let mod_file_path = path
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Path has no parent directory"))?
        .join(format!(
            "{}.rs",
            path.file_name().unwrap().to_string_lossy()
        ));

    let mod_lines: Vec<_> = fs::read_dir(path)?
        .filter_map(|entry| entry.ok())
        .filter_map(|e| {
            let path = e.path();
            if path.extension().is_some_and(|ext| ext == "rs")
                || path.is_dir() && path.has_rs_files()
            {
                path.file_stem()
                    .map(|stem| format!("pub mod {};", stem.to_string_lossy()))
            } else {
                None
            }
        })
        .collect();

    if mod_file_path.exists() {
        // Edit the existing mod file
        let content = fs::read_to_string(&mod_file_path)?;
        let mut new_content = content.clone();
        for mod_line in &mod_lines {
            if !content.contains(mod_line) {
                new_content.push_str(&format!("{}\n", mod_line));
            }
        }

        if new_content != content {
            creation_result = format!(
                "U: {}",
                mod_file_path
                    .file_name()
                    .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "File has no name"))?
                    .to_string_lossy()
            );

            update(mod_file_path, new_content)?;
        }
    } else if mod_file_path != source.with_extension("rs") {
        // Create a new mod file, but not <source>.rs
        creation_result = format!(
            "C: {}",
            mod_file_path
                .file_name()
                .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "File has no name"))?
                .to_string_lossy()
        );
        create_and_write_mod_file(mod_file_path, mod_lines.join("\n"))?;
    }

    Ok(creation_result)
}

pub fn process_directory(
    path: &Path,
    source: &Path,
    output: &mut String,
) -> Result<String, io::Error> {
    if !path.is_dir() {
        return Ok(String::from("Path is not a directory"));
    }

    let ensurer_result = ensure_mod_lines(path, source)?;

    if !ensurer_result.is_empty() {
        output.push_str(&format!("{}\n", ensurer_result));
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?.path();
        if entry.is_dir() {
            process_directory(&entry, source, output)?;
        }
    }

    Ok(output.to_string())
}
