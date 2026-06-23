/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let letters = sentence.to_ascii_lowercase();
    "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .all(|c| letters.contains(c))
}
