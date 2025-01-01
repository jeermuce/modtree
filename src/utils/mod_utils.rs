use anyhow::Result;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

pub fn ensure_mod_lines(path: &Path, source: &Path) -> Result<String, io::Error> {
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
            if path.extension().is_some_and(|ext| ext == "rs") || path.is_dir() {
                path.file_stem()
                    .map(|stem| format!("pub mod {};", stem.to_string_lossy()))
            } else {
                None
            }
        })
        .collect();

    if mod_file_path.exists() {
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

            fs::write(&mod_file_path, new_content)?;
        }
    } else if mod_file_path != source.with_extension("rs") {
        creation_result = create_and_write_mod_file(&mod_file_path, &mod_lines.join("\n"))?;
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

    if fs::read_dir(path)?.any(|res| {
        res.ok()
            .is_some_and(|e| super::file_utils::has_rs_files(&e.path()))
    }) {
        let ensurer_result = ensure_mod_lines(path, source)?;

        if !ensurer_result.is_empty() {
            output.push_str(&format!("{}\n", ensurer_result));
        }

        for entry in fs::read_dir(path)? {
            let entry = entry?.path();
            if entry.is_dir() && super::file_utils::has_rs_files(&entry) {
                process_directory(&entry, source, output)?;
            }
        }
    }

    Ok(output.to_string())
}

fn create_and_write_mod_file(file_path: &Path, content: &str) -> Result<String, io::Error> {
    let result = format!(
        "C: {}",
        file_path
            .file_name()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "File has no name"))?
            .to_string_lossy()
    );
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .open(file_path)?;
    writeln!(file, "{}", content)?;
    Ok(result)
}
