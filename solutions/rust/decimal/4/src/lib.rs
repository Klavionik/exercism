use std::cmp::Ordering;
use std::collections::VecDeque;
use std::iter;
use std::ops::{Add, Mul, Sub};

fn align(lhs: &Decimal, rhs: &Decimal) -> (Vec<i8>, Vec<i8>, usize) {
    let width_i = lhs.integer.len().max(rhs.integer.len());
    let width_f = lhs.fractional.len().max(rhs.fractional.len());

    let lhs_i = [vec![0; width_i - lhs.integer.len()], lhs.integer.clone()].concat();
    let lhs_f = [
        lhs.fractional.clone(),
        vec![0; width_f - lhs.fractional.len()],
    ]
    .concat();
    let rhs_i = [vec![0; width_i - rhs.integer.len()], rhs.integer.clone()].concat();
    let rhs_f = [
        rhs.fractional.clone(),
        vec![0; width_f - rhs.fractional.len()],
    ]
    .concat();

    ([lhs_i, lhs_f].concat(), [rhs_i, rhs_f].concat(), width_f)
}

fn num_string_to_vec(string: &str) -> Vec<i8> {
    string
        .split("")
        .flat_map(|c| c.parse::<i8>())
        .collect::<Vec<_>>()
}

#[derive(Debug)]
pub struct Decimal {
    negative: bool,
    integer: Vec<i8>,
    fractional: Vec<i8>,
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
                integer: num_string_to_vec(integer),
                fractional: num_string_to_vec(fractional),
            })
            .or_else(|| {
                Some(Decimal {
                    negative,
                    integer: num_string_to_vec(
                        input.trim_start_matches(|c| c == '0' && input.len() > 1),
                    ),
                    fractional: vec![0],
                })
            })
    }

    fn is_zero(&self) -> bool {
        self.integer == vec![0] && self.fractional == vec![0]
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
            integer: vec![0],
            fractional: vec![0],
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

        let (lhs, rhs, _) = align(&self.abs(), &other.abs());

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

        let (lhs, rhs, width_f) = align(&self.abs(), &rhs.abs());

        // Add decimals together digit by digit, starting from the right.
        // Basically performing column addition.
        let mut carry = 0;
        let mut results = VecDeque::new();

        for (idx, (l, r)) in iter::zip(lhs.iter().rev(), rhs.iter().rev()).enumerate() {
            let mut result = l + r + carry;
            carry = result / 10;
            result %= 10;

            if idx == width_f {
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

        let (lhs, rhs, width_f) = align(&self.abs(), &rhs.abs());

        // Subtract decimals digit by digit, starting from the right.
        // Basically performing column subtraction.
        let mut carry = 0;
        let mut results = VecDeque::new();

        for (idx, (l, r)) in iter::zip(lhs.iter().rev(), rhs.iter().rev()).enumerate() {
            let mut result = *l - *r + carry;
            carry = result.div_euclid(10);
            result = result + 10 * ((result < 0) as i8);

            if idx == width_f {
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

        let lhs = [&self.integer[..], &self.fractional[..]].concat();
        let rhs_ = [&rhs.integer[..], &rhs.fractional[..]].concat();

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

        let mut result = String::new();

        for (idx, n) in product.iter().enumerate() {
            if idx == product.len() - (self.fractional.len() + rhs.fractional.len()) {
                result.push('.')
            }

            result.push(char::from_digit(*n as u32, 10).unwrap())
        }

        Decimal::try_from(&format!("{}{result}", if negative { "-" } else { "" }))
            .expect("Should be a valid decimal number.")
    }
}