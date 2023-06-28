use std::{cell::RefCell, collections::HashSet, rc::Rc};

use crossterm::event::KeyCode;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame, Terminal,
};

use crate::{
    card::Card,
    game::GameState,
    player::{Client, Player},
    ui,
};
use std::io;

pub enum UIEvent {
    None,
    PlayCards(Vec<Card>), // 出牌
    Skip,                 // 跳过
                          // TODO: send message
}

pub const TICK_RATE: u64 = 250;

pub type TerminalType = Terminal<CrosstermBackend<io::Stdout>>;
pub struct GameUI {
    select_index: HashSet<usize>,            // 已选择要出的牌
    current_index: usize,                    // 选择的位置
    client: Rc<RefCell<Client>>,             // 显示手牌
    players: Rc<RefCell<Vec<Player>>>,       // 显示其他玩家
    game_state: Rc<RefCell<GameState>>, // 还需要一个用于绘制场上分数、当前出牌、当前出牌玩家的 ui
    pub terminal: Rc<RefCell<TerminalType>>, // 绘制 ui
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
            terminal: Rc::new(RefCell::new(terminal)),
            players,
            game_state,
            select_index: HashSet::new(),
            current_index: 0,
        }
    }

    pub fn main_screen(&mut self) {
        const MENU: [&str; 2] = ["Game", "Chat"];

        let terminal = self.terminal.clone();
        terminal
            .borrow_mut()
            .draw(|rect| {
                let chunks = Layout::default()
                    .direction(tui::layout::Direction::Vertical)
                    .margin(2)
                    .constraints([Constraint::Length(3), Constraint::Min(20)].as_ref())
                    .split(rect.size());

                self.draw_menu(rect, chunks[0], &MENU, 0);
                self.draw_main(rect, chunks[1]);
            })
            .unwrap();
    }

    fn draw_menu(
        &self,
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
    fn draw_main(&self, f: &mut Frame<CrosstermBackend<io::Stdout>>, location: Rect) {
        let chunks = Layout::default()
            .direction(tui::layout::Direction::Horizontal)
            .margin(2)
            .constraints([Constraint::Min(100), Constraint::Min(40)].as_ref())
            .split(location);

        // Game Part
        self.draw_game(f, chunks[0]);
        // Chat Part
        self.draw_chat(f, chunks[1]);
    }

    fn draw_game(&self, f: &mut Frame<CrosstermBackend<io::Stdout>>, location: Rect) {
        // split with 2 parts
        let chunks = Layout::default()
            .direction(tui::layout::Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(location);

        // chunks[0] is for other players
        // TODO
        // chunks[1] is for cards
        self.draw_cards(f, chunks[1]);
    }

    fn draw_chat(&self, f: &mut Frame<CrosstermBackend<io::Stdout>>, location: Rect) {
        let block = Block::default().title("Chat").borders(Borders::ALL);
        f.render_widget(block, location);
        // TODO: add logic
    }

    fn draw_cards(&self, f: &mut Frame<CrosstermBackend<io::Stdout>>, location: Rect) {
        let cards = &self.client.borrow().cards;
        let card_widget = self.card_widget(cards);
        let cards = Paragraph::new(card_widget)
            .block(Block::default().title("Cards").borders(Borders::ALL))
            .alignment(Alignment::Center);
        f.render_widget(cards, location);
    }

    fn card_widget(&self, cards: &[Card]) -> Spans<'static> {
        let mut spans = vec![];
        for (index, card) in cards.iter().enumerate() {
            let card_str = format!("{}", card.to_string());
            let mut card_style = Style::default().fg(Color::White);

            // selected
            if self.select_index.contains(&index) {
                card_style = card_style.fg(Color::Yellow);
            }

            // current: underline
            if self.current_index == index {
                card_style = card_style.add_modifier(tui::style::Modifier::UNDERLINED);
            }

            spans.push(Span::styled(card_str, card_style));

            spans.push(Span::raw(" "));
        }
        Spans::from(spans)
    }

    /// 处理用户输入
    /// 如果是需要连接服务端，则返回一个 UIEvent
    /// 其他情况在 ui 内部处理
    pub fn handle_input(&mut self, keycode: KeyCode) -> UIEvent {
        use KeyCode::*;
        match keycode {
            Left => {
                let len = self.client.borrow().cards.len();
                if len == 0 {
                    return UIEvent::None;
                }
                self.current_index = (self.current_index + len - 1) % len;
                UIEvent::None
            }
            Right => {
                let len = self.client.borrow().cards.len();
                if len == 0 {
                    return UIEvent::None;
                }
                self.current_index = (self.current_index + 1) % len;
                UIEvent::None
            }
            // space
            Char(' ') => {
                // 反选
                if self.select_index.contains(&self.current_index) {
                    self.select_index.remove(&self.current_index);
                } else {
                    self.select_index.insert(self.current_index);
                }
                UIEvent::None
            }
            Enter => {
                // 出牌
                if !self.select_index.is_empty() {
                    let mut cards = vec![];
                    for index in &self.select_index {
                        cards.push(self.client.borrow().cards[*index].clone());
                    }
                    self.select_index.clear();
                    self.current_index = 0;
                    return UIEvent::PlayCards(cards);
                }
                UIEvent::None
            }

            _ => UIEvent::None,
        }
    }
}
