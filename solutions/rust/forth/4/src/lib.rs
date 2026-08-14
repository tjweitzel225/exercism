mod token;
use std::collections::HashMap;
use token::Token;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(PartialEq, Debug)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Debug)]
pub struct Forth {
    stack: Vec<Value>,
    definitions: Vec<Vec<Token>>,
    words: HashMap<String, usize>,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: Vec::new(),
            definitions: Vec::new(),
            words: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let input = input.to_ascii_lowercase();
        match input.strip_prefix(':').and_then(|s| s.strip_suffix(';')) {
            Some(word_def) => self.register_word_def(word_def),
            None => {
                for t in self.parse(&input)? {
                    t.eval(&self.definitions, &mut self.stack)?
                }
                Ok(())
            }
        }
    }

    fn parse(&self, input: &str) -> std::result::Result<Vec<Token>, Error> {
        input
            .split_whitespace()
            .map(|w| Token::new(&self.words, w).ok_or(Error::UnknownWord))
            .collect()
    }

    fn register_word_def(&mut self, def: &str) -> Result {
        let (name, implementation) = def.trim().split_once(" ").ok_or(Error::InvalidWord)?;
        if name.parse::<Value>().is_ok() {
            return Err(Error::InvalidWord);
        }
        let body = self.parse(implementation)?;
        self.definitions.push(body);
        self.words
            .insert(name.to_string(), self.definitions.len() - 1);
        Ok(())
    }
}
