use std::collections::HashMap;

const BASE_PRICE: u32 = 800;
const DISCOUNTS: [u32; 5] = [0, 5, 10, 20, 25];

fn group_price(k: u32) -> u32 {
    k * BASE_PRICE * (100 - DISCOUNTS[k as usize - 1]) / 100
}

pub fn lowest_price(books: &[u32]) -> u32 {
    let mut counts: Vec<u32> = books
        .iter()
        .fold(HashMap::new(), |mut acc, &b| {
            *acc.entry(b).or_insert(0) += 1;
            acc
        })
        .values()
        .copied()
        .collect();
    counts.sort_unstable_by(|a, b| b.cmp(a));
    fn cheapest_grouping(counts: &[u32]) -> u32 {
        (1..=counts.len())
            .map(|k| {
                let mut next = counts.to_vec();
                for count in next.iter_mut().take(k) {
                    *count -= 1;
                }
                next.retain(|&x| x != 0);
                next.sort_unstable_by(|a, b| b.cmp(a));
                group_price(k as u32) + cheapest_grouping(&next)
            })
            .min()
            .unwrap_or(0)
    }
    cheapest_grouping(&counts)
}
