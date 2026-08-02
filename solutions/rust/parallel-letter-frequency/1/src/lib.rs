use rayon::prelude::*;
use std::collections::HashMap;

type CharCount = HashMap<char, usize>;

pub fn frequency(input: &[&str], worker_count: usize) -> CharCount {
    input
        .par_iter()
        .with_min_len(input.len().div_ceil(worker_count.max(1)))
        .fold(CharCount::new, |mut acc, line| {
            for c in line
                .chars()
                .filter(|c| c.is_alphabetic())
                .flat_map(|c| c.to_lowercase())
            {
                *acc.entry(c).or_insert(0) += 1;
            }
            acc
        })
        .reduce(CharCount::new, |mut acc, other| {
            for (c, n) in other {
                *acc.entry(c).or_insert(0) += n;
            }
            acc
        })
}
