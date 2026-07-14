#[derive(Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
}

struct ParseError;
impl std::str::FromStr for Op {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, ParseError> {
        match s {
            "plus" => Ok(Op::Add),
            "minus" => Ok(Op::Sub),
            "divided" => Ok(Op::Div),
            "multiplied" => Ok(Op::Mul),
            "raised" => Ok(Op::Exp),
            _ => Err(ParseError),
        }
    }
}
impl Op {
    fn calc(self, a: i32, b: i32) -> i32 {
        match self {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
            Op::Div => a / b,
            Op::Exp => a.pow(b.try_into().expect("pow needs b to be an integer.")),
        }
    }
}

enum State {
    Init,
    Value(i32),
    Operator(i32, Op),
}
impl State {
    fn step(self, s: &str) -> Option<State> {
        dbg!(s);
        Some(match self {
            State::Init => State::Value(s.parse().ok()?),
            State::Value(n) => State::Operator(n, s.parse().ok()?),
            State::Operator(n, op) => State::Value(op.calc(n, s.parse().ok()?)),
        })
    }
}

pub fn answer(command: &str) -> Option<i32> {
    let final_state = command
        .trim_end_matches('?')
        .split_whitespace()
        .filter_map(|s| {
            (!matches!(s, "What" | "is" | "by" | "to" | "the" | "power")).then_some(
                s.trim_end_matches("th")
                    .trim_end_matches("nd")
                    .trim_end_matches("rd")
                    .trim_end_matches("st")
                    .trim_end_matches("th"),
            )
        })
        .try_fold(State::Init, State::step)?;

    match final_state {
        State::Value(n) => Some(n),
        _ => None,
    }
}
