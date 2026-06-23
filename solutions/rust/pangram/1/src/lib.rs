/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let letters = sentence.to_ascii_lowercase();
    alphabet
        .chars()
        .all(|c| letters.chars().find(|&l| c == l).is_some())
}
