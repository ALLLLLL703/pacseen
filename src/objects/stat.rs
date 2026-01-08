use std::alloc::Global;
use std::error::Error;

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

#[derive(Debug, Default)]
pub struct App {
    items: Vec<Package>,
    exit: bool,
    filtered: Vec<Package>,
    search: String,
    selected: usize,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let all_packages = load_repo_packages()?;

        let app = Self {
            filtered: all_packages.clone(),
            exit: false,
            search: String::new(),
            selected: 0,
            items: all_packages,
        };
        Ok(app)
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
