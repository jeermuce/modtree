use std::fs;
use std::path::Path;

pub trait HasRsFiles {
    fn has_rs_files(&self) -> bool;
}

impl HasRsFiles for Path {
    fn has_rs_files(&self) -> bool {
        if self.is_dir() {
            if let Ok(entries) = fs::read_dir(self) {
                return entries.filter_map(Result::ok).any(|entry| {
                    let entry_path = entry.path();
                    entry_path.is_dir() && entry_path.has_rs_files()
                        || entry_path.extension().and_then(|ext| ext.to_str()) == Some("rs")
                });
            }
            false
        } else {
            self.extension().and_then(|ext| ext.to_str()) == Some("rs")
        }
    }
}
