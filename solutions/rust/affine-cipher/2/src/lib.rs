/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

fn are_coprime(mut a: i32, mut b: i32) -> bool {
    while b != 0 {
        (a, b) = (b, a.rem_euclid(b));
    }
    a.abs() == 1
}

fn encrypt(c: char, a: i32, b: i32) -> char {
    if c.is_ascii_digit() {
        c
    } else {
        let i = c.to_ascii_lowercase() as i32 - 'a' as i32;
        let d = (a * i + b).rem_euclid(26) as usize;
        char::from(d as u8 + b'a')
    }
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !are_coprime(a, 26) {
        Err(AffineCipherError::NotCoprime(a))
    } else {
        Ok(plaintext
            .chars()
            .filter(|&p| char::is_alphanumeric(p))
            .map(|p| encrypt(p, a, b))
            .collect::<String>()
            // Group into 5-char chunks
            .as_bytes()
            .chunks(5)
            .map(|chunk| str::from_utf8(chunk).expect("Valid chunks by construction."))
            .collect::<Vec<&str>>()
            .join(" "))
    }
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
/// D(y) = (a^-1)(y - b) mod m
fn mmi(a: i32, m: i32) -> Option<i32> {
    (1..m).find(|n| a * n % m == 1)
}

fn decrypt(y: char, a: i32, b: i32) -> char {
    let mmi = mmi(a, 26).expect("Decode should check for coprimality.");
    if y.is_ascii_digit() {
        y
    } else {
        let yi = y as i32 - 'a' as i32;
        let d = (mmi * (yi - b)).rem_euclid(26) as usize;
        char::from(d as u8 + b'a')
    }
}

pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !are_coprime(a, 26) {
        Err(AffineCipherError::NotCoprime(a))
    } else {
        Ok(ciphertext
            .chars()
            .filter(|&p| char::is_alphanumeric(p))
            .map(|c| decrypt(c, a, b))
            .collect())
    }
}
