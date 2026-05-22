use std::{array, ops::Index};

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let length = garden.len();
    if length == 0 {
        return vec![];
    }
    let width = garden[0].len();
    if width == 0 {
        return vec!["".to_string()];
    }

    let count_flowers = |x: usize, y: usize| {
        let mut count = 0_u32;
        for dx in -1..=1_isize {
            for dy in -1..=1_isize {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let (nx, ny) = (x as isize + dx, y as isize + dy);
                if nx < 0 || width == nx as usize {
                    continue;
                }
                if ny < 0 || length == ny as usize {
                    continue;
                }
                if garden[ny as usize].as_bytes()[nx as usize] == b'*' {
                    count += 1;
                }
            }
        }
        count
    };

    let mut counts: Vec<String> = vec![];
    for y in 0..length {
        counts.push(String::new());
        for x in 0..width {
            let n = count_flowers(x, y);
            let next_char = if garden[y].chars().nth(x).unwrap() == '*' {
                '*'
            } else if n > 0 {
                std::char::from_digit(n, 10).unwrap()
            } else {
                ' '
            };
            counts[y].push(next_char);
        }
    }
    counts.to_vec()
}
