const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn split_point(word: &str) -> usize {
    if word.starts_with(VOWELS) || word.starts_with("xr") || word.starts_with("yt") {
        return 0;
    }
    for (i, c) in word.char_indices() {
        if word[i..].starts_with("qu") {
            return i + 2;
        }
        if VOWELS.contains(&c) || (c == 'y' && i > 0) {
            return i;
        }
    }
    word.len()
}
pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let (head, tail) = word.split_at(split_point(word));
            format!("{tail}{head}ay")
        })
        .collect::<Vec<_>>()
        .join(" ")
}
