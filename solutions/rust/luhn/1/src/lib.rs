/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if !code
        .chars()
        .all(|c| c.is_ascii_digit() || c.is_whitespace())
    {
        return false;
    }

    let digits: Vec<u32> = code.chars().filter_map(|c| c.to_digit(10)).collect();
    if digits.len() <= 1 {
        return false;
    }

    const LUHN_RULES: [fn(u32) -> u32; 2] = [
        |n: u32| n,
        |n: u32| match 2 * n {
            d if d > 9 => d - 9,
            d => d,
        },
    ];
    digits
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, n)| LUHN_RULES[i % 2](n))
        .sum::<u32>()
        % 10
        == 0
}
