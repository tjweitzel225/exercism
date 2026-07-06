const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut dir_cycle = DIRECTIONS.iter().cycle();
    let mut dir = dir_cycle.next().unwrap();
    let (mut x, mut y) = (0_isize, 0_isize);
    let mut mat = vec![vec![0_u32; size as usize]; size as usize];
    for i in 1..=size.pow(2) {
        if mat
            .get((y + dir.1) as usize)
            .and_then(|row| row.get((x + dir.0) as usize))
            != Some(&0)
        {
            dir = dir_cycle.next().unwrap();
        }
        mat[y as usize][x as usize] = i;
        x += dir.0;
        y += dir.1;
    }
    mat
}
