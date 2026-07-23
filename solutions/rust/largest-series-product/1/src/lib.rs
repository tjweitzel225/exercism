#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if let Some(c) = string_digits.chars().find(|c| !c.is_numeric()) {
        Err(Error::InvalidDigit(c))
    } else if span == 0 {
        Ok(1)
    } else {
        string_digits
            .as_bytes()
            .windows(span)
            .map(|win| win.iter().map(|&c| (c - b'0') as u64).product())
            .max()
            .ok_or(Error::SpanTooLong)
    }
}
