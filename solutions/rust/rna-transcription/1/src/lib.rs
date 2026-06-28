#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);
const DNA_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);
const RNA_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'U'];

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        if let Some(i) = dna.find(|c| !DNA_NUCLEOTIDES.contains(&c)) {
            Err(i)
        } else {
            Ok(Dna(dna.to_string()))
        }
    }

    pub fn into_rna(self) -> Rna {
        Rna::new(
            &self
                .0
                .chars()
                .map(|c| match c {
                    'G' => 'C',
                    'C' => 'G',
                    'T' => 'A',
                    'A' => 'U',
                    _ => unreachable!(),
                })
                .collect::<String>(),
        )
        .unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        if let Some(i) = rna.find(|c| !RNA_NUCLEOTIDES.contains(&c)) {
            Err(i)
        } else {
            Ok(Rna(rna.to_string()))
        }
    }
}
