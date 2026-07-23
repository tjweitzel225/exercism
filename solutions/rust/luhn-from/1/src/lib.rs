pub struct Luhn(bool);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        self.0
    }
}

impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Luhn(
            input
                .to_string()
                .chars()
                .rev()
                .filter(|c| !c.is_whitespace())
                .try_fold((0, 0), |(sum, count), val| {
                    val.to_digit(10)
                        .map(|n| if count % 2 == 1 { n * 2 } else { n })
                        .map(|n| if n > 9 { n - 9 } else { n })
                        .map(|n| (n + sum, count + 1))
                })
                .is_some_and(|(sum, count)| sum % 10 == 0 && count > 1),
        )
    }
}
