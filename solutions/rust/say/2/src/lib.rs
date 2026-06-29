const HIGHER_POWERS_OF_10: [&str; 6] = [
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];
pub fn encode(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }
    let chars: Vec<char> = n.to_string().chars().collect();
    let say: String = chars
        .rchunks(3)
        .map(|num| {
            let num: String = num.iter().collect();
            let said = say_0_to_999(num.parse().unwrap()).unwrap();
            if said == "zero" { "".to_string() } else { said }
        })
        .enumerate()
        .map(|(i, mut s)| {
            if i > 0 && !s.is_empty() {
                s.push(' ');
                s.push_str(HIGHER_POWERS_OF_10[i - 1]);
                s.push(' ');
                s
            } else {
                s
            }
        })
        .rev()
        .collect();
    say.trim().to_string()
}

fn say_0_to_19(n: u64) -> Option<&'static str> {
    (n < 20).then_some(match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        _ => unreachable!(),
    })
}

fn say_0_to_99(n: u64) -> Option<String> {
    (n < 100).then_some({
        if n < 20 {
            say_0_to_19(n)?.to_string()
        } else {
            let first_digit = match n / 10 {
                2 => "twenty",
                3 => "thirty",
                4 => "forty",
                5 => "fifty",
                6 => "sixty",
                7 => "seventy",
                8 => "eighty",
                9 => "ninety",
                _ => unreachable!(),
            };
            let second_digit = say_0_to_19(n % 10)?;
            if second_digit == "zero" {
                first_digit.to_string()
            } else {
                format!("{first_digit}-{second_digit}")
            }
        }
    })
}

fn say_0_to_999(n: u64) -> Option<String> {
    (n < 1000).then_some({
        if n < 100 {
            say_0_to_99(n)?
        } else {
            let first_digit = n / 100;
            let mut out = if first_digit == 0 {
                "".to_string()
            } else {
                format!("{} hundred ", say_0_to_19(first_digit)?)
            };
            let last_digits = n % 100;
            if last_digits != 0 {
                out.push_str(&say_0_to_99(last_digits)?);
            }
            out.trim().to_string()
        }
    })
}
