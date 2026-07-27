use std::collections::HashMap;
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .split(|c: char| c != '\'' && !c.is_alphanumeric())
        .map(|w| w.trim_matches('\'').to_lowercase())
        .filter(|w| !w.is_empty())
        .fold(HashMap::new(), |mut acc, w| {
            acc.entry(w).and_modify(|count| *count += 1).or_insert(1);
            acc
        })
}
