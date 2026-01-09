use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use ratatui::widgets::ListState;

use crate::backend::load_repo_packages;

#[derive(Debug, Clone)]
pub struct Package {
    pub is_installed: bool,
    pub repo: ItemRepo,
    pub size: u64, //bytes
    pub descipt: String,
    pub name: String,
}

impl Package {
    pub fn new(
        is_installed: bool,
        repo: ItemRepo,
        size: u64,
        descipt: String,
        name: String,
    ) -> Self {
        Self {
            is_installed,
            repo,
            size,
            descipt,
            name,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ItemRepo {
    Core,
    Extra,
    archlinuxcn,
    multilib,
    absOther(String),
    AUR,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct InsertMode {
    pub enabled: bool,
    pub index: usize,
}

#[derive(Debug)]
pub struct App {
    pub items: Vec<Package>,
    pub exit: bool,
    pub filtered: Vec<Package>,
    pub search: String,
    pub selected_pack: usize,
    pub list_state: ListState,
    pub selected_win: Window,
    pub insert_mode: InsertMode,
}

#[derive(Debug, Clone)]
pub enum Window {
    Info,
    Search,
    List,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let all_packages = load_repo_packages()?;
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        let app = Self {
            filtered: all_packages.clone(),
            exit: false,
            search: String::new(),
            selected_pack: 0,
            items: all_packages,
            list_state: list_state,
            selected_win: Window::Search,
            insert_mode: InsertMode {
                enabled: false,
                index: 0,
            },
        };
        Ok(app)
    }

    pub fn update_filter(&mut self) {
        self.filtered = self
            .items
            .iter()
            .filter(|p| p.name.contains(&self.search))
            .cloned()
            .collect();
        if self.selected_pack >= self.filtered.len() {
            self.selected_pack = 0;
        }
    }
}

impl From<&str> for ItemRepo {
    fn from(s: &str) -> Self {
        match s {
            "core" => ItemRepo::Core,
            "extra" => ItemRepo::Extra,
            "multilib" => ItemRepo::multilib,
            "archlinuxcn" => ItemRepo::archlinuxcn,
            "aur" => ItemRepo::AUR,
            other => ItemRepo::absOther(other.to_string()),
        }
    }
}
