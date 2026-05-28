pub fn raindrops(n: u32) -> String {
    let mut s = String::new();
    if n.is_multiple_of(3) {
        s.push_str("Pling")
    }
    if n.is_multiple_of(5) {
        s.push_str("Plang")
    }
    if n.is_multiple_of(7) {
        s.push_str("Plong")
    }
    if s.is_empty() { n.to_string() } else { s }
}
