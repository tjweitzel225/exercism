pub fn check(candidate: &str) -> bool {
    let mut v: Vec<_> = candidate
        .chars()
        .filter_map(|c| c.is_alphabetic().then_some(c.to_ascii_lowercase()))
        .collect();
    let orig_len = v.len();
    v.sort_unstable();
    v.dedup();
    orig_len == v.len()
}
