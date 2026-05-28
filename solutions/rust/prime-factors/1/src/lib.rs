fn is_prime(&n: &u64) -> bool {
    match n {
        2 => true,
        _ if n <= 1 || n.is_multiple_of(2) => false,
        _ => (3..=n.isqrt()).step_by(2).all(|i| !n.is_multiple_of(i)),
    }
}
pub fn factors(n: u64) -> Vec<u64> {
    (2..=n)
        .filter(is_prime)
        .fold((vec![], n), |(mut factors, m), i: u64| {
            let mut next_m = m;
            while next_m.is_multiple_of(i) {
                factors.push(i);
                next_m /= i;
            }
            (factors, next_m)
        })
        .0
}
