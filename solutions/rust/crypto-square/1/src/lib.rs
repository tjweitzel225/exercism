pub fn encrypt(input: &str) -> String {
    let normalized: String = input
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();

    let l = normalized.len();
    if l == 0 {
        return String::new();
    }

    let c = l.isqrt();
    let c = c + usize::from(c * c < l);
    (0..c).fold(String::with_capacity(l + c - 1), |mut acc, col| {
        for row in normalized.as_bytes().chunks(c) {
            acc.push(*row.get(col).unwrap_or(&b' ') as char);
        }
        if col != c - 1 {
            acc.push(' ');
        }
        acc
    })
}
