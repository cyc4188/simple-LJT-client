use crate::player::{Client, Player};
use crate::proto::StreamRequest;

pub struct Game {
    pub players: Vec<Player>,
}

impl Game {
    /// this function is used to bridge the client and server
    /// it will receive the message from client and send it to server
    /// and then receive the message from server and send it to client
    pub fn bridge(&mut self) {
        // TODO
    }
}
