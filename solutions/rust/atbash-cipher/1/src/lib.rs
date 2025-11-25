fn atbash(input: &str) -> impl Iterator<Item = char> {
    input
        .bytes()
        .filter(u8::is_ascii_alphanumeric)
        .map(|c| {
            if c.is_ascii_digit() {
                return c as char;
            }

            let lower = c.to_ascii_lowercase();
            let converted = 25 - (lower - 97) + 97;
            converted as char
        })
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    atbash(plain)
        .collect::<Vec<_>>()
        .chunks(5)
        .collect::<Vec<_>>()
        .join(&' ')
        .iter()
        .collect()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    String::from_iter(atbash(cipher))
}