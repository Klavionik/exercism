const A: u8 = b'A';
const T: u8 = b'T';
const G: u8 = b'G';
const C: u8 = b'C';
const U: u8 = b'U';

#[derive(Debug, PartialEq, Eq)]
pub struct Dna(Vec<u8>);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(Vec<u8>);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let mut nucleotides = vec![];

        for (i, nucleotide) in dna.bytes().enumerate() {
            if !is_dna_nucleotide(nucleotide) {
                return Err(i);
            }

            nucleotides.push(nucleotide)
        }

        Ok(Self(nucleotides))
    }

    pub fn into_rna(self) -> Rna {
        let mut nucleotides = String::new();

        for nucleotide in self.0 {
            match nucleotide {
                A => nucleotides.push('U'),
                T => nucleotides.push('A'),
                G => nucleotides.push('C'),
                C => nucleotides.push('G'),
                _ => panic!("Invalid DNA nucleotide {nucleotide}."),
            }
        }

        Rna::new(&nucleotides).unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let mut nucleotides = vec![];

        for (i, nucleotide) in rna.bytes().enumerate() {
            if !is_rna_nucleotide(nucleotide) {
                return Err(i);
            }

            nucleotides.push(nucleotide)
        }

        Ok(Self(nucleotides))
    }
}

fn is_dna_nucleotide(nucleotide: u8) -> bool {
    nucleotide == A || nucleotide == T || nucleotide == G || nucleotide == C
}

fn is_rna_nucleotide(nucleotide: u8) -> bool {
    nucleotide == A || nucleotide == U || nucleotide == G || nucleotide == C
}