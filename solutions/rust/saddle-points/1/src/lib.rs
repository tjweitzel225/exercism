use itertools::Itertools;
use std::collections::HashSet;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut row_maxima: HashSet<(usize, usize)> = HashSet::new();
    let mut col_minima: HashSet<(usize, usize)> = HashSet::new();
    for (i, row) in input.iter().enumerate() {
        if let Some(max) = row.iter().max() {
            row_maxima.extend(row.iter().positions(|v| v == max).map(|j_max| (i, j_max)));
        }
    }
    for j in 0..input[0].len() {
        let col = input.iter().map(|row| row[j]);
        if let Some(min) = col.clone().min() {
            col_minima.extend(col.positions(|v| v == min).map(|i_min| (i_min, j)));
        }
    }
    row_maxima.intersection(&col_minima).cloned().collect()
}
