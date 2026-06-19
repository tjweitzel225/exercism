/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let chars = isbn.chars().filter(|&c| c != '-').rev();
    chars.clone().count() == 10
        && chars
            .enumerate()
            .map(|(i, c)| {
                (i == 0 && c == 'X')
                    .then_some(100)
                    .or_else(|| c.to_digit(10).map(|c_| c_ * (10 - i as u32)))
            })
            .sum::<Option<u32>>()
            .is_some_and(|s| s.rem_euclid(11) == 0)
}
