use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .chars()
        .map(|ch| if ch == ',' { ' ' } else { ch })
        .filter(|&ch| ch.is_ascii_alphanumeric() || ch.is_ascii_whitespace() || ch == '\'')
        .collect::<String>()
        .split_ascii_whitespace()
        .map(|word| word.trim_matches('\'').to_lowercase())
        .fold(HashMap::new(), |mut acc, word| {
            acc.entry(word.to_string())
                .and_modify(|v| *v += 1)
                .or_insert(1);

            acc
        })
}