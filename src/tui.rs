//! Terminal user interface for the project.

use anyhow::Result;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{prelude::*, text::ToText, widgets::{Block, Row, Table}, DefaultTerminal};
use tui_prompts::{State, TextPrompt, TextState};
use crate::unicode::{lookup_by_name, CodePoint};


/// The overall application state
#[derive(Debug, Default)]
pub struct App<'a> {
    search: TextState<'a>,
    results: Vec<CodePoint>,
}

impl App<'_> {
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
            ]);
        
        frame.render_stateful_widget(input, layout[0], &mut self.search);
        frame.render_widget(table, layout[1]);
    }

    fn input(&mut self) -> Result<bool> {
        let event = event::read()?;
        match event {
            Event::Key(key_event) => 
                match key_event {
                    KeyEvent { code: KeyCode::Esc, .. } => Ok(true),
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


impl <'a> From<&'a CodePoint> for Row<'a> {
    fn from(value: &'a CodePoint) -> Self {
        Row::new([value.0.to_text(), value.1.to_text()])
    }
}