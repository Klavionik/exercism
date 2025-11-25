use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    let characters: HashSet<char> = sentence
        .chars()
        .filter(|c| c.is_alphabetic())
        .flat_map(|c| c.to_lowercase())
        .collect();

    characters.len() == 26
}