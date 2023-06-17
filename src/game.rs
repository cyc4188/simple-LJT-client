use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crate::card::Card;
use crate::player::{Client, Player};
use crate::proto::{self, stream_response, Continue, End, Fail, StreamRequest, StreamResponse};
use crate::ui::{self, GameUI};
use crossterm;
use crossterm::event::{poll, Event};
use tokio::sync::mpsc::{Receiver, Sender};

pub struct Game {
    pub client: Rc<RefCell<Client>>,
    pub players: Rc<RefCell<Vec<Player>>>,
    pub request_sender: Sender<StreamRequest>,
    pub response_receiver: Receiver<StreamResponse>,
    pub current_cards: Vec<Card>,
}

impl Game {
    pub fn new(
        id: String,
        request_sender: Sender<StreamRequest>,
        response_receiver: Receiver<StreamResponse>,
    ) -> Self {
        let client = Rc::new(RefCell::new(Client::new(id)));
        let players = Rc::new(RefCell::new(vec![]));
        let current_cards = vec![];

        Self {
            client,
            players,
            request_sender,
            response_receiver,
            current_cards,
        }
    }

    pub fn game_loop(&mut self) {
        let mut ui = GameUI::new(self.client.clone(), self.players.clone());
        loop {
            // render the game ui
            ui.main_screen();

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
                    self.current_cards = cont.current_cards.iter().map(Card::from).collect();
                    let mut client = self.client.borrow_mut();
                    client.modify_cards(cont.cards.iter().map(Card::from).collect());
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
}
