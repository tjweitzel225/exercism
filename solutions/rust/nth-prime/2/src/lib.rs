fn is_prime(&n: &u32) -> bool {
    match n {
        2 => true,
        _ if n <= 1 || n.is_multiple_of(2) => false,
        _ => !(3..=n.isqrt()).step_by(2).any(|i| n.is_multiple_of(i)),
    }
}
pub fn nth(n: u32) -> u32 {
    (0..).filter(is_prime).nth(n as usize).unwrap()
}
