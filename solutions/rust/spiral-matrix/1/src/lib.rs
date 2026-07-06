const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut dir_cycle = DIRECTIONS.iter().cycle();
    let mut step = dir_cycle.next().unwrap();
    let mut pos: (isize, isize) = (0, 0);
    let mut mat = vec![vec![0_u32; size as usize]; size as usize];
    for i in 1..=size.pow(2) {
        if mat
            .get((pos.1 + step.1) as usize)
            .and_then(|row| row.get((pos.0 + step.0) as usize))
            != Some(&0)
        {
            step = dir_cycle.next().unwrap();
        }
        mat[pos.1 as usize][pos.0 as usize] = i;
        pos.0 += step.0;
        pos.1 += step.1;
    }
    mat
}
