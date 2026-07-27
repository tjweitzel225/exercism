pub fn actions(n: u8) -> Vec<&'static str> {
    let mut out = Vec::new();
    for (i, action) in [
        (1, "wink"),
        (2, "double blink"),
        (4, "close your eyes"),
        (8, "jump"),
    ] {
        if i & n == i {
            out.push(action);
        }
    }
    if 16 & n == 16 {
        out.reverse();
    }
    out
}
