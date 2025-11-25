pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.is_empty() {
        return false
    }

    let mut count = 1;
    let mut checksum = 0;

    for ch in isbn.chars().filter(|c| c != &'-').rev() {
        if count > 10 {
            return false
        }
        
        let digit = match ch.to_digit(10) {
            Some(digit) => digit,
            None if count == 1 && ch == 'X' => 10,
            _ => return false
        };

        checksum += digit * count;
        count += 1;
    }

    checksum > 0 && checksum % 11 == 0
}