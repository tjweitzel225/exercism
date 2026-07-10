#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

fn vlq_iter(n: &u32) -> impl Iterator<Item = u8> {
    let num_bytes = (32 - n.leading_zeros()).max(1).div_ceil(7);
    (0..num_bytes).rev().map(move |i| {
        let byte = ((n >> (i * 7)) & 0b0111_1111) as u8;
        if i == 0 { byte } else { byte | 0b1000_0000 }
    })
}

pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(vlq_iter).collect()
}

fn decode_vlq(vlq: &[u8]) -> Result<u32, Error> {
    vlq.iter().enumerate().try_fold(0, |acc, (i, b)| {
        if i == vlq.len() - 1 && b & 0b1000_0000 != 0 {
            Err(Error::IncompleteNumber)
        } else {
            Ok(acc << 7 | (b & 0b0111_1111) as u32)
        }
    })
}

// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    bytes
        .split_inclusive(|b| b & 0b1000_0000 == 0)
        .map(decode_vlq)
        .collect()
}
