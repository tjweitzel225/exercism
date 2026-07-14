fn reverse_alphabetically(plain: &str) -> String {
    plain
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| {
            if c.is_numeric() {
                c
            } else {
                let i = c as u8 - b'a';
                (b'a' + 25 - i) as char
            }
        })
        .collect()
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    reverse_alphabetically(plain)
        .as_bytes()
        .chunks(5)
        .map(|chunk| str::from_utf8(chunk).expect("Came from a valid &str."))
        .collect::<Vec<&str>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    reverse_alphabetically(cipher)
}
