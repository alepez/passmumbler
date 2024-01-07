pub fn select(entries: &[&str]) -> Option<String> {
    let msg = "Secrets";
    let index = rofi::Rofi::new(entries).prompt(msg).run_index().ok()?;
    entries.get(index).map(|s| s.to_string())
}
