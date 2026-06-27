use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}
pub fn classify(num: u64) -> Option<Classification> {
    (num != 0).then_some({
        let aliquot_sum: u64 = (1..=num / 2).filter(|&d| num.is_multiple_of(d)).sum();
        match num.cmp(&aliquot_sum) {
            Ordering::Less => Classification::Abundant,
            Ordering::Equal => Classification::Perfect,
            Ordering::Greater => Classification::Deficient,
        }
    })
}
