use crate::select::SelectTool;
use gtk4::Application;

pub struct Gtk4SelectTool;

impl Gtk4SelectTool {
    pub fn new(_application: &Application) -> Self {
        todo!()
    }
}

impl SelectTool for Gtk4SelectTool {
    fn select(&self, _prefix: &str, _entries: &Vec<String>) -> Option<String> {
        todo!()
    }
}
