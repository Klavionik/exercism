use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    let mut counter = HashMap::<char, u32>::new();

    for ch in candidate.to_ascii_lowercase().chars() {
        if ch == '-' || ch == ' ' {
            continue;
        }

        let occurences = counter
            .entry(ch)
            .and_modify(|occurrences| *occurrences += 1)
            .or_insert(1);

        if *occurences > 1 {
            return false;
        }
    }

    true
}