use std::fmt::Display;
use std::ops::{Rem};

pub struct Matcher<T: Display> {
    func: Box<dyn Fn(T) -> bool>,
    substitute: String
}

impl<T: Display + Copy> Matcher<T> {
    pub fn new<F: Fn(T) -> bool + 'static>(matcher: F, subs: &str) -> Matcher<T> {
        Self { func: Box::new(matcher), substitute: subs.to_string() }
    }
    
    pub fn matches(&self, item: T) -> Option<String> {
        if (self.func)(item) {
            return Some(self.substitute.clone())
        }
        
        None
    }
}

pub struct Fizzy<T: Display + Copy> {
    matchers: Vec<Matcher<T>>
}

impl<T: Display + Copy> Fizzy<T> {
    pub fn new() -> Self {
        Self { matchers: vec![] }
    }

    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    pub fn apply<I: IntoIterator<Item = T>>(self, iter: I) -> impl Iterator<Item = String> {
        iter.into_iter().map(move |el| {
            let mut result = String::new();
            
            for matcher in &self.matchers {
                if let Some(substitute) = matcher.matches(el) {
                    result.push_str(&substitute)
                }
            }
            
            if result.is_empty() {
                result.push_str(&el.to_string())
            }
            
            result
        })        
    }
}

pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: Display + Copy + Rem<Output = T> + PartialEq<T> + From<u8>,
{
    let fizzer = Fizzy::new();
    
    fizzer
        .add_matcher(Matcher::new(|n| n % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|n| n % 5.into() == 0.into(), "buzz"))
}