pub fn number(user_number: &str) -> Option<String> {
    let mut s = user_number.to_string();

    s.retain(char::is_numeric);
    if s.starts_with('1') {
        s.remove(0);
    }

    (s.len() == 10
        && !matches!(s.chars().next(), Some('0') | Some('1'))
        && !matches!(s.chars().nth(3), Some('0') | Some('1')))
    .then_some(s)
}
