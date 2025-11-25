use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul, Sub};

enum Pad {
    Right,
    Left,
}

fn align(left: &str, right: &str, pad: Pad) -> (String, String) {
    if left.len() == right.len() {
        (left.to_owned(), right.to_owned())
    } else if left.len() > right.len() {
        let padding = vec!["0"; left.len() - right.len()].join("");

        match pad {
            Pad::Right => (
                left.to_owned(),
                String::from_iter([right.to_owned(), padding]),
            ),
            Pad::Left => (
                left.to_owned(),
                String::from_iter([padding, right.to_owned()]),
            ),
        }
    } else {
        let padding = vec!["0"; right.len() - left.len()].join("");

        match pad {
            Pad::Right => (
                String::from_iter([left.to_owned(), padding]),
                right.to_owned(),
            ),
            Pad::Left => (
                String::from_iter([padding, left.to_owned()]),
                right.to_owned(),
            ),
        }
    }
}

#[derive(Debug)]
pub struct Decimal {
    negative: bool,
    integer: String,
    fractional: String,
}

impl Display for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}.{}", if self.negative { "-" } else { "" }, self.integer, self.fractional)
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

        // If both decimals are negative, invert the comparison result.
        let mut invert = false;

        if self.negative && other.negative {
            invert = true
        }

        let (self_integer, other_integer) = align(&self.integer, &other.integer, Pad::Left);
        let (self_fractional, other_fractional) =
            align(&self.fractional, &other.fractional, Pad::Right);

        let it = self_integer
            .chars()
            .zip(other_integer.chars())
            .chain(self_fractional.chars().zip(other_fractional.chars()));

        for (self_digit, other_digit) in it {
            if self_digit > other_digit {
                return Some(if !invert {
                    Ordering::Greater
                } else {
                    Ordering::Less
                });
            }

            if self_digit < other_digit {
                return Some(if !invert {
                    Ordering::Less
                } else {
                    Ordering::Greater
                });
            }
        }

        Some(Ordering::Equal)
    }
}

fn column_parse(digits: (char, char)) -> (i8, i8) {
    (
        digits.0.to_digit(10).unwrap() as i8,
        digits.1.to_digit(10).unwrap() as i8,
    )
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

        let (self_integer, rhs_integer) = align(&self.integer, &rhs.integer, Pad::Left);
        let (self_fractional, rhs_fractional) =
            align(&self.fractional, &rhs.fractional, Pad::Right);

        let mut carry_integer = false;

        let mut integers = self_integer
            .chars()
            .rev()
            .zip(rhs_integer.chars().rev())
            .map(column_parse)
            .map(|(self_digit, rhs_digit)| {
                let mut result = self_digit + rhs_digit;

                if carry_integer {
                    result += 1;
                    carry_integer = false
                }

                if result >= 10 {
                    result %= 10;
                    carry_integer = true
                }

                result
            })
            .collect::<Vec<_>>();

        let mut carry_fractional = false;

        let fractionals = self_fractional
            .chars()
            .rev()
            .zip(rhs_fractional.chars().rev())
            .map(column_parse)
            .map(|(self_digit, rhs_digit)| {
                let mut result = self_digit + rhs_digit;

                if carry_fractional {
                    result += 1;
                    carry_fractional = false
                }

                if result >= 10 {
                    result %= 10;
                    carry_fractional = true
                }

                result
            })
            .collect::<Vec<_>>();

        integers[0] += carry_integer as i8 + carry_fractional as i8;

        let integer = integers.iter().rev().map(i8::to_string).collect::<String>();
        let fractional = fractionals
            .iter()
            .rev()
            .map(i8::to_string)
            .collect::<String>();

        Decimal::try_from(&format!(
            "{}{integer}.{fractional}",
            if negative { "-" } else { "" }
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
            let self_abs = self.abs();
            let rhs_abs = rhs.abs();

            let mut result = self_abs + rhs_abs;
            result.negative = self.negative;

            return result;
            // If the right-hand side value is bigger than lhs, reverse the operation.
        } else {
            if rhs > self {
                let mut result = rhs - self;
                result.negative = true;
                return result
            }
        }

        let (self_integer, rhs_integer) = align(&self.integer, &rhs.integer, Pad::Left);
        let (self_fractional, rhs_fractional) =
            align(&self.fractional, &rhs.fractional, Pad::Right);

        let mut carry_integer = false;

        let mut integers = self_integer
            .chars()
            .rev()
            .zip(rhs_integer.chars().rev())
            .map(column_parse)
            .map(|(self_digit, rhs_digit)| {
                let mut result = self_digit - rhs_digit;

                if carry_integer {
                    result -= 1;
                    carry_integer = false
                }

                if result < 0 {
                    result += 10;
                    carry_integer = true
                }

                result
            })
            .collect::<Vec<_>>();

        let mut carry_fractional = false;

        let fractionals = self_fractional
            .chars()
            .rev()
            .zip(rhs_fractional.chars().rev())
            .map(column_parse)
            .map(|(self_digit, rhs_digit)| {
                let mut result = self_digit - rhs_digit;

                if carry_fractional {
                    result -= 1;
                    carry_fractional = false
                }

                if result < 0 {
                    result += 10;
                    carry_fractional = true
                }

                result
            })
            .collect::<Vec<_>>();

        integers[0] -= carry_integer as i8 + carry_fractional as i8;

        let integer = integers.iter().rev().map(i8::to_string).collect::<String>();
        let fractional = fractionals.iter().rev().map(i8::to_string).collect::<String>();

        Decimal::try_from(&format!(
            "{}{integer}.{fractional}",
            if negative { "-" } else { "" }
        ))
        .expect("Should be a valid decimal number.")
    }
}

impl Mul for Decimal {
    type Output = Decimal;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.is_zero() && rhs.is_zero() {
            return Decimal::default()
        }

        let mut negative = false;

        if (self.negative || rhs.negative) && !(self.negative && rhs.negative) {
            negative = true
        }

        let self_as_whole = self.abs().to_string();
        let rhs_as_whole = rhs.abs().to_string();

        let mut numbers: Vec<Decimal> = vec![];
        let mut current_number = vec![];
        let mut carry = 0;

        for (rhs_idx, rhs_digit) in rhs_as_whole.chars().filter(|&c| c != '.').rev().enumerate() {
            for (self_idx, self_digit) in self_as_whole.chars().filter(|&c| c != '.').rev().enumerate() {
                let self_digit = self_digit.to_digit(10).unwrap();
                let rhs_digit = rhs_digit.to_digit(10).unwrap();

                let mut result = self_digit * rhs_digit;

                if carry > 0 {
                    result += carry;
                    carry = 0;
                }

                if self_idx != self_as_whole.len() - 2 && result >= 10 {
                    carry = result / 10;
                    result = result % 10;
                }

                 current_number.push(result.to_string())
            }

            current_number.reverse();

            current_number.push(vec!["0"; rhs_idx].join(""));
            numbers.push(Decimal::try_from(&current_number.join("")).unwrap());
            current_number.clear();
        }

        let result = numbers.into_iter().reduce(|acc, e| acc + e).unwrap();

        let self_decimal_points = self_as_whole.split_once('.').and_then(|(_, right)| Some(right.len())).unwrap();
        let rhs_decimal_points = rhs_as_whole.split_once('.').and_then(|(_, right)| Some(right.len())).unwrap();
        let total_decimal_points = self_decimal_points + rhs_decimal_points;

        let mut result_string = result.integer.clone();
        result_string.insert(result_string.len() - total_decimal_points, '.');

        Decimal::try_from(&format!(
            "{}{result_string}",
            if negative { "-" } else { "" }
        ))
            .expect("Should be a valid decimal number.")
    }
}