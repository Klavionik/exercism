pub struct Luhn {
    input: String
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let code = self.input.replace(" ", "");

        if code.len() < 2 {
            return false
        }

        let mut checksum = 0;

        for (i, ch) in code.chars().rev().enumerate() {
            if !ch.is_ascii_digit() {
                return false
            }
            let digit = ch.to_digit(10).expect("A decimal digit.");

            if i % 2 != 0 {
                let mut doubled_digit = digit * 2;

                if doubled_digit > 9 {
                    doubled_digit -= 9;
                }

                checksum += doubled_digit;
            } else {
                checksum += digit;
            }

        }

        checksum % 10 == 0
    }
}

impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Self { input: input.to_string() }
    }
}