use std::{process::Command, time::Duration};

use serde::Deserialize;
use tokio::time::timeout;

use crate::objects::stat::{ItemRepo, Package};

#[derive(Debug, Deserialize)]
struct AurSearchResponse {
    #[serde(rename = "resultcount")]
    _result_count: usize,
    results: Vec<AurRaw>,
}

#[derive(Debug, Deserialize)]
struct AurRaw {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Version")]
    version: String,
    #[serde(rename = "Description")]
    description: Option<String>,
    #[serde(rename = "NumVotes")]
    num_votes: Option<u64>,
    #[serde(rename = "Maintainer")]
    _maintainer: Option<String>,
}

async fn search_aur(keyword: &String) -> Result<Vec<AurRaw>, Box<dyn std::error::Error>> {
    let url = format!("https://aur.archlinux.org/rpc/v5/search/{}", keyword);
    let client = reqwest::Client::new();
    let resp = timeout(Duration::from_secs(10), client.get(&url).send())
        .await??
        .error_for_status()?
        .json::<AurSearchResponse>()
        .await?;
    Ok(resp.results)
}

impl From<AurRaw> for Package {
    fn from(value: AurRaw) -> Self {
        Self {
            is_installed: false,
            name: value.name,
            version: value.version,
            size: 0,
            repo: ItemRepo::AUR(value.num_votes.unwrap_or(0) as usize),
            descipt: value.description.unwrap_or("".to_string()),
        }
    }
}

async fn _is_aur_installed(pkg: &str) -> bool {
    Command::new("pacman")
        .args(["-Q", pkg])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub async fn get_aur_packages(keyword: String) -> Result<Vec<Package>, Box<dyn std::error::Error>> {
    let mut aur_raw = search_aur(&keyword).await.unwrap_or_default();
    let mut aur_pkgs: Vec<Package> = Vec::new();
    while aur_raw.is_empty() && !keyword.is_empty() {
        aur_raw = search_aur(&keyword).await.unwrap_or_default();
    }
    for raw in aur_raw {
        aur_pkgs.push(Package::from(raw));
    }
    Ok(aur_pkgs)
}
