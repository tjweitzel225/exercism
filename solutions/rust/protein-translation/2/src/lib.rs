pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let mut proteins = Vec::new();
    for codon in rna.as_bytes().chunks(3) {
        let protein = match codon {
            b"AUG" => "Methionine",
            b"UUU" | b"UUC" => "Phenylalanine",
            b"UUA" | b"UUG" => "Leucine",
            b"UCU" | b"UCC" | b"UCA" | b"UCG" => "Serine",
            b"UAU" | b"UAC" => "Tyrosine",
            b"UGU" | b"UGC" => "Cysteine",
            b"UGG" => "Tryptophan",
            b"UAA" | b"UAG" | b"UGA" => break,
            _ => return None,
        };
        proteins.push(protein)
    }
    Some(proteins)
}
