pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let mut proteins = Vec::new();
    for codon in rna.as_bytes().chunks(3) {
        let protein = match str::from_utf8(codon).ok()? {
            "AUG" => "Methionine",
            "UUU" | "UUC" => "Phenylalanine",
            "UUA" | "UUG" => "Leucine",
            "UCU" | "UCC" | "UCA" | "UCG" => "Serine",
            "UAU" | "UAC" => "Tyrosine",
            "UGU" | "UGC" => "Cysteine",
            "UGG" => "Tryptophan",
            "UAA" | "UAG" | "UGA" => break,
            _ => return None,
        };
        proteins.push(protein)
    }
    Some(proteins)
}
