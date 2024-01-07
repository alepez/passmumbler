use crate::Secrets;
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
