pub fn abbreviate(phrase: &str) -> String {
    let phrase = phrase
        .replace("-", " ")
        .replace("_", "");
    
    let words = phrase.split_whitespace();
    
    let mut abbreviation = String::new();
    
    for word in words {
        if word.to_uppercase() == word {
            abbreviation.push(word.chars().next().unwrap());
            continue
        }
        
        for (i, c) in word.chars().enumerate() {
            if i == 0 {
                abbreviation.extend(c.to_uppercase());
                continue
            }
            
            if c.is_uppercase() {
                abbreviation.push(c);
            }
        }
    }
    
    abbreviation
}