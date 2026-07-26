fn key_shifter(key: &str) -> impl Iterator<Item = u8> {
    key.chars().map(|k| k as u8 - b'a').cycle()
}

fn shift(mut shifter: impl Iterator<Item = u8>, s: &str) -> String {
    s.chars()
        .flat_map(|c| {
            Some(match c {
                'a'..='z' => ((c as u8 - b'a' + shifter.next()?) % 26 + b'a') as char,
                'A'..='Z' => ((c as u8 - b'A' + shifter.next()?) % 26 + b'A') as char,
                _ => c,
            })
        })
        .collect()
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    let shifts = key_shifter(key);
    (!key.is_empty() && key.chars().all(|c| c.is_lowercase())).then(|| shift(shifts, s))
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    let shifts = key_shifter(key).map(|k| 26 - k);
    (!key.is_empty() && key.chars().all(|c| c.is_lowercase())).then(|| shift(shifts, s))
}

pub fn encode_random(s: &str) -> (String, String) {
    let key: String = rand::random_iter()
        .take(100)
        .map(|i: u8| (b'a' + (i % 26)) as char)
        .collect();
    let encoded = encode(&key, s).expect("Key is okay by construction.");
    (key, encoded)
}
