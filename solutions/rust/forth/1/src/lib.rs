pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

fn swap(stack: &mut [Value]) -> Result {
    let stack_len = stack.len();

    if stack_len < 2 {
        return Err(Error::StackUnderflow);
    };

    stack.swap(stack_len - 1, stack_len - 2);

    Ok(())
}

fn dup(stack: &mut Vec<Value>) -> Result {
    let stack_len = stack.len();

    if stack_len < 1 {
        return Err(Error::StackUnderflow);
    };

    stack.push(stack[stack_len - 1]);

    Ok(())
}

fn drop(stack: &mut Vec<Value>) -> Result {
    let stack_len = stack.len();

    if stack_len < 1 {
        return Err(Error::StackUnderflow);
    };

    stack.remove(stack_len - 1);

    Ok(())
}

fn over(stack: &mut Vec<Value>) -> Result {
    let stack_len = stack.len();

    if stack_len < 2 {
        return Err(Error::StackUnderflow);
    };

    stack.push(stack[stack_len - 2]);

    Ok(())
}

fn add(stack: &mut Vec<Value>) -> Result {
    let stack_len = stack.len();

    if stack_len < 2 {
        return Err(Error::StackUnderflow);
    };

    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    stack.push(a + b);

    Ok(())
}

fn sub(stack: &mut Vec<Value>) -> Result {
    let stack_len = stack.len();

    if stack_len < 2 {
        return Err(Error::StackUnderflow);
    };

    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    stack.push(b - a);

    Ok(())
}

fn mul(stack: &mut Vec<Value>) -> Result {
    let stack_len = stack.len();

    if stack_len < 2 {
        return Err(Error::StackUnderflow);
    };

    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    stack.push(a * b);

    Ok(())
}

fn div(stack: &mut Vec<Value>) -> Result {
    let stack_len = stack.len();

    if stack_len < 2 {
        return Err(Error::StackUnderflow);
    };

    let a = stack.pop().unwrap();
    let b = stack.pop().unwrap();

    if a == 0 {
        return Err(Error::DivisionByZero);
    }

    stack.push(b / a);

    Ok(())
}

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

    fn eval_new_definition(&mut self, definition: &str) -> Result {
        let definition = &definition[2..definition.len() - 2];

        if let Some((name, body)) = definition.split_once(" ") {
            if name.parse::<i32>().is_ok() {
                return Err(Error::InvalidWord);
            }

            let resolved = self.substitute_symbols(body);
            self.dictionary.push(Word::new(self.dictionary.len(), name, &resolved))
        }

        Ok(())
    }

    fn substitute_symbols(&self, definition: &str) -> String {
        let mut output = String::new();

        for token in definition.split_whitespace() {
            let maybe_word = self.dictionary.iter().find(|word| word.name == token);
            
            if let Some(word) = maybe_word {
                output.push_str(&format!("${}", &word.id.to_string()))
            } else {
                output.push_str(token)
            }

            output.push(' ');
        }

        output
    }
    
    fn expand_symbols(&self, definition: &str) -> String {
        let mut output = String::new();
        
        for token in definition.split_whitespace() {
            if token.starts_with("$") {
                let id = token.strip_prefix("$").unwrap().parse::<usize>().unwrap();
                let word = &self.dictionary[id];

                let expanded = self.expand_symbols(&word.body);
                output.push_str(&expanded)
            } else {
                output.push_str(token)
            }

            output.push(' ');
        }
        
        output
    }

    fn resolve_user_defined_words(&self, input: &str) -> String {
        let mut output = String::new();

        for token in input.split_whitespace() {
            if let Some(user_defined_word) = self.dictionary.iter().rfind(|word| word.name == token) {
                let expanded = self.expand_symbols(&user_defined_word.body);
                output.push_str(&expanded);
            } else {
                output.push_str(token);
            }

            output.push(' ');
        }

        output
    }

    fn _eval(&mut self, input: &str) -> Result {
        let input = self.resolve_user_defined_words(input);

        for token in input.split_whitespace() {
            match token {
                "swap" => swap(&mut self.stack)?,
                "dup" => dup(&mut self.stack)?,
                "drop" => drop(&mut self.stack)?,
                "over" => over(&mut self.stack)?,
                "+" => add(&mut self.stack)?,
                "-" => sub(&mut self.stack)?,
                "*" => mul(&mut self.stack)?,
                "/" => div(&mut self.stack)?,
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
}
