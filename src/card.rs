use std::str::FromStr;

use crate::proto;
#[derive(Debug, Clone)]
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

impl From<&proto::Card> for Card {
    fn from(card: &proto::Card) -> Self {
        Card {
            suit: card.suit,
            rank: card.rank,
        }
    }
}
impl Into<proto::Card> for Card {
    fn into(self) -> proto::Card {
        proto::Card {
            suit: self.suit,
            rank: self.rank,
        }
    }
}
impl From<&str> for Card {
    fn from(s: &str) -> Self {
        Card::from_str(s).unwrap()
    }
}
impl ToString for Card {
    fn to_string(&self) -> String {
        format!("{}{}", suit_to_string(self.suit), rank_to_string(self.rank))
    }
}
impl FromStr for Card {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let suit = match chars.next() {
            Some('0') => 0,
            Some('1') => 1,
            Some('2') => 2,
            Some('3') => 3,
            _ => return Err(()),
        };
        let rank = match chars.next() {
            Some('J') => 9,
            Some('Q') => 10,
            Some('K') => 11,
            Some('A') => 12,
            Some('2') => 13,
            Some('0') => 14,
            Some('1') => 15,
            Some(c) => c.to_digit(10).unwrap() as i32 - 2,
            _ => return Err(()),
        };
        Ok(Card { suit, rank })
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
        14 => "🃏0".to_string(),
        15 => "🃏1".to_string(),
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

pub fn show_cards(cards: &[Card]) -> String {
    cards
        .iter()
        .map(|card| card.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}
