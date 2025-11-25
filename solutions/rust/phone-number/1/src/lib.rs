pub fn number(user_number: &str) -> Option<String> {
    let only_digits = user_number
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<Vec<char>>();

    let normalized_phone = only_digits.strip_prefix(&['1']).unwrap_or(&only_digits);

    if normalized_phone.len() != 10
        || !('2'..='9').contains(&normalized_phone[0])
        || !('2'..='9').contains(&normalized_phone[3])
    {
        return None;
    }

    Some(String::from_iter(normalized_phone))
}
