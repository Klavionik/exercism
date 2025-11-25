const STOP_CODON: &str = "STOP";
const CODON_LENGTH: usize = 3;

pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let mut amino_acids = vec![];

    let mut i = 0;

    while i < rna.len() {
        let (start, end) = (i, (i + CODON_LENGTH).min(rna.len()));
        i = end;
        
        let codon = &rna[start..end];
        let acid = codon_to_amino_acid(codon)?;

        if acid == STOP_CODON {
            break
        }

        amino_acids.push(acid);
    }

    Some(amino_acids)
}

fn codon_to_amino_acid(codon: &str) -> Option<&str> {
    let amino_acid = match codon {
        "AUG" => "Methionine",
        "UUU" | "UUC" => "Phenylalanine",
        "UUA" | "UUG" => "Leucine",
        "UCU" | "UCC" | "UCA" | "UCG" => "Serine",
        "UAU" | "UAC" => "Tyrosine",
        "UGU" | "UGC" => "Cysteine",
        "UGG" => "Tryptophan",
        "UAA" | "UAG" | "UGA" => STOP_CODON,
        _ => ""
    };
    
    if amino_acid.is_empty() { None } else { Some(amino_acid) }
}