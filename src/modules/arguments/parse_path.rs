use std::path::PathBuf;

use anyhow::{Result, anyhow};
pub fn parse_path(s: &str) -> Result<PathBuf> {
    let path = PathBuf::from(s);
    path.canonicalize().map_err(|e| anyhow!(e))
}
