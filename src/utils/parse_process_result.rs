use std::collections::HashSet;

pub fn parse_process_result(process_result: &str, action: &str) -> HashSet<String> {
    // Using a HashSet to avoid duplicates
    process_result
        .lines()
        .filter_map(|line| {
            if line.starts_with(action) {
                line.split_once(": ")
                    .map(|(_, path)| path.trim().to_string())
            } else {
                None
            }
        })
        .collect()
}
