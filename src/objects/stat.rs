use std::{error::Error, thread, time::Duration};

use ratatui::widgets::ListState;
use tokio::{
    sync::mpsc::{self, error::TryRecvError},
    task::JoinHandle,
    time::sleep,
};

use crate::backend::{aur::get_aur_packages, load_repo_packages};

#[derive(Debug, Clone)]
pub struct Package {
    pub is_installed: bool,
    pub repo: ItemRepo,
    pub size: u64, //bytes
    pub descipt: String,
    pub name: String,
    pub version: String,
}

impl Package {
    pub fn new(
        is_installed: bool,
        repo: ItemRepo,
        size: u64,
        descipt: String,
        name: String,
        version: String,
    ) -> Self {
        Self {
            is_installed,
            repo,
            size,
            descipt,
            name,
            version,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ItemRepo {
    Core,
    Extra,
    Archlinuxcn,
    Multilib,
    AbsOther(String),
    AUR(usize),
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
    pub aur_search_block: bool,
    pub aur_tx: mpsc::UnboundedSender<Vec<Package>>,
    pub aur_rx: mpsc::UnboundedReceiver<Vec<Package>>,
    pub last_search: String,
    pub notice: String,
    pub aur_task: Option<JoinHandle<()>>,
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
        let (aur_tx, aur_rx) = mpsc::unbounded_channel();
        let app = Self {
            filtered: all_packages.clone(),
            exit: false,
            search: String::new(),
            selected_pack: 0,
            items: all_packages,
            list_state,
            selected_win: Window::Search,
            insert_mode: InsertMode {
                enabled: false,
                index: 0,
            },
            aur_search_block: false,
            aur_tx,
            aur_rx,
            last_search: String::new(),
            notice: String::new(),
            aur_task: None,
        };
        Ok(app)
    }

    pub fn update_filter_local(&mut self) {
        self.filtered = self
            .items
            .iter()
            .filter(|p| p.name.contains(&self.search))
            .cloned()
            .collect();
        if self.selected_pack >= self.filtered.len() {
            self.selected_pack = 0;
        }

        self.update_aur();
    }

    pub fn update_aur(&mut self) {
        if let Some(task) = self.aur_task.take() {
            task.abort();
        }
        let keyword = self.search.clone();
        let tx = self.aur_tx.clone();
        self.aur_task = Some(tokio::spawn(async move {
            sleep(Duration::from_millis(300)).await;
            let pkgs = get_aur_packages(keyword).await.unwrap_or_default();
            tx.send(pkgs).unwrap_or_default();
        }));
    }
}

impl From<&str> for ItemRepo {
    fn from(s: &str) -> Self {
        match s {
            "core" => ItemRepo::Core,
            "extra" => ItemRepo::Extra,
            "multilib" => ItemRepo::Multilib,
            "archlinuxcn" => ItemRepo::Archlinuxcn,
            "aur" => ItemRepo::AUR(0),
            other => ItemRepo::AbsOther(other.to_string()),
        }
    }
}
