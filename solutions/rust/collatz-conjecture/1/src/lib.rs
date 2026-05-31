pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut steps = 0;
    let mut m = n;
    while m != 1 {
        match m.is_multiple_of(2) {
            true => m /= 2,
            false => m = 3 * m + 1,
        }
        steps += 1;
    }
    Some(steps)
}
