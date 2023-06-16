use std::rc::Rc;
use std::time::Duration;

use crate::player::{Client, Player};
use crate::proto::{StreamRequest, StreamResponse};
use crate::ui::{self, GameUI};
use crossterm;
use crossterm::event::{poll, Event};
use tokio::sync::mpsc::{Receiver, Sender};

pub struct Game {
    pub client: Rc<Client>,
    pub players: Rc<Vec<Player>>,
    pub request_sender: Sender<StreamRequest>,
    pub response_receiver: Receiver<StreamResponse>,
}

impl Game {
    pub fn new(
        id: String,
        request_sender: Sender<StreamRequest>,
        response_receiver: Receiver<StreamResponse>,
    ) -> Self {
        let client = Rc::new(Client::new(id));
        let players = Rc::new(vec![]);
        Self {
            client,
            players,
            request_sender,
            response_receiver,
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

            // check reponse receiver

            // handle event
        }
    }
}
