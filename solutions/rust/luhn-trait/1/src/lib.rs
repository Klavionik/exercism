pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}


impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let code = self.to_string().replace(" ", "");

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