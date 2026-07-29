use std::array;

#[derive(Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}
type Dice = [u8; 5];

fn count(dice: Dice, n: u8) -> u8 {
    dice.into_iter().filter(|&i| i == n).count() as u8
}

pub fn score(mut dice: Dice, category: Category) -> u8 {
    match category {
        Category::Ones => count(dice, 1),
        Category::Twos => 2 * count(dice, 2),
        Category::Threes => 3 * count(dice, 3),
        Category::Fours => 4 * count(dice, 4),
        Category::Fives => 5 * count(dice, 5),
        Category::Sixes => 6 * count(dice, 6),
        Category::FourOfAKind => (1..=6)
            .map(|i| (i, count(dice, i)))
            .find_map(|(i, cnt)| (cnt >= 4).then_some(4 * i))
            .unwrap_or(0),
        Category::FullHouse => {
            let counts: [u8; 5] = array::from_fn(|i| count(dice, i as u8 + 1));
            if counts.contains(&3) && counts.contains(&2) {
                dice.into_iter().sum()
            } else {
                0
            }
        }
        Category::LittleStraight => {
            dice.sort_unstable();
            if dice == [1, 2, 3, 4, 5] { 30 } else { 0 }
        }
        Category::BigStraight => {
            dice.sort_unstable();
            if dice == [2, 3, 4, 5, 6] { 30 } else { 0 }
        }
        Category::Choice => dice.iter().sum(),
        Category::Yacht => {
            let first = dice[0];
            if dice.iter().all(|&i| i == first) {
                50
            } else {
                0
            }
        }
    }
}
