pub fn build_proverb(list: &[&str]) -> String {
    let Some(first_thing) = list.first() else {
        return String::new();
    };
    list.windows(2)
        .map(|win| {
            let [a, b] = [win[0], win[1]];
            format!("For want of a {a} the {b} was lost.\n")
        })
        .chain([format!("And all for the want of a {first_thing}.")])
        .collect()
}
