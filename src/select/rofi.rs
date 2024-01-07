pub fn select(prefix: &str, entries: &Vec<String>) -> Option<String> {
    let msg = "Secrets";

    // Keep only entries that start with the prefix
    // Remove prefix from all entries
    let entries: Vec<&str> = entries
        .iter()
        .filter(|s| s.starts_with(prefix))
        .map(|s| s.strip_prefix(prefix))
        .flatten()
        .collect();

    let index = rofi::Rofi::new(&*entries).prompt(msg).run_index().ok()?;

    // Add prefix back
    entries.get(index).map(|s| format!("{}{}", prefix, s))
}
