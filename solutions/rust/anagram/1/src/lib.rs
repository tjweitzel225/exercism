use std::collections::{HashMap, HashSet};

fn count_chars(word: &str) -> HashMap<char, u32> {
    let mut letters: HashMap<char, u32> = HashMap::new();
    for c in word.chars() {
        *letters.entry(c).or_insert(0) += 1;
    }
    letters
}
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let counts = count_chars(&word_lower);

    let is_anagram = |w: &&str| {
        let w_lower = w.to_lowercase();
        w_lower != word_lower && counts == count_chars(&w_lower)
    };
    possible_anagrams
        .iter()
        .copied()
        .filter(is_anagram)
        .collect()
}
