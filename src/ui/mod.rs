use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    prelude::Widget,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::objects::stat::App;

pub fn render(frame: &mut Frame, app: &App) {
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(frame.size());

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(main_chunks[0]);

    let search = Paragraph::new("").block(
        Block::default()
            .title("Search")
            .borders(Borders::ALL)
            .border_set(border::HEAVY_QUADRUPLE_DASHED),
    );

    let options = Paragraph::new("packages").block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );

    let info = Paragraph::new("package info").block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .light_blue(),
    );

    frame.render_widget(search, left_chunks[0]);
    frame.render_widget(options, left_chunks[1]);
    frame.render_widget(info, main_chunks[1]);
}
