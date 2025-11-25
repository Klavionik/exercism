use std::collections::HashMap;

const VALID_NUCLEOTIDES: &str = "ATGC";

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !VALID_NUCLEOTIDES.contains(nucleotide) {
        return Err(nucleotide)
    }
    
    let mut count = 0;

    for n in dna.chars() {
        if !VALID_NUCLEOTIDES.contains(n) {
            return Err(n)
        }
        
        if n == nucleotide {
            count += 1
        }
    }
    
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::from_iter([('A', 0), ('T', 0), ('G', 0), ('C', 0)]);
    
    for nucleotide in dna.chars() {
        if !VALID_NUCLEOTIDES.contains(nucleotide) {
            return Err(nucleotide)
        }
        
        counts.insert(nucleotide, counts[&nucleotide] + 1);
    }
    
    Ok(counts)
}