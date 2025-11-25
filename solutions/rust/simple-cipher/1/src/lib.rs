use rand::random_range;

const ASCII_A: u8 = 97;
const ASCII_Z: u8 = 122;

fn check_key(key: &str) -> bool {
    !key.is_empty() && !key.bytes().any(|c| {
        c.is_ascii_digit()
        || c.is_ascii_uppercase()
    })
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    if !check_key(key) {
        return None
    }

    Some(key.bytes().cycle().zip(s.bytes()).map(|(key, plaintext)| {
        let offset = key - ASCII_A;
        let mut encoded = plaintext + offset;

        if encoded > ASCII_Z {
            encoded = encoded % (ASCII_Z + 1) + ASCII_A;
        }

        encoded as char
    }).collect())
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if !check_key(key) {
        return None
    }

    Some(key.bytes().cycle().zip(s.bytes()).map(|(key, plaintext)| {
        let offset = key - ASCII_A;
        let mut decoded = plaintext - offset;

        if decoded < ASCII_A {
            decoded = (ASCII_Z + 1) - (ASCII_A - decoded);
        }

        decoded as char
    }).collect())
}

pub fn encode_random(s: &str) -> (String, String) {
    let key = (0..=100).map(|_| random_range('a'..='z')).collect::<String>();
    let encoded = encode(&key, s).unwrap();

    (key, encoded)
}