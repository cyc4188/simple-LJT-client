use crate::game;

// #[derive(PartialOrd, Eq, PartialEq)]
pub struct Card {
    pub suit: i32, // 花色
    pub rank: i32, //
}

impl Card {
    pub fn sort_cards(cards: &mut [Card]) {
        cards.sort_by(|a, b| {
            if a.rank == b.rank {
                a.suit.cmp(&b.suit)
            } else {
                a.rank.cmp(&b.rank)
            }
        });
    }
}

impl From<&game::Card> for Card {
    fn from(card: &game::Card) -> Self {
        Card {
            suit: card.suit,
            rank: card.rank,
        }
    }
}
impl ToString for Card {
    fn to_string(&self) -> String {
        format!("{}{}", suit_to_string(self.suit), rank_to_string(self.rank))
    }
}

// map rank to string
pub fn rank_to_string(rank: i32) -> String {
    match rank {
        1..=8 => (rank + 2).to_string(),
        9 => "J".to_string(),
        10 => "Q".to_string(),
        11 => "K".to_string(),
        12 => "A".to_string(),
        13 => "2".to_string(),
        14 => "Jocker0".to_string(),
        15 => "Jocker1".to_string(),
        _ => unreachable!(),
    }
}

pub fn suit_to_string(suit: i32) -> String {
    match suit {
        0 => "♠".to_string(),
        1 => "♥".to_string(),
        2 => "♣".to_string(),
        3 => "♦".to_string(),
        _ => "".to_string(),
    }
}
