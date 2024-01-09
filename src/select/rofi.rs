use crate::select::{filter_and_remove_prefix, SelectTool};

pub struct RofiSelectTool;

impl SelectTool for RofiSelectTool {
    fn select(&self, entries: Vec<String>) -> Option<String> {
        let msg = "Secrets";

        let index = rofi::Rofi::new(&*entries).prompt(msg).run_index().ok()?;

        Some(entries.get(index)?.clone())
    }
}
