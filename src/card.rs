use crate::game;

pub struct Card {
    pub suit: i32,  // 花色
    pub rank: i32,  // 
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
        card_to_string(self)
    }
}

pub fn card_to_string(card: &Card) -> String {
    format!("{}{}", suit_to_string(card.suit), rank_to_string(card.rank))
}

// map rank to string
pub fn rank_to_string(rank: i32) -> String {
    match rank {
        1..= 8  => (rank + 2).to_string(),
        9 => "J".to_string(),
        10 => "Q".to_string(),
        11 => "K".to_string(),
        12 => "A".to_string(),
        13 => "2".to_string(),
        14 => "Jocker0".to_string(),
        15 => "Jocker1".to_string(),
        _ => unreachable!()
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
