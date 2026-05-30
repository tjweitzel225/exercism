pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: Vec<u32> = factors
        .iter()
        .filter(|&&f| f != 0)
        .flat_map(|&f| (f..limit).step_by(f as usize))
        .collect();
    multiples.sort_unstable();
    multiples.dedup();
    multiples.iter().sum()
}
