use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use std::time::Duration;

use crate::card::Card;
use crate::player::{Client, Player};
use crate::proto::{
    self, stream_response, Continue, End, Fail, PlayCards, StreamRequest, StreamResponse,
};
use crate::ui::{self, GameUI};
use crossterm;
use crossterm::event::{poll, Event};
use tokio::sync::mpsc::{Receiver, Sender};

#[derive(Debug)]
pub struct GameState {
    pub score: i32,
    pub current_cards: Vec<Card>,
    pub current_index: u32,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            score: 0,
            current_cards: vec![],
            current_index: 0,
        }
    }
}

pub struct Game {
    pub client: Rc<RefCell<Client>>,
    pub players: Rc<RefCell<Vec<Player>>>,
    pub request_sender: Sender<StreamRequest>,
    pub response_receiver: Receiver<StreamResponse>,
    pub game_state: Rc<RefCell<GameState>>,
    // pub current_cards: Vec<Card>,
}

impl Game {
    pub fn new(
        id: String,
        request_sender: Sender<StreamRequest>,
        response_receiver: Receiver<StreamResponse>,
    ) -> Self {
        let client = Rc::new(RefCell::new(Client::new(id)));
        let players = Rc::new(RefCell::new(vec![]));
        let game_state = Rc::new(RefCell::new(GameState::new()));

        Self {
            client,
            players,
            request_sender,
            response_receiver,
            game_state,
        }
    }

    pub async fn game_loop(&mut self) {
        self.init().await.unwrap();
        let mut ui = GameUI::new(
            self.client.clone(),
            self.players.clone(),
            self.game_state.clone(),
        );
        loop {
            // render the game ui
            // ui.main_screen();

            // listen for event
            if poll(Duration::from_millis(ui::TICK_RATE)).unwrap() {
                if let Event::Key(key_event) = crossterm::event::read().unwrap() {
                    use crossterm::event::KeyCode::*;
                    match key_event.code {
                        Esc => break,
                        Left => {}
                        Right => {}
                        _ => (),
                    }
                }
            }
            // handle event

            // check reponse receiver
            self.check_response();
        }
    }

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
                    // change game state
                    let mut game_state = self.game_state.borrow_mut();
                    game_state.current_cards = cont.current_cards.iter().map(Card::from).collect();
                    game_state.current_index = cont.current_player.unwrap().index;
                    game_state.score = cont.score;

                    // change client cards
                    let mut client = self.client.borrow_mut();
                    client.modify_cards(cont.cards.iter().map(Card::from).collect());
                    println!("{:?}", client.cards);
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
