use std::{cell::RefCell, rc::Rc};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame, Terminal,
};

use crate::{
    card::Card,
    game::GameState,
    player::{Client, Player},
};
use std::io;

pub enum UIstate {}

pub const TICK_RATE: u64 = 250;

pub type TerminalType = Terminal<CrosstermBackend<io::Stdout>>;
pub struct GameUI {
    client: Rc<RefCell<Client>>,        // 显示手牌
    players: Rc<RefCell<Vec<Player>>>,  // 显示其他玩家
    terminal: TerminalType,             // 绘制 ui
    game_state: Rc<RefCell<GameState>>, // 还需要一个用于绘制场上分数、当前出牌、当前出牌玩家的 ui
}

impl GameUI {
    pub fn new(
        client: Rc<RefCell<Client>>,
        players: Rc<RefCell<Vec<Player>>>,
        game_state: Rc<RefCell<GameState>>,
    ) -> Self {
        let stdout = io::stdout;
        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.clear().unwrap();
        Self {
            client,
            terminal,
            players,
            game_state,
        }
    }

    pub fn main_screen(&mut self) {
        const MENU: [&str; 2] = ["Game", "Chat"];

        let client = self.client.clone();
        self.terminal
            .draw(|rect| {
                let chunks = Layout::default()
                    .direction(tui::layout::Direction::Vertical)
                    .margin(2)
                    .constraints([Constraint::Length(3), Constraint::Min(20)].as_ref())
                    .split(rect.size());

                GameUI::draw_menu(rect, chunks[0], &MENU, 0);
                GameUI::draw_main(rect, chunks[1], client);
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
    fn draw_main(
        f: &mut Frame<CrosstermBackend<io::Stdout>>,
        location: Rect,
        client: Rc<RefCell<Client>>,
    ) {
        let chunks = Layout::default()
            .direction(tui::layout::Direction::Horizontal)
            .margin(2)
            .constraints([Constraint::Length(60), Constraint::Min(40)].as_ref())
            .split(location);

        let block = Block::default().title("Game").borders(Borders::ALL);
        f.render_widget(block, chunks[0]);

        // show cards in chunks[0]
        let cards = client.borrow().cards.clone();
        let subchunks = Layout::default()
            .direction(tui::layout::Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[0]);
        for (i, card) in cards.iter().enumerate() {
            let card_widget = GameUI::card_widget(card.clone());
            f.render_widget(card_widget, subchunks[i]);
        }

        let block = Block::default().title("Chat").borders(Borders::ALL);
        f.render_widget(block, chunks[1]);
    }

    fn card_widget(card: Card) -> Paragraph<'static> {
        let card_str = format!("{}^{}", card.suit, card.rank);
        let card_span = Span::styled(card_str, Style::default().fg(Color::Green));
        let card_spans = Spans::from(vec![card_span]);
        Paragraph::new(card_spans)
    }
}
