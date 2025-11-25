use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    let letters: HashSet<char> = sentence
        .chars()
        .filter(|c| c.is_alphabetic())
        .flat_map(|c| c.to_lowercase())
        .collect();

    letters.len() == 26
}