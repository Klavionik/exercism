const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn is_consonant(c: &char) -> bool {
    !VOWELS.contains(c)
}

fn check_rule_1(input: &str) -> bool {
    input.starts_with(VOWELS) || input.starts_with("xr") || input.starts_with("yt")
}

fn check_rule_2(input: &str) -> Option<usize> {
    input.find(VOWELS)
}

fn check_rule_3(input: &str) -> Option<usize> {
    input.find("qu").and_then(|idx| {
        input[..idx]
            .chars()
            .all(|letter| is_consonant(&letter))
            .then_some(idx + 2) // Account for the length of "qu".
    })
}

fn check_rule_4(input: &str) -> Option<usize> {
    input.find("y").and_then(|idx| {
        // Check that there are consonants before "y".
        (idx > 0 && input[..idx].chars().all(|letter| is_consonant(&letter))).then_some(idx)
    })
}

pub fn translate(input: &str) -> String {
    let mut translated = vec![];

    for word in input.split_whitespace() {
        if check_rule_1(word) {
            translated.push(format!("{word}ay"))
        } else if let Some(n) = check_rule_4(word)
            .or(check_rule_3(word))
            .or(check_rule_2(word))
        {
            let (cons, rest) = word.split_at(n);
            translated.push(format!("{rest}{cons}ay"))
        }
    }

    translated.join(" ")
}