pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut factors = vec![];
    while n.is_multiple_of(2) {
        factors.push(2);
        n /= 2
    }
    let mut odd_divisor = 3;
    while odd_divisor <= n / odd_divisor {
        while n.is_multiple_of(odd_divisor) {
            factors.push(odd_divisor);
            n /= odd_divisor
        }
        odd_divisor += 2
    }
    if n > 1 {
        factors.push(n);
    }
    factors
}
