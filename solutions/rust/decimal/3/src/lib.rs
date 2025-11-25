use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::iter;
use std::ops::{Add, Mul, Sub};

/// Pad two decimal values with zeroes so that both have
/// the same amount of digits both in the integer and
/// in the fractional part.
fn align(left: &str, right: &str) -> (String, String) {
    let (left_i, left_f) = left.split_once(".").unwrap();
    let (right_i, right_f) = right.split_once(".").unwrap();

    let width_i = left_i.len().max(right_i.len());
    let width_f = left_f.len().max(right_f.len());

    let left = format!(
        "{left_i:0>width_i$}.{left_f:0<width_f$}",
        width_i = width_i,
        width_f = width_f
    );
    let right = format!(
        "{right_i:0>width_i$}.{right_f:0<width_f$}",
        width_i = width_i,
        width_f = width_f
    );

    (left, right)
}

#[derive(Debug)]
pub struct Decimal {
    negative: bool,
    integer: String,
    fractional: String,
}

impl Display for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}.{}",
            if self.negative { "-" } else { "" },
            self.integer,
            self.fractional
        )
    }
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let mut negative = true;

        let input = input.strip_prefix('-').unwrap_or_else(|| {
            negative = false;
            input.strip_prefix('+').unwrap_or(input)
        });

        input
            .split_once('.')
            .map(|(integer, fractional)| {
                // Compress zeroes to decrease memory consumption and simplify comparison.
                (
                    integer.trim_start_matches(|c| c == '0' && integer.len() > 1),
                    fractional.trim_end_matches(|c| c == '0' && fractional.len() > 1),
                )
            })
            .map(|(integer, fractional)| Decimal {
                negative,
                integer: integer.to_string(),
                fractional: fractional.to_string(),
            })
            .or_else(|| {
                Some(Decimal {
                    negative,
                    integer: input
                        .trim_start_matches(|c| c == '0' && input.len() > 1)
                        .to_string(),
                    fractional: String::from("0"),
                })
            })
    }

    fn is_zero(&self) -> bool {
        self.integer == "0" && self.fractional == "0"
    }

    fn abs(&self) -> Decimal {
        Decimal {
            negative: false,
            integer: self.integer.clone(),
            fractional: self.fractional.clone(),
        }
    }
}

impl Default for Decimal {
    fn default() -> Self {
        Decimal {
            negative: false,
            integer: "0".to_string(),
            fractional: "0".to_string(),
        }
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other).is_some_and(|v| v.is_eq())
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // 1. If both decimals are zero (no matter the sign), they are equal.
        if self.is_zero() && other.is_zero() {
            return Some(Ordering::Equal);
        }

        // 2. A negative decimal is always less than a positive.
        if self.negative && !other.negative {
            return Some(Ordering::Less);
        }

        if !self.negative && other.negative {
            return Some(Ordering::Greater);
        }

        let (lhs, rhs) = align(&self.abs().to_string(), &other.abs().to_string());

        lhs.partial_cmp(&rhs).map(|o| {
            if self.negative && other.negative {
                o.reverse()
            } else {
                o
            }
        })
    }
}

impl Add for Decimal {
    type Output = Decimal;

    fn add(self, rhs: Self) -> Self::Output {
        // Adding two zeroes results in a zero.
        if self.is_zero() && rhs.is_zero() {
            return Decimal::default();
        }

        let mut negative = false;

        // If both decimals are negative, the result is negative too.
        if self.negative && rhs.negative {
            negative = true
        // If only one of the decimals is negative:
        // 1. Find the larger absolute decimal.
        // 2. Perform subtraction of the absolutes.
        // 3. Assign the sign of the larger one to the result.
        } else if self.negative || rhs.negative {
            let self_abs = self.abs();
            let rhs_abs = rhs.abs();

            return if self_abs > rhs_abs {
                let mut result = self_abs - rhs_abs;
                result.negative = self.negative;
                result
            } else {
                let mut result = rhs_abs - self_abs;
                result.negative = rhs.negative;
                result
            };
        }

        let (lhs, rhs) = align(&self.abs().to_string(), &rhs.abs().to_string());

        let mut carry = false;
        // Add decimals together digit by digit, starting from the right.
        // Basically performing the column addition.
        let results = iter::zip(lhs.chars().rev(), rhs.chars().rev())
            .map(|(l, r)| {
                if l == '.' {
                    '.'
                } else {
                    let mut result = l.to_digit(10).unwrap() + r.to_digit(10).unwrap();

                    if carry {
                        result += 1;
                        carry = false
                    }

                    if result >= 10 {
                        result %= 10;
                        carry = true
                    }

                    char::from_digit(result, 10).unwrap()
                }
            })
            .collect::<Vec<_>>();

        Decimal::try_from(&format!(
            "{}{}",
            if negative { "-" } else { "" },
            results.iter().rev().collect::<String>()
        ))
        .expect("Should be a valid decimal number.")
    }
}

impl Sub for Decimal {
    type Output = Decimal;

    fn sub(self, rhs: Self) -> Self::Output {
        // Subtracting two zeroes results in a zero.
        if self.is_zero() && rhs.is_zero() {
            return Decimal::default();
        }

        let mut negative = false;

        // If both decimals are negative, the result is negative too.
        if self.negative && rhs.negative {
            negative = true;
        // If only one of the decimals is negative:
        // 1. Perform addition of the decimals' absolutes.
        // 2. Assign the sign of the left-hand side one to the result.
        } else if self.negative || rhs.negative {
            let mut result = self.abs() + rhs.abs();
            result.negative = self.negative;
            return result;
            // If the right-hand side is bigger than the left-hand side, reverse the operation.
        } else if rhs > self {
            let mut result = rhs - self;
            result.negative = true;
            return result;
        }

        let (lhs, rhs) = align(&self.abs().to_string(), &rhs.abs().to_string());

        let mut carry = false;
        // Subtract decimals digit by digit, starting from the right.
        // Basically performing the column subtraction.
        let results = iter::zip(lhs.chars().rev(), rhs.chars().rev())
            .map(|(l, r)| {
                if l == '.' {
                    '.'
                } else {
                    let mut result = l.to_digit(10).unwrap() as i8 - r.to_digit(10).unwrap() as i8;

                    if carry {
                        result -= 1;
                        carry = false
                    }

                    if result < 0 {
                        result += 10;
                        carry = true
                    }

                    char::from_digit(result as u32, 10).unwrap()
                }
            })
            .collect::<Vec<_>>();

        Decimal::try_from(&format!(
            "{}{}",
            if negative { "-" } else { "" },
            results.iter().rev().collect::<String>()
        ))
        .expect("Should be a valid decimal number.")
    }
}

impl Mul for Decimal {
    type Output = Decimal;

    fn mul(self, rhs: Self) -> Self::Output {
        // Multiplying two zeroes results in a zero.
        if self.is_zero() && rhs.is_zero() {
            return Decimal::default();
        }

        let mut negative = false;

        // Multiplying two negative decimals results in a positive decimal.
        if (self.negative || rhs.negative) && !(self.negative && rhs.negative) {
            negative = true
        }

        let (lhs, rhs_) = (
            self.abs()
                .to_string()
                .split("")
                .flat_map(|c| c.parse::<u8>())
                .collect::<Vec<_>>(),
            rhs.abs()
                .to_string()
                .split("")
                .flat_map(|c| c.parse::<u8>())
                .collect::<Vec<_>>(),
        );

        // Perform long multiplication.
        let mut product = vec![0; lhs.len() + rhs_.len()];

        for (i, r) in rhs_.iter().enumerate().rev() {
            let mut carry = 0;

            for (j, l) in lhs.iter().enumerate().rev() {
                product[i + j + 1] += carry + l * r;
                carry = product[i + j + 1] / 10;
                product[i + j + 1] %= 10;
            }

            product[i] = carry
        }

        let mut result = product
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>()
            .join("");

        result.insert(
            result.len() - (self.fractional.len() + rhs.fractional.len()),
            '.',
        );

        Decimal::try_from(&format!("{}{result}", if negative { "-" } else { "" }))
            .expect("Should be a valid decimal number.")
    }
}