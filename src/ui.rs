use std::{cell::RefCell, rc::Rc};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Frame, Terminal,
};

use crate::player::{Client, Player};
use std::io;

pub enum UIstate {}

pub const TICK_RATE: u64 = 250;

pub type TerminalType = Terminal<CrosstermBackend<io::Stdout>>;
pub struct GameUI {
    client: Rc<RefCell<Client>>,
    players: Rc<RefCell<Vec<Player>>>,
    terminal: TerminalType,
}

impl GameUI {
    pub fn new(client: Rc<RefCell<Client>>, players: Rc<RefCell<Vec<Player>>>) -> Self {
        let stdout = io::stdout;
        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.clear().unwrap();
        Self {
            client,
            terminal,
            players,
        }
    }

    pub fn main_screen(&mut self) {
        const MENU: [&str; 2] = ["Game", "Chat"];

        self.terminal
            .draw(|rect| {
                let chunks = Layout::default()
                    .direction(tui::layout::Direction::Vertical)
                    .margin(2)
                    .constraints([Constraint::Length(3), Constraint::Min(20)].as_ref())
                    .split(rect.size());

                GameUI::draw_menu(rect, chunks[0], &MENU, 0);
            })
            .unwrap();
    }

    fn draw_menu(
        f: &mut Frame<CrosstermBackend<io::Stdout>>,
        location: Rect,
        menu: &[&str],
        selected: usize,
    ) {
        let items: Vec<_> = menu
            .iter()
            .map(|item| {
                let (first, rest) = item.split_at(1);
                let first = first.to_uppercase();
                Spans::from(vec![
                    Span::styled(first, Style::default().fg(Color::Yellow)),
                    Span::styled(rest, Style::default().fg(Color::White)),
                ])
            })
            .collect();
        let tabs = Tabs::new(items)
            .select(selected)
            .block(Block::default().title("Menu").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow))
            .divider(Span::raw("|"));
        f.render_widget(tabs, location);
    }
}
