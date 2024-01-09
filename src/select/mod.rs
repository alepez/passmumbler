use crate::pass::{list_entries, load_secrets};
use crate::Secrets;

pub mod dmenu;
pub mod rofi;

/// Keep only entries that start with the prefix
/// Remove prefix from all entries
fn filter_and_remove_prefix(prefix: &str, entries: Vec<String>) -> Vec<String> {
    entries
        .iter()
        .filter(|s| s.starts_with(prefix))
        .map(|s| s.strip_prefix(prefix))
        .flatten()
        .map(|s| s.to_string())
        .collect()
}

/// A tool to select a secret from a list of secrets
pub trait SelectTool {
    /// Select a secret from a list of secrets, using the prefix to filter the list
    fn select(&self, prefix: &str, entries: Vec<String>) -> Option<String>;

    fn select_and_load_secrets(&self, prefix: &str) -> Option<(String, Secrets)> {
        // Get all entries from password store
        let entries = list_entries();

        // Select one with the select tool
        let selected = self.select(&prefix, entries)?;

        // Load the secrets from password store
        let secrets = load_secrets(&selected)?;

        // Remove prefix from selected name
        let selected = selected.strip_prefix(&prefix)?.to_string();

        Some((selected, secrets))
    }
}
