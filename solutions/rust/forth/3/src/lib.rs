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
    words: HashMap<String, String>,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: Vec::new(),
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
                for t in input.split_whitespace().map(|w| Token::new(&self.words, w)) {
                    t.ok_or(Error::UnknownWord)?
                        .eval(&self.words, &mut self.stack)?
                }
                Ok(())
            }
        }
    }

    fn register_word_def(&mut self, def: &str) -> Result {
        let (name, implementation) = def.trim().split_once(" ").ok_or(Error::InvalidWord)?;
        if name.parse::<i32>().is_ok() {
            return Err(Error::InvalidWord);
        }
        let mut out: Vec<&str> = implementation.split_whitespace().collect();
        let mut idx = 0;
        while idx < out.len() {
            match self.words.get(out[idx]) {
                Some(existing_impl) => {
                    let expansion: Vec<&str> = existing_impl.split_whitespace().collect();
                    let len = expansion.len();
                    out.splice(idx..=idx, expansion);
                    idx += len;
                }
                None => idx += 1,
            }
        }
        self.words.insert(name.to_string(), out.join(" "));
        Ok(())
    }
}
