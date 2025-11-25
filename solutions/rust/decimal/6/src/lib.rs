use std::cmp::Ordering;
use std::collections::VecDeque;
use std::iter;
use std::ops::{Add, Mul, Sub};

fn align(lhs: &Decimal, rhs: &Decimal) -> (Vec<u8>, Vec<u8>, usize) {
    let len_i = lhs.integer().len().max(rhs.integer().len());
    let len_f = lhs.fractional().len().max(rhs.fractional().len());

    let lhs_i = [&vec![0; len_i - lhs.integer().len()], lhs.integer()].concat();
    let lhs_f = [lhs.fractional(), &vec![0; len_f - lhs.fractional().len()]].concat();
    let rhs_i = [&vec![0; len_i - rhs.integer().len()], rhs.integer()].concat();
    let rhs_f = [rhs.fractional(), &vec![0; len_f - rhs.fractional().len()]].concat();

    ([lhs_i, lhs_f].concat(), [rhs_i, rhs_f].concat(), len_f)
}

fn num_string_to_vec(string: &str) -> Vec<u8> {
    string
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

#[derive(Debug)]
pub struct Decimal {
    negative: bool,
    decimal_idx: usize,
    digits: Vec<u8>
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let mut negative = true;

        let input = input.strip_prefix('-').unwrap_or_else(|| {
            negative = false;
            input.strip_prefix('+').unwrap_or(input)
        });

        input
            .find('.')
            .map(|index| Decimal {
                negative,
                decimal_idx: index,
                digits: [
                    num_string_to_vec(&input[..index]),
                    num_string_to_vec(&input[index + 1..])
                ].concat(),
            })
            .or_else(|| {
                Some(Decimal {
                    negative,
                    decimal_idx: input.len(),
                    digits: [num_string_to_vec(input), vec![0]].concat(),
                })
            })
    }

    fn is_zero(&self) -> bool {
        self.digits.iter().all(|d| *d == 0)
    }

    fn abs(&self) -> Decimal {
        Decimal {
            negative: false,
            decimal_idx: self.decimal_idx,
            digits: self.digits.clone()
        }
    }

    fn integer(&self) -> &[u8] {
        &self.digits[..self.decimal_idx]
    }

    fn fractional(&self) -> &[u8] {
        &self.digits[self.decimal_idx..]
    }
}

impl Default for Decimal {
    fn default() -> Self {
        Decimal {
            negative: false,
            decimal_idx: 1,
            digits: vec![0, 0]
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

        let (lhs, rhs, _) = align(self, other);

        lhs.partial_cmp(&rhs).map(|o| {
            if self.negative && other.negative { o.reverse() } else { o }
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

        let (lhs, rhs, len_f) = align(&self, &rhs);

        // Add decimals together digit by digit, starting from the right.
        // Basically performing column addition.
        let mut carry = 0;
        let mut results = VecDeque::new();

        for (idx, (l, r)) in iter::zip(lhs.iter().rev(), rhs.iter().rev()).enumerate() {
            let mut result = l + r + carry;
            carry = result / 10;
            result %= 10;

            if idx == len_f {
                results.push_front('.')
            }

            results.push_front(char::from_digit(result as u32, 10).unwrap())
        }

        Decimal::try_from(&format!(
            "{}{}",
            if negative { "-" } else { "" },
            results.into_iter().collect::<String>()
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

        let (lhs, rhs, len_f) = align(&self, &rhs);

        // Subtract decimals digit by digit, starting from the right.
        // Basically performing column subtraction.
        let mut carry = 0;
        let mut results = VecDeque::new();

        for (idx, (l, r)) in iter::zip(lhs.iter().rev(), rhs.iter().rev()).enumerate() {
            let mut result = *l as i8 - *r as i8 + carry;
            carry = result.div_euclid(10);
            result = result + 10 * ((result < 0) as i8);

            if idx == len_f {
                results.push_front('.')
            }

            results.push_front(char::from_digit(result as u32, 10).unwrap())
        }

        Decimal::try_from(&format!(
            "{}{}",
            if negative { "-" } else { "" },
            results.into_iter().collect::<String>()
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

        // Perform long multiplication.
        let mut product = vec![0; self.digits.len() + rhs.digits.len()];

        for (i, r) in rhs.digits.iter().enumerate().rev() {
            let mut carry = 0;

            for (j, l) in self.digits.iter().enumerate().rev() {
                product[i + j + 1] += l * r + carry;
                carry = product[i + j + 1] / 10;
                product[i + j + 1] %= 10;
            }

            product[i] = carry
        }

        let mut result = String::new();

        for (idx, n) in product.iter().enumerate() {
            if idx == product.len() - (self.fractional().len() + rhs.fractional().len()) {
                result.push('.')
            }

            result.push_str(&n.to_string())
        }

        Decimal::try_from(&format!("{}{result}", if negative { "-" } else { "" }))
            .expect("Should be a valid decimal number.")
    }
}