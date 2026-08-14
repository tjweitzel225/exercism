use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(Debug, PartialEq, Eq)]
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
        match input.as_bytes() {
            [b':', body @ .., b';'] => {
                let (name, implementation) = str::from_utf8(body)
                    .expect("text should be utf8")
                    .trim()
                    .split_once(" ")
                    .ok_or(Error::InvalidWord)?;
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
            }
            _ => {
                for token in self.parse(&input)? {
                    self.eval_token(token)?
                }
            }
        }
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
                        "+" => Some(Token::Op(BinOp::Plus)),
                        "-" => Some(Token::Op(BinOp::Minus)),
                        "*" => Some(Token::Op(BinOp::Multiply)),
                        "/" => Some(Token::Op(BinOp::Divide)),
                        "dup" | "swap" | "drop" | "over" => Some(Token::Cmd(s.parse().ok()?)),
                        _ if let Ok(num) = s.parse() => Some(Token::Num(num)),
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

#[derive(Clone, Debug)]
enum Token {
    Num(Value),
    Word(String),
    Op(BinOp),
    Cmd(StackOp),
}

#[derive(Clone, Debug)]
enum BinOp {
    Plus,
    Minus,
    Divide,
    Multiply,
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum StackOp {
    Dup,
    Drop,
    Swap,
    Over,
}
impl BinOp {
    pub fn eval(self, stack: &mut Vec<Value>) -> Result {
        match self {
            BinOp::Plus => {
                let (a, b) = stack.pop().zip(stack.pop()).ok_or(Error::StackUnderflow)?;
                stack.push(a + b);
            }
            BinOp::Minus => {
                let (a, b) = stack.pop().zip(stack.pop()).ok_or(Error::StackUnderflow)?;
                stack.push(b - a);
            }
            BinOp::Divide => {
                let (a, b) = stack.pop().zip(stack.pop()).ok_or(Error::StackUnderflow)?;
                stack.push(b.checked_div(a).ok_or(Error::DivisionByZero)?);
            }
            BinOp::Multiply => {
                let (a, b) = stack.pop().zip(stack.pop()).ok_or(Error::StackUnderflow)?;
                stack.push(a * b);
            }
        }
        Ok(())
    }
}
impl StackOp {
    pub fn eval(self, stack: &mut Vec<Value>) -> Result {
        match self {
            StackOp::Dup => stack.push(*stack.last().ok_or(Error::StackUnderflow)?),
            StackOp::Drop => {
                stack.pop().ok_or(Error::StackUnderflow)?;
            }
            StackOp::Swap => {
                let (a, b) = stack.pop().zip(stack.pop()).ok_or(Error::StackUnderflow)?;
                stack.push(a);
                stack.push(b);
            }
            StackOp::Over => {
                let second_last = stack
                    .get(stack.len().checked_sub(2).ok_or(Error::StackUnderflow)?)
                    .unwrap();
                stack.push(*second_last);
            }
        }
        Ok(())
    }
}

impl std::str::FromStr for StackOp {
    type Err = ();
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        for (word, variant) in [
            ("dup", StackOp::Dup),
            ("drop", StackOp::Drop),
            ("swap", StackOp::Swap),
            ("over", StackOp::Over),
        ] {
            if s == word {
                return Ok(variant);
            }
        }
        Err(())
    }
}
