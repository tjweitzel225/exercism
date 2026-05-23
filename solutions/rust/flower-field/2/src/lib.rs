pub fn annotate(garden: &[&str]) -> Vec<String> {
    let garden_bytes: Vec<&[u8]> = garden.iter().map(|row| row.as_bytes()).collect();

    const NEIGHBORS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let count_flowers = |x: usize, y: usize| match garden_bytes[y][x] {
        b'*' => '*',
        _ => match NEIGHBORS
            .iter()
            .filter_map(|&(dx, dy)| {
                let nx = x.checked_add_signed(dx)?;
                let ny = y.checked_add_signed(dy)?;
                garden_bytes.get(ny)?.get(nx)
            })
            .filter(|&&c| c == b'*')
            .count() as u8
        {
            0 => ' ',
            n => char::from(b'0' + n),
        },
    };

    garden_bytes
        .iter()
        .enumerate()
        .map(|(y, row)| (0..row.len()).map(|x| count_flowers(x, y)).collect())
        .collect()
}
