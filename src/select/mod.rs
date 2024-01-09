use crate::pass::{list_entries, load_secrets};
use crate::Secrets;

pub mod dmenu;
pub mod fzf;
pub mod rofi;

/// Keep only entries that start with the prefix
/// Remove prefix from all entries
fn filter_and_remove_prefix(prefix: &str, entries: Vec<String>) -> Vec<String> {
    entries
        .iter()
        .filter(|s| s.starts_with(prefix))
        .filter_map(|s| s.strip_prefix(prefix))
        .map(|s| s.to_string())
        .collect()
}

/// A tool to select a secret from a list of secrets
pub trait SelectTool {
    /// Select a secret from a list of secrets, using the prefix to filter the list
    fn select(&self, entries: Vec<String>) -> Option<String>;

    fn select_and_load_secrets(&self, prefix: &str) -> Result<(String, Secrets), Error> {
        // Get all entries from password store
        let entries_with_prefix = list_entries();

        // Filter and remove prefix from entries
        let entries = filter_and_remove_prefix(prefix, entries_with_prefix);

        // Select one with the select tool
        let Some(selected) = self.select(entries) else {
            return Err(Error::NothingSelected);
        };

        // Add prefix back, needed to find a match in password store
        let selected_with_prefix = format!("{prefix}{selected}");

        // Load the secrets from password store
        let Some(secrets) = load_secrets(&selected_with_prefix) else {
            return Err(Error::NoSecretsFound);
        };

        // Return the selected entry (without prefix) and the secrets
        Ok((selected, secrets))
    }
}

pub enum Error {
    NoSecretsFound,
    NothingSelected,
}
