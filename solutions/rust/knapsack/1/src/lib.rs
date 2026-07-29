#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    match items {
        [] => 0,
        [head, rest @ ..] => {
            let max_if_skip = maximum_value(max_weight, rest);
            if head.weight <= max_weight {
                let max_if_take = head.value + maximum_value(max_weight - head.weight, rest);
                max_if_take.max(max_if_skip)
            } else {
                max_if_skip
            }
        }
    }
}
