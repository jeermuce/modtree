use anyhow::Result;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::PathBuf;
pub fn create_and_write_mod_file(file_path: PathBuf, content: String) -> Result<(), io::Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .open(file_path)?;
    writeln!(file, "{}", content)?;
    Ok(())
}
