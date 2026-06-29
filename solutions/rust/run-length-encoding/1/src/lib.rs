use std::iter::once;
pub fn encode(source: &str) -> String {
    let mut run_counter = 0;
    let mut last_c = None;
    source
        .chars()
        .chain(once('_'))
        .fold(String::new(), |mut acc, c| match last_c {
            Some(prev) if prev == c => {
                run_counter += 1;
                acc
            }
            None => {
                last_c = Some(c);
                run_counter += 1;
                acc
            }
            Some(prev) => {
                if run_counter == 1 {
                    acc.push(prev)
                } else {
                    acc.push_str(&format!("{}{}", run_counter, prev))
                }
                run_counter = 1;
                last_c = Some(c);
                acc
            }
        })
}

pub fn decode(source: &str) -> String {
    let mut out = String::new();
    let mut buf = String::new();
    let mut nums = String::new();
    for c in source.chars() {
        if c.is_numeric() {
            nums.push(c)
        } else {
            if nums.is_empty() {
                buf.push(c);
            } else {
                for _ in 0..nums.parse().unwrap() {
                    buf.push(c);
                }
                nums.clear();
            }
            out.push_str(&buf);
            buf.clear();
        }
    }
    out
}
