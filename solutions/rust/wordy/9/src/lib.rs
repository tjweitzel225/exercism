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
            Op::Exp => a.pow(b.try_into().expect("pow needs b to be positive.")),
        }
    }
}

type CalcState = (Option<Op>, Option<i32>);
fn process_term(state: CalcState, s: &str) -> Option<CalcState> {
    Some(match state {
        // No n yet, first s must be initial value.
        (_, None) => {
            let next_n = s.parse().ok()?;
            (None, Some(next_n))
        }
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
        .split_whitespace()
        .filter_map(|s| {
            (!matches!(s, "What" | "is" | "by" | "to" | "the" | "power")).then_some(
                s.trim_end_matches("th")
                    .trim_end_matches("nd")
                    .trim_end_matches("rd"),
            )
        })
        .try_fold((None, None), process_term)
        .and_then(|(op, n)| if op.is_none() { n } else { None })
}
