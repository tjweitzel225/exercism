const DIGITS_TO_WORDS: [&str; 11] = [
    "No", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
];
pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut current_bottles = start_bottles;
    let mut song = String::new();
    for _ in 0..take_down {
        let start_word = DIGITS_TO_WORDS[current_bottles as usize];
        let end_word = DIGITS_TO_WORDS[(current_bottles.saturating_sub(1)) as usize].to_lowercase();
        let maybe_start_s = if start_word == "One" { "" } else { "s" };
        let maybe_end_s = if end_word == "one" { "" } else { "s" };
        current_bottles -= 1;

        song.push_str(&format!(
            "
{start_word} green bottle{maybe_start_s} hanging on the wall,
{start_word} green bottle{maybe_start_s} hanging on the wall,
And if one green bottle should accidentally fall,
There'll be {end_word} green bottle{maybe_end_s} hanging on the wall.
"
        ));
    }
    song
}
