#[derive(Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn parse(s: &str) -> Option<Self> {
        match s {
            "plus" => Some(Op::Add),
            "minus" => Some(Op::Sub),
            "divided" => Some(Op::Div),
            "multiplied" => Some(Op::Mul),
            _ => None,
        }
    }
    fn calc(self, a: i32, b: i32) -> i32 {
        match self {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
            Op::Div => a / b,
        }
    }
}

type CalcState = (Option<Op>, Option<i32>);
fn process_term(state: CalcState, s: &str) -> Option<CalcState> {
    Some(match state {
        (None, None) => (None, s.parse::<i32>().ok()),
        (Some(op), Some(n)) => (None, Some(op.calc(n, s.parse().ok()?))),
        (None, Some(n)) => (Some(Op::parse(s)?), Some(n)),
        _ => return None,
    })
}

pub fn answer(command: &str) -> Option<i32> {
    command
        .trim_end_matches('?')
        .replace("by ", "")
        .split_whitespace()
        .try_fold((None, None), process_term)
        .map(|(op, n)| if op.is_none() { n } else { None })?
}
