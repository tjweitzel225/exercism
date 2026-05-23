pub fn is_armstrong_number(num: u32) -> bool {
    let stringified = num.to_string();
    let n_digits = stringified.len() as u32;
    num == stringified
        .chars()
        .map(|n| n.to_digit(10).unwrap().pow(n_digits))
        .sum()
}
