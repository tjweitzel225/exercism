pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let n = size as usize;
    let coords = (0..n.div_ceil(2)).flat_map(|layer| {
        let (min, max) = (layer, n - 1 - layer);
        let top = (min..=max).map(move |x| (x, min));
        let right = (min + 1..=max).map(move |y| (max, y));
        let bottom = (min..max).rev().map(move |x| (x, max));
        let left = (min + 1..max).rev().map(move |y| (min, y));
        top.chain(right).chain(bottom).chain(left)
    });
    let mut mat = vec![vec![0_u32; n]; n];
    for (i, (x, y)) in (1..).zip(coords) {
        dbg!(i, (x, y));
        mat[y][x] = i;
    }
    mat
}
