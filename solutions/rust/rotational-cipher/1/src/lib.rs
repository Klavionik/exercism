const ASCII_UPPER_MAX: u8 = 91;
const ASCII_LOWER_MAX: u8 = 123;
const ASCII_UPPER_MIN: u8 = 65;
const ASCII_LOWER_MIN: u8 = 97;

pub fn rotate(input: &str, key: u8) -> String {
    input.chars().map(|c| {
        if !c.is_alphabetic() {
            c
        } else {
            let ascii_index = c as u8;
            let mut shifted_ascii_index = ascii_index + key;

            if c.is_ascii_lowercase() && shifted_ascii_index >= ASCII_LOWER_MAX {
                shifted_ascii_index = shifted_ascii_index % ASCII_LOWER_MAX + ASCII_LOWER_MIN;
            }

            if c.is_ascii_uppercase() && shifted_ascii_index >= ASCII_UPPER_MAX {
                shifted_ascii_index = shifted_ascii_index % ASCII_UPPER_MAX + ASCII_UPPER_MIN;
            }

            shifted_ascii_index as char
        }
    }).collect()
}