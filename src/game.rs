use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
use std::time::Duration;

use crate::card::Card;
use crate::player::{Client, Player};
use crate::proto::{self, stream_response, PlayCards, StreamRequest, StreamResponse};
use crate::ui::{self, GameUI, UIEvent};
use crossterm;
use crossterm::event::{poll, Event};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use tokio::sync::mpsc::{Receiver, Sender};

#[derive(Debug, Default)]
pub struct GameStatus {
    pub score: i32,
    pub current_cards: Vec<Card>,
    pub current_index: u32,
    pub players: HashMap<u32, Player>,
    pub last_played: HashMap<u32, Vec<Card>>,
}

impl GameStatus {
    pub fn new() -> Self {
        Self {
            score: 0,
            current_cards: vec![],
            current_index: 0,
            players: HashMap::new(),
            last_played: HashMap::new(),
        }
    }
    pub fn update(&mut self, cont: proto::Continue) {
        self.current_cards = cont.current_cards.iter().map(Card::from).collect();
        self.current_index = cont.current_player.unwrap().index;
        self.score = cont.score;
        // modify players
        for play_cards in cont.players.iter() {
            let proto_player = play_cards.player.as_ref().unwrap();
            let index = proto_player.index;
            self.players.insert(index, proto_player.into());
            // TODO: 修改 last_played
            let proto_last_played = &play_cards.cards;
            self.last_played.insert(
                index,
                proto_last_played
                    .iter()
                    .map(|proto_card| proto_card.into())
                    .collect(),
            );
        }
    }
}

pub struct Game {
    pub client: Rc<RefCell<Client>>,
    pub request_sender: Sender<StreamRequest>,
    pub response_receiver: Receiver<StreamResponse>,
    pub game_state: Rc<RefCell<GameStatus>>,
}

impl Game {
    pub fn new(
        id: String,
        request_sender: Sender<StreamRequest>,
        response_receiver: Receiver<StreamResponse>,
    ) -> Self {
        let client = Rc::new(RefCell::new(Client::new(id)));
        let game_state = Rc::new(RefCell::new(GameStatus::new()));

        Self {
            client,
            request_sender,
            response_receiver,
            game_state,
        }
    }

    pub async fn game_loop(&mut self) {
        // self.init().await.unwrap();
        let mut ui = GameUI::new(self.client.clone(), self.game_state.clone());
        enable_raw_mode().unwrap(); // important
        loop {
            // render the game ui
            ui.main_screen();

            // listen for event
            if poll(Duration::from_millis(ui::TICK_RATE)).unwrap() {
                if let Event::Key(key_event) = crossterm::event::read().unwrap() {
                    use crossterm::event::KeyCode::*;
                    match key_event.code {
                        Esc => break,
                        _ => match ui.handle_input(key_event.code) {
                            UIEvent::PlayCards(cards) => {
                                self.play_cards(cards).await;
                            }
                            UIEvent::Skip => {
                                self.pass().await;
                            }
                            _ => {}
                        },
                    };
                }
            }
            // handle event
            // check reponse receiver
            self.check_response();
        }
        // exit
        self.exit(&mut ui);
    }

    fn exit(&self, ui: &mut GameUI) {
        disable_raw_mode().unwrap();
        ui.terminal.borrow_mut().clear().unwrap();
        ui.terminal.borrow_mut().show_cursor().unwrap();
    }

    /// send play cards action to server
    // TODO: add error handler
    async fn play_cards(&mut self, cards: Vec<Card>) {
        let request = StreamRequest {
            request: Some(proto::stream_request::Request::PlayCards(
                proto::PlayCards {
                    cards: cards.into_iter().map(|c| c.into()).collect(),
                    player: Some(proto::Player::from(&self.client.borrow().player)),
                },
            )),
        };
        self.request_sender.send(request).await.unwrap();
    }

    /// send pass to server
    async fn pass(&mut self) {
        let request = StreamRequest {
            request: Some(proto::stream_request::Request::Pass(proto::Pass {
                player: Some(proto::Player::from(&self.client.borrow().player)),
            })),
        };
        self.request_sender.send(request).await.unwrap();
    }

    fn check_response(&mut self) {
        // recive reponse until empty
        while let Ok(response) = self.response_receiver.try_recv() {
            // modify the game state
            match response.response.expect("response is empty") {
                stream_response::Response::Continue(cont) => {
                    // change client cards
                    let mut client = self.client.borrow_mut();
                    client.modify_cards(cont.cards.iter().map(Card::from).collect());

                    // change game state
                    let mut game_status = self.game_state.borrow_mut();
                    game_status.update(cont);
                }
                stream_response::Response::Fail(fail) => {
                    // TODO: pop fail message
                    println!("fail message: {}", &fail.reason);
                }
                stream_response::Response::End(_) => {
                    // TODO: end the game
                }
            }
        }
    }
    // send init stream
    pub async fn init(&mut self) -> Result<(), Box<dyn Error>> {
        let player = proto::Player::from(&self.client.borrow().player);
        self.request_sender
            .send(StreamRequest {
                request: Some(proto::stream_request::Request::PlayCards(PlayCards {
                    cards: vec![],
                    player: Some(player),
                })),
            })
            .await
            .expect("stream connect failed");
        Ok(())
    }
}
