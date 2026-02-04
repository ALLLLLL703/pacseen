use std::io;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Widget,
    style::{Color, Modifier, Style, Stylize, palette::tailwind::SLATE},
    symbols::border,
    text::Line,
    widgets::{
        Block, BorderType, Borders, HighlightSpacing, List, ListItem, Padding, Paragraph,
        StatefulWidget, Wrap,
    },
};
use tokio::time::Duration;
use tokio::time::sleep;

use crate::objects::stat::{App, ItemRepo};

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;

pub fn render<'a>(frame: &mut Frame<'a>, app: &mut App) {
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(frame.area());

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(main_chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)])
        .split(main_chunks[1]);

    let search = Paragraph::new(app.search.as_str())
        .block(
            Block::default()
                .title("Search")
                .borders(Borders::ALL)
                .border_set(border::HEAVY_QUADRUPLE_DASHED),
        )
        .style(match app.insert_mode.enabled {
            true => Style::default(),
            false => Style::default().fg(Color::Yellow),
        });

    let items: Vec<ListItem> = app
        .filtered
        .iter()
        .map(|p| {
            let prefix = if p.is_installed { "●" } else { " " };

            ListItem::new(format!(
                "{} {:<24} [{}]",
                prefix,
                p.name,
                match &p.repo {
                    ItemRepo::Core => "core",
                    ItemRepo::Extra => "extra",
                    ItemRepo::Multilib => "multilib",
                    ItemRepo::Archlinuxcn => "archlinuxcn",
                    ItemRepo::AbsOther(string) => string,
                    ItemRepo::AUR(_) => "aur",
                }
            ))
        })
        .collect();
    let options = List::new(items)
        .block(
            Block::default()
                .title("Packages")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .highlight_style(SELECTED_STYLE)
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always)
        .style(Style::new());

    let info = Paragraph::new("package info").block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .light_blue(),
    );

    let notice = Paragraph::new(app.notice.clone()).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .light_blue(),
    );

    frame.render_widget(search, left_chunks[0]);
    StatefulWidget::render(
        options,
        left_chunks[1],
        frame.buffer_mut(),
        &mut app.list_state,
    );
    frame.render_widget(info, right_chunks[0]);
    app.render_selected_item(right_chunks[0], frame.buffer_mut());
    frame.render_widget(notice, right_chunks[1]);
    if app.insert_mode.enabled {
        let original_x = left_chunks[0].x;
        let original_y = left_chunks[0].y;

        frame.set_cursor_position((
            original_x + app.insert_mode.index as u16 + 1,
            original_y + 1,
        ));
    }
}

impl App {
    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        if !self.insert_mode.enabled {
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') => self.exit = true,
                KeyCode::Char('h') | KeyCode::Left => self.select_none(),
                KeyCode::Char('l') | KeyCode::Enter => {
                    if let Some(i) = self.list_state.selected() {
                        self.install_pack(i);
                        println!("will went back to pacseen after type <enter>");
                        io::stdin()
                            .read_line(&mut String::new())
                            .unwrap_or_default();

                        let mut terminal: DefaultTerminal = ratatui::init();
                        terminal.clear().unwrap_or_default();
                        terminal.draw(|frame| {
                            render(frame, self);
                        });
                    }
                }
                KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
                KeyCode::Char('j') | KeyCode::Down => self.select_next(),
                KeyCode::Char('g') | KeyCode::Home => self.select_first(),
                KeyCode::Char('G') | KeyCode::End => self.select_last(),
                KeyCode::Char('i') | KeyCode::Tab => self.insert_mode.enabled = true,
                _ => {}
            }
        } else {
            match key.code {
                KeyCode::Backspace => self.delete_char(),
                KeyCode::Enter | KeyCode::Tab => self.insert_mode.enabled = false,
                KeyCode::Char(c) => self.enter_char(c),
                KeyCode::Left => self.move_cursor_left(),
                KeyCode::Right => self.move_cursor_right(),

                _ => {}
            }

            self.update_filter_local();
        }
    }

    fn select_none(&mut self) {
        self.list_state.select(None);
    }

    fn select_next(&mut self) {
        self.list_state.select_next();
    }

    fn select_previous(&mut self) {
        self.list_state.select_previous();
    }

    fn select_first(&mut self) {
        self.list_state.select_first();
    }

    fn select_last(&mut self) {
        self.list_state.select_last();
    }

    pub fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        let info = if let Some(i) = self.list_state.selected() {
            let pak = self.filtered[i].clone();
            let repo_name = match &pak.repo {
                ItemRepo::Core => "core",
                ItemRepo::Extra => "Extra",
                ItemRepo::Archlinuxcn => "archlinuxcn",
                ItemRepo::Multilib => "multilib",
                ItemRepo::AUR(_) => "aur",
                ItemRepo::AbsOther(string) => &string.clone()[..],
            };
            match pak.repo {
                ItemRepo::AUR(votes) => format!(
                    "name:\t{}\nrepo:\t{}\ninstalled:\t{}\ndesc:\t{}\nvote:\t{}\n",
                    pak.name, repo_name, pak.is_installed, pak.descipt, votes,
                ),

                _ => format!(
                    "name:\t{}\nrepo:\t{}\ninstalled:\t{}\ndesc:\t{}\n",
                    pak.name, repo_name, pak.is_installed, pak.descipt
                ),
            }
        } else {
            "Nothing selected".to_string()
        };

        let block = Block::new()
            .title(Line::raw("Package Info").centered())
            .borders(Borders::TOP)
            .padding(Padding::horizontal(1));

        Paragraph::new(info)
            .block(block)
            .fg(TEXT_FG_COLOR)
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }
    fn delete_char(&mut self) {
        let is_not_cursor_left_most = self.insert_mode.index != 0;
        if is_not_cursor_left_most {
            let current_index = self.insert_mode.index;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.search.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.search.chars().skip(current_index);

            self.search = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.insert_mode.index.saturating_sub(1);
        self.insert_mode.index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.insert_mode.index.saturating_add(1);
        self.insert_mode.index = self.clamp_cursor(cursor_moved_right);
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.search.chars().count())
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.search.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.search
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.insert_mode.index)
            .unwrap_or(self.search.len())
    }
}
