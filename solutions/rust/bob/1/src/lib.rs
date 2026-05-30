pub fn reply(message: &str) -> &str {
    let question = message.trim().ends_with("?");
    let yelled = message
        .chars()
        .filter_map(|c| c.is_alphabetic().then_some(c.is_uppercase()))
        .reduce(|acc, b| acc && b)
        .unwrap_or(false);
    match (question, yelled) {
        (true, false) => "Sure.",
        (false, true) => "Whoa, chill out!",
        (true, true) => "Calm down, I know what I'm doing!",
        _ if message.trim().is_empty() => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
