use std::collections::HashMap;

const NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut count = 0;
    if !NUCLEOTIDES.contains(&nucleotide) {
        return Err(nucleotide);
    }
    for c in dna.chars() {
        if !NUCLEOTIDES.contains(&c) {
            return Err(c);
        }
        if c == nucleotide {
            count += 1
        }
    }
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::new();
    for c in NUCLEOTIDES {
        counts.insert(c, count(c, dna)?);
    }
    Ok(counts)
}
