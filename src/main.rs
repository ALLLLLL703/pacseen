use core::result;
use std::error::Error;

use color_eyre::eyre::{Ok, Result};
use crossterm::event::{self, Event};

use ratatui::DefaultTerminal;

use crate::backend::load_repo_packages;
use crate::objects::stat::App;
use crate::ui::render;

pub mod backend;
pub mod objects;
#[cfg(test)]
pub mod test;
pub mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    let mut state = App::new()?;
    color_eyre::install();

    let terminal = ratatui::init();
    let result = run(terminal, &mut state);

    ratatui::restore();
    result::Result::Ok(())
}

pub fn run(mut terminal: DefaultTerminal, app: &mut App) -> Result<()> {
    loop {
        //Rendering
        terminal.draw(|frame| render(frame, app))?;
        //input handling
        if let Event::Key(key) = event::read()? {
            app.handle_key(key);
            if app.exit == true {
                break;
            }
        }
    }

    Ok(())
}
