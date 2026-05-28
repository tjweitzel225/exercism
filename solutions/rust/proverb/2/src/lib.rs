pub fn build_proverb(list: &[&str]) -> String {
    match list.first() {
        None => String::new(),
        Some(first_word) => list
            .windows(2)
            .map(|win| {
                let [a, b] = [win[0], win[1]];
                format!("For want of a {a} the {b} was lost.\n")
            })
            .chain([format!("And all for the want of a {first_word}.")])
            .collect(),
    }
}
