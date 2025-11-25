use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::iter;
use std::ops::{Add, Mul, Sub};

fn align(left: &str, right: &str) -> (String, String) {
    let (left_i, left_f) = left.split_once(".").unwrap();
    let (right_i, right_f) = right.split_once(".").unwrap();

    let width_i = left_i.len().max(right_i.len());
    let width_f = left_f.len().max(right_f.len());

    let left = format!(
        "{:0>width_i$}.{:0<width_f$}",
        left_i,
        left_f,
        width_i = width_i,
        width_f = width_f
    );
    let right = format!(
        "{:0>width_i$}.{:0<width_f$}",
        right_i,
        right_f,
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
                // Compress zeroes to decrease memory consumption.
                (
                    integer
                        .chars()
                        .enumerate()
                        .skip_while(|&(idx, c)| c == '0' && idx != integer.len() - 1)
                        .map(|(_, c)| c)
                        .collect(),
                    fractional
                        .chars()
                        .rev()
                        .enumerate()
                        .skip_while(|&(idx, c)| c == '0' && idx != fractional.len() - 1)
                        .map(|(_, c)| c)
                        .collect::<Vec<_>>()
                        .iter()
                        .rev()
                        .collect(),
                )
            })
            .map(|(integer, fractional)| Decimal {
                negative,
                integer,
                fractional,
            })
            .or_else(|| {
                Some(Decimal {
                    negative,
                    integer: input
                        .chars()
                        .enumerate()
                        .skip_while(|&(idx, c)| idx != input.len() - 1 && c == '0')
                        .map(|(_, c)| c)
                        .collect(),
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
        // A few special cases.
        // 1. If both decimals are zero (no matter the sign), they are equal.
        if self.is_zero() && other.is_zero() {
            return Some(Ordering::Equal);
        }

        // 2. A negative decimal is always less than a positive.
        if self.negative && !other.negative {
            return Some(Ordering::Less);
        }

        // 2.1 A negative decimal is always less than a positive.
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
        // Short-circuit: adding two zeroes results in a zero.
        if self.is_zero() && rhs.is_zero() {
            return Decimal::default();
        }

        let mut negative = false;

        // If both decimals are negative, the result is negative too.
        if self.negative && rhs.negative {
            negative = true
        // If one of the decimals is negative:
        // 1. Find the larger absolute decimal.
        // 2. Perform subtraction of the absolutes.
        // 3. Assign the sign of the larger one to the result.
        } else if self.negative || rhs.negative {
            let self_abs = self.abs();
            let rhs_abs = rhs.abs();

            let result = if self_abs > rhs_abs {
                let mut result = self_abs - rhs_abs;
                result.negative = self.negative;
                result
            } else {
                let mut result = rhs_abs - self_abs;
                result.negative = rhs.negative;
                result
            };

            return result;
        }

        let (lhs, rhs) = align(&self.abs().to_string(), &rhs.abs().to_string());

        let mut carry = false;

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
        // Short-circuit: adding two zeroes results in a zero.
        if self.is_zero() && rhs.is_zero() {
            return Decimal::default();
        }

        let mut negative = false;

        // If both decimals are negative, the result is negative too.
        if self.negative && rhs.negative {
            negative = true;
        // If one of the decimals is negative:
        // 1. Perform addition of the decimals' absolutes.
        // 2. Assign the sign of the left-hand side one to the result.
        } else if self.negative || rhs.negative {
            let mut result = self.abs() + rhs.abs();
            result.negative = self.negative;

            return result;
            // If the right-hand side value is bigger than lhs, reverse the operation.
        } else if rhs > self {
            let mut result = rhs - self;
            result.negative = true;
            return result;
        }

        let (lhs, rhs) = align(&self.abs().to_string(), &rhs.abs().to_string());
        let mut carry = false;

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
        if self.is_zero() && rhs.is_zero() {
            return Decimal::default();
        }

        let mut negative = false;

        if (self.negative || rhs.negative) && !(self.negative && rhs.negative) {
            negative = true
        }

        let self_as_whole = self.abs().to_string();
        let rhs_as_whole = rhs.abs().to_string();

        let mut number = Decimal::default();
        let mut current_number = VecDeque::new();
        let mut carry = 0;

        for (rhs_idx, rhs_digit) in rhs_as_whole.chars().filter(|&c| c != '.').rev().enumerate() {
            for (self_idx, self_digit) in self_as_whole
                .chars()
                .filter(|&c| c != '.')
                .rev()
                .enumerate()
            {
                let mut result = self_digit.to_digit(10).unwrap() * rhs_digit.to_digit(10).unwrap();

                if carry > 0 {
                    result += carry;
                    carry = 0;
                }

                if self_idx != self_as_whole.len() - 2 && result >= 10 {
                    carry = result / 10;
                    result %= 10;
                }

                current_number.push_front(result.to_string())
            }

            current_number.push_back(vec!["0"; rhs_idx].join(""));
            number =
                number + Decimal::try_from(&current_number.make_contiguous().join("")).unwrap();
            current_number.clear();
        }

        let total_decimal_points = self.fractional.len() + rhs.fractional.len();

        let mut result_string = number.integer.clone();
        result_string.insert(result_string.len() - total_decimal_points, '.');

        Decimal::try_from(&format!(
            "{}{result_string}",
            if negative { "-" } else { "" }
        ))
        .expect("Should be a valid decimal number.")
    }
}