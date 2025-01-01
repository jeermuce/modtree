use std::fs;
use std::path::Path;

pub fn has_rs_files(path: &Path) -> bool {
    if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            return entries.filter_map(Result::ok).any(|entry| {
                let entry_path = entry.path();
                entry_path.is_dir()
                    || entry_path.extension().and_then(|ext| ext.to_str()) == Some("rs")
            });
        }
        false
    } else {
        path.extension().and_then(|ext| ext.to_str()) == Some("rs")
    }
}
