/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.to_lowercase().chars().fold(0, |acc, e| {
        let char_score = match e {
            'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' => 1,
            'd' | 'g' => 2,
            'b' | 'c' | 'm' | 'p' => 3,
            'f' | 'h'| 'v'| 'w'| 'y'=> 4,
            'k' => 5,
            'j' | 'x' => 8,
            'q' | 'z' => 10,
            _ => 0 
        };
        
        acc + char_score
    })
}
