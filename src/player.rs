use tokio::sync::mpsc::Sender;

use crate::card::show_cards;
use crate::card::Card;
use crate::proto::{self, stream_request, StreamRequest};
use tokio::io::AsyncBufReadExt;

pub struct Client {
    pub id: String,       // 玩家id
    pub cards: Vec<Card>, // 手牌
    pub player: Player,   // 玩家信息
}

impl Client {
    pub fn new(id: String) -> Self {
        Self {
            id,
            cards: vec![],
            player: Player {
                name: "test".into(),
                score: 0,
                card_num: 0,
                index: 0,
            },
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

    pub async fn listen_to_keyboard(&mut self, tx: Sender<StreamRequest>) {
        let stdin = tokio::io::stdin();
        let mut lines = tokio::io::BufReader::new(stdin).lines();
        while let Some(line) = lines.next_line().await.unwrap() {
            println!("you entered {}", line);
            let cards: Vec<_> = line.split_ascii_whitespace().map(Card::from).collect();

            println!("{}", show_cards(&cards));

            let request = StreamRequest {
                request: Some(stream_request::Request::PlayCards(proto::PlayCards {
                    player: Some(proto::Player {
                        id: self.id.clone(),
                        name: "test".into(),
                        score: 0,
                        card_num: 0,
                        index: 0,
                    }),
                    cards: cards.into_iter().map(Card::into).collect(),
                })),
            };
            // TODO: check if the cards are valid
            tx.send(request).await.unwrap();
        }
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
