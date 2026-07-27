use std::collections::HashMap;
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .to_ascii_lowercase()
        .split(|c: char| c != '\'' && !c.is_alphanumeric())
        .map(|w| w.trim_matches('\''))
        .filter(|w| !w.is_empty())
        .fold(HashMap::new(), |mut acc, w| {
            acc.entry(w.to_string())
                .and_modify(|count| *count += 1)
                .or_insert(1);
            acc
        })
}
