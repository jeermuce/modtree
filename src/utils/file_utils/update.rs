use anyhow::Result;
use std::fs::{self};
use std::io::{self};
use std::path::PathBuf;
pub fn update(mod_file_path: PathBuf, new_line: String) -> Result<(), io::Error> {
    fs::write(&mod_file_path, new_line)?;
    Ok(())
}
