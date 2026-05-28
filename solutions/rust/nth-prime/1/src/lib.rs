fn is_prime(&n: &u32) -> bool {
    if n == 2 {
        return true;
    }
    if n <= 1 || n.is_multiple_of(2) {
        return false;
    }
    !(3..=n.isqrt()).step_by(2).any(|i| n.is_multiple_of(i))
}
pub fn nth(n: u32) -> u32 {
    (0..)
        .filter(is_prime)
        .take((n + 1) as usize)
        .last()
        .unwrap()
}
