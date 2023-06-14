use crate::card::Card;

pub struct Player {
    pub id: String,       // 玩家id
    pub cards: Vec<Card>, // 手牌
}

impl Player {
    pub fn new(id: String) -> Self {
        Player {
            id,
            cards: Vec::new(),
        }
    }
    pub fn modify_cards(&mut self, cards: Vec<Card>) {
        Card::sort_cards(&mut self.cards); // sort first
        self.cards = cards;
    }
    pub fn show_cards(&self) -> String {
        self.cards
            .iter()
            .map(|card| card.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
