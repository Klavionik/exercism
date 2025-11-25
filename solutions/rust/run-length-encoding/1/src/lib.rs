pub fn encode(source: &str) -> String {
    if source.is_empty() {
        return "".to_string();
    }

    let mut chars = source.chars();

    let mut encoded = String::new();
    let mut curr_char: Option<char> = chars.next();
    let mut curr_occurrences = 0;

    loop {
        curr_occurrences += 1;

        let next_char = chars.next();

        if next_char != curr_char {
            if curr_occurrences > 1 {
                encoded.push_str(&curr_occurrences.to_string());
            }

            encoded.push(curr_char.unwrap());
            curr_char = next_char;
            curr_occurrences = 0;
        }

        if next_char.is_none() {
            break
        }
    }

    encoded
}

pub fn decode(source: &str) -> String {
    if source.is_empty() {
        return "".to_string();
    }
    
    let mut decoded = String::new();
    let mut occurrences = String::new();
    
    for ch in source.chars() {
        if ch.is_ascii_digit() {
            occurrences.push(ch);
            continue
        }
        
        if !occurrences.is_empty() {
            let occurrences_count = occurrences.parse::<usize>().unwrap();
            
            for _ in 1..occurrences_count {
                decoded.push(ch)
            }
            
            occurrences.clear()
        }
        
        decoded.push(ch)
    }
    
    decoded
}