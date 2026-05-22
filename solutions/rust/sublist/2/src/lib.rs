use std::iter::zip;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    };
    let left_is_inside_right =
        |left: &[i32], right: &[i32]| right.windows(left.len()).find(|&win| left == win).is_some();
    match (first_list.len(), second_list.len()) {
        (_, 0) => Comparison::Superlist,
        (0, _) => Comparison::Sublist,
        (n, m) if n <= m => {
            if left_is_inside_right(first_list, second_list) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
        _ => {
            if left_is_inside_right(second_list, first_list) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
    }
}
