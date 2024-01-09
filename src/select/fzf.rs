use std::{
    io::Write,
    process::{Command, Stdio},
};

use crate::select::SelectTool;

pub struct FzfSelectTool;

impl SelectTool for FzfSelectTool {
    fn select(&self, entries: Vec<String>) -> Option<String> {
        let mut fzf = Command::new("fzf")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .ok()?;

        let mut stdin = fzf.stdin.take().unwrap();

        entries.iter().for_each(|s| {
            writeln!(stdin, "{}", s).unwrap();
        });

        let output = fzf.wait_with_output().ok()?;

        if !output.status.success() || output.stdout.is_empty() {
            return None;
        }

        // The output from fzf is the selected entry
        String::from_utf8(output.stdout).ok()
    }
}
