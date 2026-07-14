#[derive(Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
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
        }
    }
}

type CalcState = (Option<Op>, Option<i32>);
fn process_term(state: CalcState, s: &str) -> Option<CalcState> {
    Some(match state {
        // Nothing loaded yet, first s must be initial value.
        (op, None) => (op, s.parse().ok()),
        // There's a loaded n, the next s better be an Op.
        (None, n) => {
            let next_op = s.parse().ok()?;
            (Some(next_op), n)
        }
        // Both op and n are loaded -- next s must be another val to combine with n.
        (Some(op), Some(n)) => {
            let next_n = op.calc(n, s.parse().ok()?);
            (None, Some(next_n))
        }
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
