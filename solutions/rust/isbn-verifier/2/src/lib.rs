/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    isbn.chars()
        .rev()
        .filter(|&c| c != '-')
        .enumerate()
        .try_fold((0, 0), |(_, sum), (i, c)| {
            let a = if i == 0 && c == 'X' {
                100
            } else {
                c.to_digit(10)? * (10 - i as u32)
            };
            Some((i + 1, sum + a))
        })
        .is_some_and(|(count, total)| count == 10 && total.rem_euclid(11) == 0)
}
