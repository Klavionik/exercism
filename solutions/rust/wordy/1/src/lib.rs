use regex_lite::{Match, Regex};
use std::collections::VecDeque;
use std::ops::{Add, Div, Mul, Sub};

type Op = fn(i32, i32) -> i32;
const VALIDATION_PATTERN: &str = r"(?x)
    ^What\sis\s                                                 # Match What is.
    (?P<exps>                                                   # Capture the expression.
        -?\d+                                                       # Capture the first number.
        (?:\s(?:plus|minus|multiplied\sby|divided\sby)\s-?\d+)*)    # Capture the putative operator and the next number multiple times.
    \?$                                                         # Match the question mark.
";

#[derive(Debug)]
enum Pattern {
    Number,
    Operator,
}

impl Pattern {
    fn regex(&self) -> Regex {
        let regex = match self {
            Pattern::Number => r"^-?\d+",
            Pattern::Operator => r"^(plus|minus|multiplied by|divided by)",
        };

        Regex::new(regex).expect("A valid regex.")
    }

    pub fn find<'a>(&self, source: &'a str) -> Option<Match<'a>> {
        self.regex().find(source)
    }
}

#[derive(Debug)]
enum Token {
    Number(i32),
    Operator(Op),
}

struct Parser<'a> {
    source: &'a str,
    offset: usize,
    next_pattern: Pattern,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Option<Self> {
        let regex = Regex::new(VALIDATION_PATTERN).expect("A valid regex.");

        regex
            .captures(source)
            .and_then(|captures| captures.name("exps"))
            .map(|exps| Self {
                source: exps.as_str(),
                offset: 0,
                next_pattern: Pattern::Number,
            })
    }
}

impl Iterator for Parser<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset > self.source.len() {
            return None;
        }

        let next_slice = &self.source[self.offset..];
        let maybe_match = self.next_pattern.find(next_slice);

        if let Some(match_) = maybe_match {
            // Implicitly step over the next whitespace character.
            self.offset += match_.end() + 1;
            let content = match_.as_str();

            let token = match self.next_pattern {
                Pattern::Number => {
                    self.next_pattern = Pattern::Operator;

                    let val = content.parse::<i32>().expect("A valid 32-bit number.");

                    Token::Number(val)
                }
                Pattern::Operator => {
                    self.next_pattern = Pattern::Number;

                    let op = match content {
                        "minus" => i32::sub,
                        "plus" => i32::add,
                        "divided by" => i32::div,
                        "multiplied by" => i32::mul,
                        &_ => panic!("Unexpected operator: {content}."),
                    };

                    Token::Operator(op)
                }
            };

            return Some(token);
        }

        None
    }
}

pub fn answer(command: &str) -> Option<i32> {
    let parser = Parser::new(command)?;
    let mut operators: VecDeque<Op> = VecDeque::new();
    let mut operands: Vec<i32> = vec![];

    for token in parser {
        match token {
            Token::Number(value) => {
                operands.push(value);
            }
            Token::Operator(func) => {
                operators.push_back(func);
            }
        }
    }

    operands.into_iter().reduce(|left, right| {
        let operator = operators
            .pop_front()
            .expect("For every two operands there must be an operator.");

        operator(left, right)
    })
}