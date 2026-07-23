pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        self.to_string()
            .chars()
            .rev()
            .filter(|c| !c.is_whitespace())
            .try_fold((0, 0), |(sum, count), val| {
                val.to_digit(10)
                    .map(|n| if count % 2 == 1 { n * 2 } else { n })
                    .map(|n| if n > 9 { n - 9 } else { n })
                    .map(|n| (n + sum, count + 1))
            })
            .is_some_and(|(sum, count)| sum % 10 == 0 && count > 1)
    }
}
