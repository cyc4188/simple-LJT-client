use crate::card::Card;
use crate::proto;

pub struct Client {
    pub id: String,       // 玩家id
    pub cards: Vec<Card>, // 手牌
    pub player: Player,   // 玩家信息
}

impl Client {
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

/// 玩家信息，可能会有多个玩家，因此并不存储手牌信息和id
pub struct Player {
    pub name: String,
    pub score: i32,
    pub card_num: i32,
    pub index: u32,
}

impl Player {}

impl From<&proto::Player> for Player {
    fn from(player: &proto::Player) -> Self {
        Player {
            name: player.name.clone(),
            score: player.score,
            card_num: player.card_num,
            index: player.index,
        }
    }
}
