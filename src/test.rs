use std::{error::Error, time::Duration};

use tokio::{sync::mpsc, time::sleep};

use crate::{backend::aur::get_aur_packages, objects::stat::Package};

#[tokio::test]
pub async fn test_aur() -> Result<(), Box<dyn Error>> {
    let (mut tx, mut rx) = mpsc::unbounded_channel::<Vec<Package>>();

    let task = tokio::spawn(async move {
        let aur_pkgs = get_aur_packages("vim".to_string())
            .await
            .unwrap_or_default();
        let _ = tx.send(aur_pkgs);
    });

    loop {
        if let Ok(pkgs) = rx.try_recv() {
            for p in pkgs {
                println!("{}: {}", p.name, p.descipt);
            }
            break;
        }
        println!("ui tick");
        sleep(Duration::from_millis(50)).await;
    }

    Ok(())
}

pub fn poll_aur() {}
