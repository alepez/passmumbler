use dmenu_facade::DMenu;

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

    let s = DMenu::default()
        .with_prompt(msg)
        .execute_consume(entries)
        .ok()?;

    // Add prefix back
    Some(format!("{prefix}{s}"))
}
