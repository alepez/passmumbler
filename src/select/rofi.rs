use crate::select::filter_and_remove_prefix;

pub fn select(prefix: &str, entries: &Vec<String>) -> Option<String> {
    let msg = "Secrets";
    let entries = filter_and_remove_prefix(prefix, entries);

    let index = rofi::Rofi::new(&*entries).prompt(msg).run_index().ok()?;
    let s = entries.get(index)?;

    // Add prefix back
    Some(format!("{prefix}{s}"))
}
