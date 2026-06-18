use std::{collections::BTreeMap, iter::repeat};

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .flat_map(|(val, letters)| {
            letters
                .iter()
                .map(|l| l.to_lowercase().next().unwrap())
                .zip(repeat(*val))
        })
        .collect()
}
