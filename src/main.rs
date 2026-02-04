use core::result;
use std::error::Error;

use color_eyre::eyre::{Ok, Result};
use crossterm::event::{self, Event};

use ratatui::DefaultTerminal;
use tokio::select;
use tokio::time::Duration;
use tokio::time::sleep;

use crate::objects::stat::App;
use crate::ui::render;

pub mod backend;
pub mod objects;
#[cfg(test)]
pub mod test;
pub mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    color_eyre::install()?;

    let terminal = ratatui::init();
    let mut state = App::new()?;
    run(terminal, &mut state).await?;

    ratatui::restore();
    result::Result::Ok(())
}

pub async fn run(mut terminal: DefaultTerminal, app: &mut App) -> Result<()> {
    loop {
        select! {
            Some(pkgs) = app.aur_rx.recv() => {
                if pkgs.is_empty(){
                    app.notice = "aur searching error or the result is totally empty".to_string();
                }
                app.filtered.extend(pkgs);
                app.aur_search_block = false;
            }
            _ = sleep(Duration::from_millis(5)) => {
                // UI tick
                    terminal.draw(|frame| {
                        render(frame, app);
                    })?;
                    if let Event::Key(key) = event::read()? {
                        app.handle_key(key);
                        if app.exit {
                            break;
                        }
                    }
            }
        }

        //input handling
    }

    Ok(())
}
