use std::{cell::RefCell, rc::Rc, time::Duration};

use crossterm::{
    event::{poll, Event},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Clear, Paragraph},
};

use super::util::{centered_rect, TerminalType};

pub struct RoomUI {
    pub terminal: Rc<RefCell<TerminalType>>, // 绘制 ui
    pub roomid: String,
}

const TICKS: u64 = 250;

impl RoomUI {
    pub fn new(terminal: Rc<RefCell<TerminalType>>) -> Self {
        Self {
            terminal,
            roomid: String::new(),
        }
    }

    pub fn screen(&mut self) -> Option<String> {
        let mut run = true;
        let terminal = self.terminal.clone();
        enable_raw_mode().unwrap();

        while run {
            terminal
                .borrow_mut()
                .draw(|f| {
                    let size = centered_rect(40, 50, f.size());
                    f.render_widget(Clear, size);

                    let layout = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(1)
                        .constraints(
                            [
                                Constraint::Length(1),
                                Constraint::Length(3),
                                Constraint::Length(3),
                                Constraint::Length(3),
                                Constraint::Percentage(100),
                            ]
                            .as_ref(),
                        )
                        .split(size);
                    let input = Paragraph::new(&self.roomid[..])
                        .alignment(Alignment::Center)
                        .block(Block::default().borders(Borders::ALL).title("RoomId"));

                    let text = Paragraph::new("Enter RoomId").alignment(Alignment::Left);
                    let instruction = Paragraph::new("Press Enter to confirm")
                        .alignment(Alignment::Left)
                        .block(Block::default().borders(Borders::ALL));
                    f.render_widget(Block::default().borders(Borders::all()), size);
                    f.render_widget(text, layout[0]);
                    f.render_widget(input, layout[1]);
                    f.render_widget(instruction, layout[2]);
                })
                .unwrap();

            if poll(Duration::from_millis(TICKS)).unwrap() {
                if let Event::Key(event) = crossterm::event::read().unwrap() {
                    use crossterm::event::KeyCode::*;

                    match event.code {
                        Char(c) => {
                            if self.roomid.len() <= 20 {
                                self.roomid.push(c);
                            }
                        }
                        Esc => {
                            disable_raw_mode().unwrap();
                            terminal.borrow_mut().clear().unwrap();
                            return None;
                        }
                        Backspace => {
                            self.roomid.pop();
                        }
                        Enter => {
                            run = false;
                        }
                        _ => (),
                    }
                }
            }
        }
        disable_raw_mode().unwrap();
        Some(self.roomid.clone())
    }
}
