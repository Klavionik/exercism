use std::collections::HashMap;

const STOP_CODON: &str = "STOP";

const CODONS_MAP: [(&str, &str); 17] = [
    ("AUG", "Methionine"),
    ("UUU", "Phenylalanine"),
    ("UUC", "Phenylalanine"),
    ("UUA", "Leucine"),
    ("UUG", "Leucine"),
    ("UCU", "Serine"),
    ("UCC", "Serine"),
    ("UCA", "Serine"),
    ("UCG", "Serine"),
    ("UAU", "Tyrosine"),
    ("UAC", "Tyrosine"),
    ("UGU", "Cysteine"),
    ("UGC", "Cysteine"),
    ("UGG", "Tryptophan"),
    ("UAA", STOP_CODON),
    ("UAG", STOP_CODON),
    ("UGA", STOP_CODON)
];

pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let codons_map = HashMap::from(CODONS_MAP);
    let mut amino_acids = vec![];

    let mut i = 0;

    while i < rna.len() {
        let (start, end) = (i, (i + 3).min(rna.len()));
        i = end;
        
        let next = &rna[start..end];
        let acid = codons_map.get(next)?;

        if *acid == STOP_CODON {
            break
        }

        amino_acids.push(*acid);
    }

    Some(amino_acids)
}