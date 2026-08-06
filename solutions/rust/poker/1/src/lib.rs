use std::{cmp::Ordering, collections::HashMap, str::FromStr};

#[derive(PartialEq, Debug)]
enum Suit {
    Hearts,
    Spades,
    Clubs,
    Diamonds,
}
#[derive(Debug)]
enum Rank {
    N(u8),
    Jack,
    Queen,
    King,
    Ace,
}
impl Rank {
    fn value(&self) -> u8 {
        match self {
            Rank::N(i) => *i,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
        }
    }
}

#[derive(Debug)]
struct Card {
    rank: Rank,
    suit: Suit,
}

#[derive(Debug)]
struct ParseError;
impl FromStr for Card {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rank, suit) = s.split_at(s.len() - 1);
        Ok(Card {
            rank: match rank {
                "2" => Rank::N(2),
                "3" => Rank::N(3),
                "4" => Rank::N(4),
                "5" => Rank::N(5),
                "6" => Rank::N(6),
                "7" => Rank::N(7),
                "8" => Rank::N(8),
                "9" => Rank::N(9),
                "10" => Rank::N(10),
                "J" => Rank::Jack,
                "Q" => Rank::Queen,
                "K" => Rank::King,
                "A" => Rank::Ace,
                _ => return Err(ParseError),
            },
            suit: match suit {
                "H" => Suit::Hearts,
                "S" => Suit::Spades,
                "D" => Suit::Diamonds,
                "C" => Suit::Clubs,
                _ => return Err(ParseError),
            },
        })
    }
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug)]
enum Category {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug)]
struct Score {
    category: Category,
    tiebreak: Vec<u8>,
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new(hand: &str) -> Option<Self> {
        let cards: Vec<Card> = hand
            .split(' ')
            .map(|s| s.parse().ok())
            .collect::<Option<_>>()?;
        (cards.len() == 5).then_some(Hand { cards })
    }

    fn straight_high(&self) -> Option<u8> {
        let mut ranks: Vec<u8> = self.cards.iter().map(|card| card.rank.value()).collect();
        ranks.sort_unstable();
        ranks.dedup();
        match ranks[..] {
            [a, .., e] if ranks.len() == 5 && e - a == 4 => Some(e),
            [2, 3, 4, 5, 14] => Some(5),
            _ => None,
        }
    }

    fn score(&self) -> Score {
        let mut counts: HashMap<u8, u8> = HashMap::new();
        for card in &self.cards {
            *counts.entry(card.rank.value()).or_insert(0) += 1;
        }
        // Group as (count, rank) and sort descending: bigger groups first, and
        // higher ranks first within the same group size. `shape` is then the
        // group sizes, which name the category, and `ranks` is the matching
        // rank order — which is exactly the tiebreak order for every
        // non-straight category: pair before kickers, triplet before pair,
        // quad before kicker, and plain high-card-down-to-the-last-card for
        // the rest. Straights instead rank solely by their top card, so the
        // wheel's ace does not carry its usual weight.
        let mut groups: Vec<(u8, u8)> = counts.into_iter().map(|(r, c)| (c, r)).collect();
        groups.sort_unstable_by(|a, b| b.cmp(a));
        let (shape, ranks): (Vec<u8>, Vec<u8>) = groups.into_iter().unzip();

        let is_flush = self
            .cards
            .iter()
            .all(|card| card.suit == self.cards[0].suit);

        let (category, tiebreak) = match (self.straight_high(), is_flush, shape.as_slice()) {
            (Some(high), true, _) => (Category::StraightFlush, vec![high]),
            (_, _, [4, 1]) => (Category::FourOfAKind, ranks),
            (_, _, [3, 2]) => (Category::FullHouse, ranks),
            (_, true, _) => (Category::Flush, ranks),
            (Some(high), _, _) => (Category::Straight, vec![high]),
            (_, _, [3, ..]) => (Category::ThreeOfAKind, ranks),
            (_, _, [2, 2, 1]) => (Category::TwoPair, ranks),
            (_, _, [2, ..]) => (Category::OnePair, ranks),
            _ => (Category::HighCard, ranks),
        };

        Score { category, tiebreak }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.score() == other.score()
    }
}
impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score().cmp(&other.score())
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let parsed: Vec<(&'a str, Hand)> = hands
        .iter()
        .map(|&hand| (hand, Hand::new(hand).expect("Unrecognized parse symbol!")))
        .collect();
    let Some((_, best)) = parsed.iter().max_by(|a, b| a.1.cmp(&b.1)) else {
        return vec![];
    };
    parsed
        .iter()
        .filter(|(_, hand)| hand == best)
        .map(|(hand_str, _)| *hand_str)
        .collect()
}
