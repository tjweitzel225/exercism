use std::collections::HashMap;

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

/// A `Word` holds an index into `Forth::definitions` rather than a name, so a
/// definition keeps referring to the version of a word that existed when it was
/// written even if that name is later redefined.
#[derive(Clone, Copy, Debug)]
enum Token {
    Num(Value),
    Word(usize),
    Op(BinOp),
    Cmd(StackOp),
}

impl Token {
    fn new(words: &HashMap<String, usize>, s: &str) -> Option<Self> {
        match words.get(s) {
            Some(&idx) => Some(Token::Word(idx)),
            None => match s {
                "+" | "-" | "*" | "/" => Some(Token::Op(s.parse().ok()?)),
                "dup" | "swap" | "drop" | "over" => Some(Token::Cmd(s.parse().ok()?)),
                _ if let Ok(val) = s.parse::<Value>() => Some(Token::Num(val)),
                _ => None,
            },
        }
    }
    fn eval(&self, definitions: &[Vec<Token>], stack: &mut Vec<Value>) -> Result {
        match *self {
            Token::Word(idx) => {
                for t in &definitions[idx] {
                    t.eval(definitions, stack)?
                }
            }
            Token::Num(n) => stack.push(n),
            Token::Cmd(cmd) => cmd.eval(stack)?,
            Token::Op(bin_op) => bin_op.eval(stack)?,
        };
        Ok(())
    }
}
#[derive(Clone, Copy, Debug)]
enum BinOp {
    Plus,
    Minus,
    Divide,
    Multiply,
}
impl BinOp {
    fn eval(self, stack: &mut Vec<Value>) -> Result {
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
impl std::str::FromStr for BinOp {
    type Err = ();
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "+" => Ok(BinOp::Plus),
            "-" => Ok(BinOp::Minus),
            "*" => Ok(BinOp::Multiply),
            "/" => Ok(BinOp::Divide),
            _ => Err(()),
        }
    }
}
#[derive(Clone, Copy, Debug)]
enum StackOp {
    Dup,
    Drop,
    Swap,
    Over,
}
impl StackOp {
    fn eval(self, stack: &mut Vec<Value>) -> Result {
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
        match s {
            "dup" => Ok(StackOp::Dup),
            "drop" => Ok(StackOp::Drop),
            "swap" => Ok(StackOp::Swap),
            "over" => Ok(StackOp::Over),
            _ => Err(()),
        }
    }
}
