use std::collections::HashMap;
use std::iter::once;

#[derive(Debug)]
struct Column {
    lhs_letters: Vec<Option<u8>>,
    rhs_letter: u8,
}

#[derive(Debug)]
pub enum NoSolution {
    LeadingZero,
    RHSTooShort,
    CurrentColHasNoUnassignedLetters,
    OutOfColumnBounds,
    NoValidAssignment,
}

fn step(
    col_idx: usize,
    cols: &[Column],
    carry: usize,
    assignment: &mut HashMap<u8, u8>,
    unassigned_digits: &mut Vec<usize>,
    leading_letters: &[u8],
) -> Result<bool, NoSolution> {
    // dbg!(carry);
    if col_idx == cols.len() {
        return Ok(carry == 0);
    }
    let col = cols.get(col_idx).ok_or(NoSolution::OutOfColumnBounds)?;
    // dbg!((&assignment, &col));
    let lhs_values = col
        .lhs_letters
        .iter()
        .filter_map(|l| l.and_then(|ll| assignment.get(&ll)));
    let current_cols_unassigned_letters: Vec<&u8> = col
        .lhs_letters
        .iter()
        .flatten()
        .chain(once(&col.rhs_letter))
        .filter(|l| !assignment.contains_key(l))
        .collect();
    match current_cols_unassigned_letters.len() {
        0 => {
            // dbg!(&assignment);
            let lhs_values: Vec<&u8> = lhs_values.collect();
            let lhs_sum = lhs_values.into_iter().map(|v| *v as usize).sum::<usize>() + carry;
            let next_carry = lhs_sum / 10;
            if lhs_sum % 10 == assignment[&col.rhs_letter] as usize {
                step(
                    col_idx + 1,
                    cols,
                    next_carry,
                    assignment,
                    unassigned_digits,
                    leading_letters,
                )
            } else {
                Ok(false)
            }
        }

        1 => {
            let lhs_values: Vec<&u8> = lhs_values.collect();
            let lhs_sum = lhs_values.into_iter().map(|v| *v as usize).sum::<usize>() + carry;
            let implied_missing_val = if current_cols_unassigned_letters.contains(&&col.rhs_letter)
            {
                lhs_sum % 10
            } else {
                (assignment[&col.rhs_letter] as isize - lhs_sum as isize).rem_euclid(10) as usize
            };
            if !unassigned_digits.contains(&implied_missing_val)
                || (implied_missing_val == 0
                    && leading_letters.contains(current_cols_unassigned_letters[0]))
            {
                return Ok(false);
            }
            unassigned_digits.swap_remove(
                unassigned_digits
                    .iter()
                    .position(|&i| i == implied_missing_val)
                    .unwrap(),
            );
            assignment.insert(
                *current_cols_unassigned_letters[0],
                implied_missing_val as u8,
            );
            let next_carry = (lhs_sum
                + if current_cols_unassigned_letters.contains(&&col.rhs_letter) {
                    0
                } else {
                    implied_missing_val
                })
                / 10;
            if step(
                col_idx + 1,
                cols,
                next_carry,
                assignment,
                unassigned_digits,
                leading_letters,
            )? {
                Ok(true)
            } else {
                unassigned_digits.push(implied_missing_val);
                assignment.remove(current_cols_unassigned_letters[0]);
                Ok(false)
            }
        }
        _ => {
            let &c = current_cols_unassigned_letters
                .first()
                .ok_or(NoSolution::CurrentColHasNoUnassignedLetters)?;
            for possible_assignment in unassigned_digits.clone() {
                if possible_assignment == 0 && leading_letters.contains(c) {
                    continue;
                }
                unassigned_digits.swap_remove(
                    unassigned_digits
                        .iter()
                        .position(|&i| i == possible_assignment)
                        .unwrap(),
                );
                assignment.insert(*c, possible_assignment as u8);

                // After assigning c, recheck how many remain unassigned in this column
                let remaining: Vec<&u8> = col
                    .lhs_letters
                    .iter()
                    .flatten()
                    .chain(once(&col.rhs_letter))
                    .filter(|l| !assignment.contains_key(l))
                    .collect();

                let should_recurse = if remaining.len() == 1 {
                    // Derive the last letter directly — same logic as the `1 =>` arm
                    let lhs_sum = col
                        .lhs_letters
                        .iter()
                        .filter_map(|l| l.and_then(|ll| assignment.get(&ll)))
                        .map(|v| *v as usize)
                        .sum::<usize>()
                        + carry;
                    let last = remaining[0];
                    let implied = if last == &col.rhs_letter {
                        lhs_sum % 10
                    } else {
                        (assignment[&col.rhs_letter] as isize - lhs_sum as isize).rem_euclid(10)
                            as usize
                    };
                    if !unassigned_digits.contains(&implied)
                        || (implied == 0 && leading_letters.contains(last))
                    {
                        false
                    } else {
                        let pos = unassigned_digits
                            .iter()
                            .position(|&i| i == implied)
                            .unwrap();
                        unassigned_digits.swap_remove(pos);
                        assignment.insert(*last, implied as u8);
                        let next_carry =
                            (lhs_sum + if last == &col.rhs_letter { 0 } else { implied }) / 10;
                        let result = step(
                            col_idx + 1,
                            cols,
                            next_carry,
                            assignment,
                            unassigned_digits,
                            leading_letters,
                        )?;
                        if !result {
                            unassigned_digits.push(implied);
                            assignment.remove(last);
                        }
                        result
                    }
                } else {
                    step(
                        col_idx,
                        cols,
                        carry,
                        assignment,
                        unassigned_digits,
                        leading_letters,
                    )?
                };

                if should_recurse {
                    return Ok(true);
                }
                unassigned_digits.push(possible_assignment);
                assignment.remove(c);
            }
            Ok(false)
        }
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let n_unique_chars = {
        let mut v: Vec<char> = input.chars().filter(|c| c.is_alphabetic()).collect();
        v.sort_unstable();
        v.dedup();
        v.len()
    };
    let (lhs_words, rhs): (Vec<Vec<u8>>, &[u8]) = {
        let (lhs, rhs) = input.trim().split_once("==")?;
        (
            lhs.split('+')
                .map(|w| w.trim().as_bytes().to_vec())
                .collect(),
            rhs.trim().as_bytes(),
        )
    };
    let leading_letters: Vec<u8> = lhs_words
        .iter()
        .chain(once(&Vec::from(rhs)))
        .map(|w| w[0])
        .collect();
    let cols: Vec<Column> = (0..rhs.len())
        .map(|i| {
            lhs_words
                .iter()
                .map(|w| w.get(w.len().checked_sub(1)?.checked_sub(i)?).copied())
                .collect()
        })
        .zip(rhs.iter().rev())
        .map(|(lhs_letters, &rhs_letter)| Column {
            lhs_letters,
            rhs_letter,
        })
        .collect();

    let mut assignment: HashMap<u8, u8> = HashMap::with_capacity(n_unique_chars);
    let mut unassigned_digits: Vec<usize> = (0..=9).rev().collect();
    match step(
        0,
        &cols,
        0,
        &mut assignment,
        &mut unassigned_digits,
        &leading_letters,
    ) {
        Ok(true) => Some(
            assignment
                .into_iter()
                .map(|(k, v)| (k as char, v))
                .collect(),
        ),
        _ => None,
    }
}
