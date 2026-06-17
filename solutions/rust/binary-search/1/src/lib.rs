pub fn find(array: &[i32], key: i32) -> Option<usize> {
    use std::cmp::Ordering::*;
    let mut left = 0;
    let mut right = array.len().checked_sub(1)?;
    loop {
        let i = left + (right.checked_sub(left)?) / 2;
        match key.cmp(array.get(i)?) {
            Equal => return Some(i),
            Less => right = i.checked_sub(1)?,
            Greater => left = i.checked_add(1)?,
        }
    }
}
