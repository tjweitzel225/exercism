fn split_camelcase(s: &str) -> Vec<&str> {
    let mut start = 0;
    let mut parts = Vec::new();
    for (i, w) in s.as_bytes().windows(2).enumerate() {
        if w[0].is_ascii_lowercase() && w[1].is_ascii_uppercase() {
            parts.push(&s[start..i + 1]);
            start = i + 1;
        }
    }
    parts.push(&s[start..]);
    parts
}
pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split([' ', '-'])
        .flat_map(split_camelcase)
        .filter_map(|w| match w {
            "" => None,
            _ => w.chars().find(|c| c.is_alphabetic()),
        })
        .collect::<String>()
        .to_ascii_uppercase()
}
