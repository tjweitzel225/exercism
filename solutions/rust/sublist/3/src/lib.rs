#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn contains_subslice(haystack: &[i32], needle: &[i32]) -> bool {
    needle.is_empty()
        || haystack
            .windows(needle.len())
            .any(|window| needle == window)
}
pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    } else if contains_subslice(second_list, first_list) {
        Comparison::Sublist
    } else if contains_subslice(first_list, second_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}
