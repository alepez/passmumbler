use super::{SelectTool};
use dmenu_facade::DMenu;

pub struct DmenuSelectTool;

impl SelectTool for DmenuSelectTool {
    fn select(&self, entries: Vec<String>) -> Option<String> {
        let msg = "Secrets";

        DMenu::default()
            .with_prompt(msg)
            .execute_consume(entries)
            .ok()
    }
}
