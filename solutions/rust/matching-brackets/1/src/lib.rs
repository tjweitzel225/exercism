pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for c in string.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' | ']' | '}' => {
                let Some(last_opener) = stack.pop() else {
                    return false;
                };
                match (last_opener, c) {
                    ('(', ')') | ('[', ']') | ('{', '}') => {}
                    _ => return false,
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}
