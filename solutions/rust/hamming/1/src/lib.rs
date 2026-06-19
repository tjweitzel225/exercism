/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    (s1.len() == s2.len()).then_some(
        std::iter::zip(s1.chars(), s2.chars())
            .filter(|(x, y)| x != y)
            .count(),
    )
}
