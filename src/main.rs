use color_eyre::eyre::{Ok, Result};
use crossterm::event::{self, Event};

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::terminal,
    prelude::Widget,
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Paragraph},
};

use crate::objects::stat::App;
use crate::ui::render;

pub mod backend;
pub mod objects;
pub mod ui;
fn main() {
    let mut state = App::default();
    color_eyre::install();

    let terminal = ratatui::init();
    let result = run(terminal, &mut state);

    ratatui::restore();
}

fn run(mut terminal: DefaultTerminal, app: &mut App) -> Result<()> {
    loop {
        //Rendering
        terminal.draw(|frame| render(frame, app))?;
        //input handling
        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Esc | event::KeyCode::Char('q') => {
                    break;
                }
                _ => {}
            }
        }
    }

    Ok(())
}
