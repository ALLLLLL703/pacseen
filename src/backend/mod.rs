use alpm::Alpm;
use std::{collections::HashSet, error::Error};

use crate::objects::stat::{ItemRepo, Package};

pub fn load_repo_packages() -> Result<Vec<Package>, Box<dyn Error>> {
    let alpm = Alpm::new("/", "/var/lib/pacman")?;

    let mut packages = Vec::new();
    let installed: HashSet<String> = alpm
        .localdb()
        .pkgs()
        .iter()
        .map(|p| p.name().to_string())
        .collect();

    for repo in alpm.syncdbs() {
        let repo_name = repo.name();
        for pkg in repo.pkgs() {
            let pack = Package::new(
                installed.contains(pkg.name()),
                ItemRepo::from(repo_name),
                pkg.size() as u64 / 1024,
                pkg.desc().unwrap_or("None").to_string(),
                pkg.name().to_string(),
            );

            packages.push(pack);
        }
    }

    packages.sort_by(|a, b| a.name.cmp(&b.name));
    packages.dedup_by(|a, b| a.name == b.name);

    Ok(packages)
}
