use super::{filter_and_remove_prefix, SelectTool};
use dmenu_facade::DMenu;

pub struct DmenuSelectTool;

impl SelectTool for DmenuSelectTool {
    fn select(&self, prefix: &str, entries: &Vec<String>) -> Option<String> {
        let msg = "Secrets";
        let entries = filter_and_remove_prefix(prefix, entries);

        let s = DMenu::default()
            .with_prompt(msg)
            .execute_consume(entries)
            .ok()?;

        // Add prefix back
        Some(format!("{prefix}{s}"))
    }
}
