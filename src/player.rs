use crate::card::Card;
use crate::proto;

pub struct Client {
    pub id: String, // 玩家id
    pub name: String,
    pub cards: Vec<Card>, // 手牌
    pub player: Player,   // 玩家信息
}

impl Client {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id: id.clone(),
            name: name.clone(),
            cards: vec![],
            player: Player {
                name,
                score: 0,
                card_num: 0,
                index: 0,
                id,
            },
        }
    }

    pub fn modify_cards(&mut self, cards: Vec<Card>) {
        self.cards = cards;
        Card::sort_cards(&mut self.cards); // sort
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
#[derive(Debug)]
pub struct Player {
    pub name: String,  // 名字
    pub score: i32,    // 分数
    pub card_num: i32, // 剩余手牌数量
    pub index: u32,    // 在所有玩家中的下标
    pub id: String,    // id
}

impl Player {}

impl From<&proto::Player> for Player {
    fn from(player: &proto::Player) -> Self {
        Player {
            name: player.name.clone(),
            score: player.score,
            card_num: player.card_num,
            index: player.index,
            id: "".into(),
        }
    }
}

impl From<&Player> for proto::Player {
    fn from(player: &Player) -> Self {
        proto::Player {
            name: player.name.clone(),
            score: player.score,
            card_num: player.card_num,
            index: player.index,
            id: player.id.clone(),
        }
    }
}
