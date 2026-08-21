fn is_corner(b: u8) -> bool {
    b == b'+'
}

fn is_horizontal(b: u8) -> bool {
    matches!(b, b'-' | b'+')
}

fn is_vertical(b: u8) -> bool {
    matches!(b, b'|' | b'+')
}

pub fn count(lines: &[&str]) -> u32 {
    let grid: Vec<&[u8]> = lines.iter().map(|line| line.as_bytes()).collect();
    let (height, width) = match grid.first() {
        Some(row) => (grid.len(), row.len()),
        None => return 0,
    };
    let xy = |x: usize, y: usize| grid[y][x];
    (0..height)
        .flat_map(|top| (0..width).map(move |left| (top, left)))
        // Candidate top-left corners
        .filter(|&(top, left)| is_corner(xy(left, top)))
        // Candidate top edges
        .flat_map(|(top, left)| {
            (left + 1..width)
                .take_while(move |&right| is_horizontal(xy(right, top)))
                .filter(move |&right| is_corner(xy(right, top)))
                .map(move |right| (top, left, right))
        })
        // Candidate right edges
        .flat_map(|(top, left, right)| {
            (top + 1..height)
                .take_while(move |&bottom| is_vertical(xy(right, bottom))) // break
                .filter(move |&bottom| is_corner(xy(right, bottom))) // continue
                .map(move |bottom| (top, left, right, bottom))
        })
        // Complete the rectangle
        .filter(|&(top, left, right, bottom)| {
            is_corner(xy(left, bottom))
                && (left + 1..right).all(|x| is_horizontal(xy(x, bottom)))
                && (top + 1..bottom).all(|y| is_vertical(xy(left, y)))
        })
        .count() as u32
}
