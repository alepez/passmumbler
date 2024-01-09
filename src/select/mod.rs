pub mod dmenu;
pub mod rofi;

/// Keep only entries that start with the prefix
/// Remove prefix from all entries
fn filter_and_remove_prefix<'a>(prefix: &str, entries: &'a Vec<String>) -> Vec<&'a str> {
    entries
        .iter()
        .filter(|s| s.starts_with(prefix))
        .map(|s| s.strip_prefix(prefix))
        .flatten()
        .collect()
}

pub trait SelectTool {
    fn select(&self, prefix: &str, entries: &Vec<String>) -> Option<String>;
}