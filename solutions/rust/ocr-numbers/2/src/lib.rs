#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

#[rustfmt::skip]
const DIGITS: [[&str; 4]; 10] = [
    [" _ ",
     "| |", 
     "|_|", "   "],
    ["   ",
     "  |",
     "  |", "   "],
    [" _ ",
     " _|",
     "|_ ", "   "],
    [" _ ",
     " _|",
     " _|", "   "],
    ["   ",
     "|_|",
     "  |", "   "],
    [" _ ",
     "|_ ",
     " _|", "   "],
    [" _ ",
     "|_ ",
     "|_|", "   "],
    [" _ ",
     "  |",
     "  |", "   "],
    [" _ ",
     "|_|",
     "|_|", "   "],
    [" _ ",
     "|_|",
     " _|", "   "],
];

pub fn convert(input: &str) -> Result<String, Error> {
    let n_lines = input.lines().count();
    if !n_lines.is_multiple_of(4) {
        return Err(Error::InvalidRowCount(n_lines));
    }
    if let Some(line) = input.lines().find(|l| !l.len().is_multiple_of(3)) {
        return Err(Error::InvalidColumnCount(line.len()));
    }

    let mut lines = input.lines();
    let chunks = // Four lines at a time
        std::iter::from_fn(|| Some([lines.next()?, lines.next()?, lines.next()?, lines.next()?]));
    Ok(chunks
        .enumerate()
        .flat_map(|(chunk_idx, chunk_of_rows)| {
            let width = chunk_of_rows.iter().map(|row| row.len()).max().unwrap_or(0);
            // Three columns at a time
            let digits = (0..width).step_by(3).map(move |i| {
                let cell = chunk_of_rows.map(|row| &row[i..i + 3]);
                DIGITS
                    .iter()
                    .position(|&d| d == cell)
                    .and_then(|i| char::from_digit(i as u32, 10))
                    .unwrap_or('?')
            });
            (chunk_idx > 0).then_some(',').into_iter().chain(digits)
        })
        .collect())
}
