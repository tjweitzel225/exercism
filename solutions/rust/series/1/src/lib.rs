pub fn series(digits: &str, len: usize) -> Vec<String> {
    digits
        .as_bytes()
        .windows(len)
        .map(|win| std::str::from_utf8(win).unwrap().to_string())
        .collect()
}
