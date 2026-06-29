pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    (0..input.len())
        .flat_map(|i| (0..input[i].len()).map(move |j| (i, j)))
        .filter(|&(i, j)| input[i].iter().all(|&n| input[i][j] >= n))
        .filter(|&(i, j)| input.iter().map(|row| row[j]).all(|n| input[i][j] <= n))
        .collect()
}
