pub fn number(user_number: &str) -> Option<String> {
    let only_digits = user_number
        .chars()
        .filter(char::is_ascii_digit)
        .skip_while(|&ch| ch == '1')
        .collect::<Vec<char>>();

    if only_digits.len() != 10
        || !('2'..='9').contains(&only_digits[0])
        || !('2'..='9').contains(&only_digits[3])
    {
        return None;
    }

    Some(String::from_iter(only_digits))
}