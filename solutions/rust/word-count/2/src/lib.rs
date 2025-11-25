use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .replace(",", " ")
        .split_ascii_whitespace()
        .map(|word| word.trim_matches(|c: char| !c.is_ascii_alphanumeric()))
        .map(|word| word.to_ascii_lowercase())
        .fold(HashMap::new(), |mut acc, word| {
            acc.entry(word.to_string())
                .and_modify(|v| *v += 1)
                .or_insert(1);
            acc
        })
}