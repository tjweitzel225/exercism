fn mirror<T: Clone>(head: Vec<T>) -> Vec<T> {
    head.iter()
        .chain(head[..head.len() - 1].iter().rev())
        .cloned()
        .collect()
}

pub fn get_diamond(c: char) -> Vec<String> {
    let n_rows = ('A'..='Z')
        .position(|l| l == c)
        .expect("c wasn't in the uppercase alphabet!")
        + 1;
    let mut letters = 'A'..='Z';
    let top: Vec<String> = (0..n_rows)
        .map(|i| {
            let mut row = vec![b' '; n_rows - 1];
            row.insert(
                (n_rows - 1) - i,
                letters.next().expect("Too many letters!") as u8,
            );
            String::from_utf8(mirror(row)).expect("Oh no! Not utf8!")
        })
        .collect();
    mirror(top)
}
