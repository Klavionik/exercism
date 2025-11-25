use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();
    let word = word.to_lowercase();
    let mut target_sorted = word.chars().collect::<Vec<_>>();
    target_sorted.sort();

    for candidate in possible_anagrams {
        let lc_candidate = candidate.to_lowercase();
        
        if lc_candidate == word || lc_candidate.len() != word.len() {
            continue
        }

        let mut candidate_sorted = lc_candidate.chars().collect::<Vec<_>>();
        candidate_sorted.sort();
        
        if candidate_sorted == target_sorted {
            result.insert(*candidate);
        }
    }
    
    result
}