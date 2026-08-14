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
                for token in self.parse(&input)? {
                    self.eval_token(token)?
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

    fn parse(&self, input: &str) -> std::result::Result<Vec<Token>, Error> {
        input
            .split_whitespace()
            .map(|s: &str| {
                self.words
                    .contains_key(s)
                    .then(|| Token::Word(s.to_string()))
                    .or_else(|| match s {
                        "+" | "-" | "*" | "/" => Some(Token::Op(s.parse().ok()?)),
                        "dup" | "swap" | "drop" | "over" => Some(Token::Cmd(s.parse().ok()?)),
                        _ if let Ok(val) = s.parse::<Value>() => Some(Token::Num(val)),
                        _ => None,
                    })
                    .ok_or(Error::UnknownWord)
            })
            .collect()
    }

    fn eval_token(&mut self, t: Token) -> Result {
        let stack = &mut self.stack;
        match t {
            Token::Word(word) => {
                let word_tokens = self.words.get(&word).cloned().ok_or(Error::UnknownWord)?;
                for t in self.parse(&word_tokens)? {
                    self.eval_token(t)?
                }
            }
            Token::Num(n) => stack.push(n),
            Token::Cmd(cmd) => cmd.eval(stack)?,
            Token::Op(bin_op) => bin_op.eval(stack)?,
        };
        Ok(())
    }
}
