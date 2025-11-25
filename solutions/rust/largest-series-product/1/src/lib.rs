#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }

    if span == 0 {
        return Ok(1);
    }

    let mut product = 0;

    for chunk in string_digits.as_bytes().windows(span) {
        let mut running_product = 1;

        for byte in chunk {
            let ch = *byte as char;
            let digit = ch.to_digit(10).ok_or(Error::InvalidDigit(ch))?;
            running_product *= digit;
        }

        if running_product > product {
            product = running_product;
        }
    }

    Ok(product as u64)
}