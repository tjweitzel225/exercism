pub fn encode(source: &str) -> String {
    source
        .chars()
        .chain(std::iter::once('_'))
        .fold(
            (String::new(), None, 0),
            |(mut acc, last_c, run_counter), c| match last_c {
                None => (acc, Some(c), run_counter + 1),
                Some(prev) if prev == c => (acc, last_c, run_counter + 1),
                Some(prev) => {
                    if run_counter == 1 {
                        acc.push(prev)
                    } else {
                        acc.push_str(&format!("{}{}", run_counter, prev))
                    }
                    (acc, Some(c), 1)
                }
            },
        )
        .0
}

pub fn decode(source: &str) -> String {
    source
        .chars()
        .fold((String::new(), String::new()), |(mut acc, mut nums), c| {
            if c.is_numeric() {
                nums.push(c);
            } else {
                acc.push_str(&c.to_string().repeat(nums.parse().unwrap_or(1)));
                nums.clear();
            }
            (acc, nums)
        })
        .0
}
