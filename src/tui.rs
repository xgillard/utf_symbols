//! Terminal user interface for the project.

use anyhow::{Ok, Result};
use arboard::Clipboard;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::layout::Flex;
use ratatui::widgets::{Clear, Paragraph, TableState};
use ratatui::{prelude::*, text::ToText, widgets::{Block, Row, Table}, DefaultTerminal};
use tui_prompts::{State, TextPrompt, TextState};
use crate::unicode::{lookup_by_name, CodePoint};

const ABOUT: &'static str = "
    Press 'Down'  to select next item.
    Press 'up'    to select previous item.
    Press 'Enter' to copy selected item 
                  to system clipboard.

    Press 'Esc'   to close this dialog.
    Press 'q'     to quit.

    20205 -- X. Gillard
";

/// The overall application state
pub struct App<'a> {
    search: TextState<'a>,
    results: Vec<CodePoint>,
    popup: bool,
    table_state: TableState,
    clipboard: Clipboard,
}

impl App<'_> {
    pub fn new() -> Result<Self> {
        Ok(Self {
            search: TextState::default(),
            results: vec![],
            popup: false,
            table_state: TableState::default(),
            clipboard: Clipboard::new()?,
        })
    }
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|f| self.render(f))?;
            if self.input()? {
                break Ok(());
            }
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3),
                Constraint::Fill(1),
            ])
            .split(frame.area());

        let input = TextPrompt::from("Search :")
            .with_block(Block::bordered());

        let table = Table::new(
            &self.results, vec![
                Constraint::Length(3), 
                Constraint::Fill(1),
            ])
            .block(Block::bordered())
            .row_highlight_style(Style::new().on_blue());
        
        frame.render_stateful_widget(input, layout[0], &mut self.search);
        frame.render_stateful_widget(table, layout[1], &mut self.table_state);

        if self.popup {
            let about = Block::bordered()
                .title("About")
                .style(Style::new().on_blue());

            let about_msg = Paragraph::new(ABOUT)
                .style(Style::new().on_blue());

            let area = popup_area(frame.area(), 75, 50);
            let inner_area = about.inner(area);
            frame.render_widget(Clear, area);
            frame.render_widget(about, area);
            frame.render_widget(about_msg, inner_area);
        }
    }

    fn input(&mut self) -> Result<bool> {
        let event = event::read()?;
        match event {
            Event::Key(key_event) => 
                match key_event {
                    KeyEvent { code: KeyCode::Esc, .. } => {
                        if self.popup {
                            self.popup = false;
                            Ok(false)
                        } else {
                            self.popup = true;
                            Ok(false)
                        }
                    }, 
                    KeyEvent {code: KeyCode::Char('q'), ..} if self.popup => {
                        Ok(true)
                    },
                    KeyEvent { code: KeyCode::Down, ..} => {
                        if self.table_state.selected().is_none() {
                            self.table_state.select_first();
                        } else {
                            self.table_state.select_next();
                        }
                        Ok(false)
                    },
                    KeyEvent { code: KeyCode::Up, ..} => {
                        if self.table_state.selected().is_none() {
                            self.table_state.select_last();
                        } else {
                            self.table_state.select_previous();
                        }
                        Ok(false)
                    },
                    KeyEvent {code: KeyCode::Enter, ..} => {
                        if self.table_state.selected().is_none() {
                            self.table_state.select_first();
                        }
                        let selected = self.table_state.selected().unwrap();
                        let text = self.results[selected].0;
                        self.clipboard.set_text(text.to_string())?;
                        Ok(false)
                    },
                    e => {
                        self.search.handle_key_event(e);
                        lookup_by_name(self.search.value(), &mut self.results);
                        Ok(false)
                    },
                },
            _ => Ok(false)
        }
    }
}


/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

impl <'a> From<&'a CodePoint> for Row<'a> {
    fn from(value: &'a CodePoint) -> Self {
        Row::new([value.0.to_text(), value.1.to_text()])
    }
}