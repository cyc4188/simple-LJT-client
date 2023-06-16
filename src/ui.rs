use std::rc::Rc;

use tui::{backend::CrosstermBackend, Terminal};

use crate::player::{Client, Player};
use std::io;

pub enum UIstate {}

pub const TICK_RATE: u64 = 250;

pub type TerminalType = Terminal<CrosstermBackend<io::Stdout>>;
pub struct GameUI {
    client: Rc<Client>,
    players: Rc<Vec<Player>>,
    terminal: TerminalType,
}

impl GameUI {
    pub fn new(client: Rc<Client>, players: Rc<Vec<Player>>) -> Self {
        let stdout = io::stdout;
        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend).unwrap();
        Self {
            client,
            terminal,
            players,
        }
    }

    pub fn main_screen(&mut self) {
        self.terminal.clear().expect("cannot clear the terminal");
    }
}
