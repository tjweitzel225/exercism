use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut palindromes_to_factors: HashMap<u64, HashSet<(u64, u64)>> = (min..=max)
        .flat_map(|i| (i..=max).map(move |j| (i * j, (i, j))))
        .filter(|(product, _)| {
            let s = product.to_string();
            s.chars().eq(s.chars().rev())
        })
        .fold(HashMap::new(), |mut acc, (val, factors)| {
            acc.entry(val).or_default().insert(factors);
            acc
        });
    let &first = palindromes_to_factors.keys().min()?;
    let &last = palindromes_to_factors.keys().max()?;
    Some((
        Palindrome {
            value: first,
            factors: palindromes_to_factors.remove(&first)?,
        },
        Palindrome {
            value: last,
            factors: palindromes_to_factors.remove(&last)?,
        },
    ))
}
