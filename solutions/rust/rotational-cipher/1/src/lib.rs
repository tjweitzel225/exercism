const ALPHABET: &[u8] = "abcdefghijklmnopqrstuvwxyz".as_bytes();
pub fn rotate(input: &str, key: u8) -> String {
    let mut rotated = [0; 26];
    rotated.copy_from_slice(ALPHABET);
    rotated.rotate_left(key as usize);
    input
        .chars()
        .map(|c| match c {
            c if c.is_ascii_uppercase() => {
                let i = (c as u8 - b'A') as usize;
                rotated[i].to_ascii_uppercase() as char
            }
            c if c.is_ascii_lowercase() => {
                let i = (c as u8 - b'a') as usize;
                rotated[i] as char
            }
            _ => c,
        })
        .collect()
}
