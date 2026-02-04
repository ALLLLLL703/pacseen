use alpm::{Alpm, SigLevel};
use std::{collections::HashSet, error::Error, io, process::Command};

use crate::objects::stat::{App, ItemRepo, Package};
pub mod aur;

pub fn load_repo_packages() -> Result<Vec<Package>, Box<dyn Error>> {
    let alpm = Alpm::new("/", "/var/lib/pacman")?;

    let mut packages = Vec::new();
    let installed: HashSet<String> = alpm
        .localdb()
        .pkgs()
        .iter()
        .map(|p| p.name().to_string())
        .collect();
    alpm.register_syncdb("core", SigLevel::USE_DEFAULT).unwrap();
    alpm.register_syncdb("extra", SigLevel::USE_DEFAULT)
        .unwrap();
    alpm.register_syncdb("multilib", SigLevel::USE_DEFAULT)
        .unwrap();
    alpm.register_syncdb("archlinuxcn", SigLevel::USE_DEFAULT)
        .unwrap();
    alpm.register_syncdb("arch4edu", SigLevel::USE_DEFAULT)
        .unwrap();

    for repo in alpm.syncdbs() {
        let repo_name = repo.name();
        for pkg in repo.pkgs() {
            let pack = Package::new(
                installed.contains(pkg.name()),
                ItemRepo::from(repo_name),
                pkg.size() as u64 / 1024,
                pkg.desc().unwrap_or("None").to_string(),
                pkg.name().to_string(),
                pkg.version().to_string(),
            );

            packages.push(pack);
        }
    }

    packages.sort_by(|a, b| a.name.cmp(&b.name));
    packages.dedup_by(|a, b| a.name == b.name);

    Ok(packages)
}

impl App {
    pub fn install_pack(&mut self, index: usize) {
        let pack = &mut self.filtered[index];
        let cmd = "paru".to_string();
        if pack.is_installed {
            ratatui::restore();
            let mut child = Command::new(&cmd)
                .arg("-Rns")
                .arg(&pack.name)
                .spawn()
                .expect("failed to start a new process");
            let status = child.wait().unwrap();
            if status.success() {
                pack.is_installed == false;
            }

            return;
        }
        ratatui::restore();
        let mut child = Command::new(&cmd)
            .arg("-S")
            .arg(&pack.name)
            .spawn()
            .expect("failed to start a new process");
        let status = child.wait().unwrap_or_default();
        if status.success() {
            pack.is_installed = true;
        }
    }
}
