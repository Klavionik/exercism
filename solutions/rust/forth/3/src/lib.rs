pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

struct Word {
    id: usize,
    name: String,
    body: String
}

impl Word {
    pub fn new(id: usize, name: &str, body: &str) -> Self {
        Word {
            id,
            name: name.to_owned(),
            body: body.to_owned()
        }
    }

    fn symbol(&self) -> String {
        format!("${}", self.id)
    }
}

pub struct Forth {
    dictionary: Vec<Word>,
    stack: Vec<Value>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            dictionary: vec![],
            stack: vec![],
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let input = input.to_ascii_lowercase();

        if input.starts_with(":") {
            return self.eval_new_definition(&input);
        }

        self._eval(&input)
    }

    /// Add a new word to the dictionary.
    fn eval_new_definition(&mut self, definition: &str) -> Result {
        // Strip `: ` prefix and ` ;` suffix.
        let definition = &definition[2..definition.len() - 2];

        if let Some((name, body)) = definition.split_once(" ") {
            if name.parse::<i32>().is_ok() {
                return Err(Error::InvalidWord);
            }

            let body = self.insert_symbols(body);
            self.dictionary.push(Word::new(self.dictionary.len(), name, &body))
        }

        Ok(())
    }

    /// Find a word in the dictionary by its name. Returns the latest
    /// defined word, if it exists.
    fn find_word(&self, name: &str) -> Option<&Word> {
        self.dictionary.iter().rfind(|word| word.name == name)
    }

    /// Replace usages of other user-defined word with symbols
    /// (where symbol is $<dictionary id>).
    fn insert_symbols(&self, body: &str) -> String {
        let mut output = vec![];

        for token in body.split_whitespace() {
            let maybe_word = self.find_word(token);

            if let Some(word) = maybe_word {
                output.push(word.symbol())
            } else {
                output.push(token.to_owned())
            }
        }

        output.join(" ")
    }

    /// Recursively replace symbols representing other words in the input word's body
    /// with their corresponding bodies.
    fn expand_symbols(&self, body: &str) -> String {
        let mut output = vec![];

        for token in body.split_whitespace() {
            if token.starts_with("$") {
                let id = token.strip_prefix("$").unwrap().parse::<usize>().unwrap();
                let word = &self.dictionary[id];
                let expanded = self.expand_symbols(&word.body);

                output.push(expanded)
            } else {
                output.push(token.to_owned())
            }
        }

        output.join(" ")
    }

    /// Replace user-defined words in input with
    /// actual words' bodies before evaluation.
    fn resolve_user_defined_words(&self, input: &str) -> String {
        let mut output = vec![];

        for token in input.split_whitespace() {
            if let Some(user_defined_word) = self.find_word(token) {
                let expanded = self.expand_symbols(&user_defined_word.body);
                output.push(expanded);
            } else {
                output.push(token.to_owned());
            }
        }

        output.join(" ")
    }

    fn _eval(&mut self, input: &str) -> Result {
        let input = self.resolve_user_defined_words(input);

        for token in input.split_whitespace() {
            match token {
                "swap" => self.swap()?,
                "dup" => self.dup()?,
                "drop" => self.drop()?,
                "over" => self.over()?,
                "+" => self.add()?,
                "-" => self.sub()?,
                "*" => self.mul()?,
                "/" => self.div()?,
                &_ => {
                    if let Ok(num) = token.parse::<Value>() {
                        self.stack.push(num);
                    } else {
                        return Err(Error::UnknownWord);
                    }
                }
            }
        }

        Ok(())
    }

    fn swap(&mut self) -> Result {
        let stack_len = self.stack.len();

        if stack_len < 2 {
            return Err(Error::StackUnderflow);
        };

        self.stack.swap(stack_len - 1, stack_len - 2);

        Ok(())
    }

    fn dup(&mut self) -> Result {
        let stack_len = self.stack.len();

        if stack_len < 1 {
            return Err(Error::StackUnderflow);
        };

        self.stack.push(self.stack[stack_len - 1]);

        Ok(())
    }

    fn drop(&mut self) -> Result {
        let stack_len = self.stack.len();

        if stack_len < 1 {
            return Err(Error::StackUnderflow);
        };

        self.stack.remove(stack_len - 1);

        Ok(())
    }

    fn over(&mut self) -> Result {
        let stack_len = self.stack.len();

        if stack_len < 2 {
            return Err(Error::StackUnderflow);
        };

        self.stack.push(self.stack[stack_len - 2]);

        Ok(())
    }

    fn add(&mut self) -> Result {
        let stack_len = self.stack.len();

        if stack_len < 2 {
            return Err(Error::StackUnderflow);
        };

        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();

        self.stack.push(a + b);

        Ok(())
    }

    fn sub(&mut self) -> Result {
        let stack_len = self.stack.len();

        if stack_len < 2 {
            return Err(Error::StackUnderflow);
        };

        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();

        self.stack.push(a - b);

        Ok(())
    }

    fn mul(&mut self) -> Result {
        let stack_len = self.stack.len();

        if stack_len < 2 {
            return Err(Error::StackUnderflow);
        };

        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();

        self.stack.push(a * b);

        Ok(())
    }

    fn div(&mut self) -> Result {
        let stack_len = self.stack.len();

        if stack_len < 2 {
            return Err(Error::StackUnderflow);
        };

        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();

        if b == 0 {
            return Err(Error::DivisionByZero);
        }

        self.stack.push(a / b);

        Ok(())
    }
}