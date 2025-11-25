const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn is_consonant(c: &char) -> bool {
    !VOWELS.contains(c)
}

fn check_rule_1(input: &str) -> bool {
    input.starts_with(VOWELS) || input.starts_with("xr") || input.starts_with("yt")
}

fn check_rule_2(input: &str) -> bool {
    !input.starts_with(VOWELS)
}

fn check_rule_3(input: &str) -> bool {
    if let Some(qu_idx) = input.find("qu") {
         return input[..qu_idx].chars().all(|letter| is_consonant(&letter))
    }
    
    false
}

fn check_rule_4(input: &str) -> bool {
    if let Some(y_idx) = input.find("y") {
        return y_idx != 0 && input[..y_idx].chars().all(|letter| is_consonant(&letter))
    }

    false
}

pub fn translate(input: &str) -> String {
    let mut translated = String::new();
    let words = input.split_ascii_whitespace().enumerate();
    
    for (i, word) in words {
        if i > 0 {
            translated.push(' ');
        }
        
        if check_rule_1(word) {
            translated.push_str(word);
        } else if check_rule_4(word) {
            let first_y_idx = word.find("y").unwrap();
            let (cons, rest) = word.split_at(first_y_idx);
            translated.push_str(rest);
            translated.push_str(cons);
        } else if check_rule_3(word) {
            word
                .split_inclusive("qu")
                .collect::<Vec<&str>>()
                .iter()
                .rev()
                .for_each(|part| translated.push_str(part));
        } else if check_rule_2(word) {
            let first_vowel_idx = word.find(VOWELS).unwrap();
            let (cons, rest) = word.split_at(first_vowel_idx);
            translated.push_str(rest);
            translated.push_str(cons);
        }
        
        translated.push_str("ay");
    }
    
    translated
}