use std::{
    io::{self, Write},
    process::{Child, ChildStdin, Command, Stdio},
};

use crate::select::{filter_and_remove_prefix, SelectTool};

pub struct FzfSelectTool;

impl SelectTool for FzfSelectTool {
    fn select(&self, prefix: &str, entries: Vec<String>) -> Option<String> {
        let entries = filter_and_remove_prefix(prefix, entries);

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
        let s = String::from_utf8(output.stdout).ok()?;

        // Add prefix back
        Some(format!("{prefix}{s}"))
    }
}
