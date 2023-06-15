use crate::{Client, Player};

pub struct Game {
    pub client: Client,
    pub players: Vec<Player>,
}

impl Game {
    pub fn game_loop(&mut self) {
        // TODO
    }
}
