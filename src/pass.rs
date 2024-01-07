use crate::Secrets;
use glob::glob;
use std::io::BufReader;
use std::process::Stdio;

pub fn load_secrets(id: &str) -> Option<Secrets> {
    let cmd = format!("pass show {}", id);
    let mut child = std::process::Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::piped())
        .spawn()
        .ok()?;
    let stdout = child.stdout.take()?;
    let reader = BufReader::new(stdout);
    let secrets = Secrets::from(reader);
    Some(secrets)
}

pub fn list_entries() -> Vec<String> {
    let home_dir = dirs::home_dir().unwrap();
    let pass_dir = home_dir.join(".password-store");
    let pass_dir = pass_dir.to_str().unwrap();
    let prefix = format!("{}/", pass_dir);
    let suffix = ".gpg";
    let glob_pattern = format!("{prefix}**/*{suffix}");

    let prefix_len = prefix.len();
    let suffix_len = suffix.len();

    let mut entries: Vec<String> = glob(glob_pattern.as_str())
        .unwrap()
        .filter_map(|entry| entry.ok())
        .map(|path| {
            let s = path.to_str().unwrap().to_string();
            // Remove prefix and suffix
            let s = &s[prefix_len..s.len() - suffix_len];
            s.to_string()
        })
        .collect();

    entries.sort();
    entries
}
