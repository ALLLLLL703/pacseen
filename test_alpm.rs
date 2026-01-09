use alpm::{Alpm, SigLevel};

fn main() {
    // 测试不同的路径组合
    let test_paths = [
        ("/", "var/lib/pacman/"),
        ("/", "/var/lib/pacman/"),
        ("/", "/var/lib/pacman"),
        ("", "/var/lib/pacman"),
    ];
    
    for (root, dbpath) in test_paths {
        println!("Testing with root='{}', dbpath='{}'", root, dbpath);
        match Alpm::new(root, dbpath) {
            Ok(alpm) => {
                println!("  ALPM initialized successfully");
                println!("  Local packages count: {}", alpm.localdb().pkgs().len());
                
                // 尝试注册所有仓库
                let repos = ["core", "extra", "multilib", "archlinuxcn", "arch4edu"];
                for repo in repos.iter() {
                    if let Ok(_) = alpm.register_syncdb(repo, SigLevel::USE_DEFAULT) {
                        println!("  Registered repo: {}", repo);
                    } else {
                        println!("  Failed to register repo: {}", repo);
                    }
                }
                
                let total_packages: usize = alpm.syncdbs().map(|db| db.pkgs().len()).sum();
                println!("  Total packages in all repos: {}", total_packages);
            }
            Err(e) => {
                println!("  Failed to initialize ALPM: {}", e);
            }
        }
        println!();
    }
}
