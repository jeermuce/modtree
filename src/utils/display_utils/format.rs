use colored::*;
use std::collections::HashSet;

pub fn format_file(
    file_name: String,
    updated_files: &HashSet<String>,
    created_files: &HashSet<String>,
) -> String {
    if updated_files.contains(&file_name) {
        let file_str: String = format!(" {} ", file_name);
        format!(
            "{}{}",
            file_str.bold().white().on_truecolor(180, 77, 0),
            " [Updated] ".bold().truecolor(232, 97, 0)
        )
    } else if created_files.contains(&file_name) {
        let file_str: String = format!(" {} ", file_name);
        format!(
            "{}{}",
            file_str.bold().on_truecolor(0, 150, 0),
            " [Created] ".bold().green()
        )
    } else {
        (file_name.yellow()).to_string()
    }
}
