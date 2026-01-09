use std::{error::Error, io};

use crate::backend::load_repo_packages;
use crate::objects::stat::Package;

#[test]
fn main1() -> Result<(), Box<dyn Error>> {
    let packs = load_repo_packages()?;
    for pack in packs {
        println!("{}", pack.name);
    }
    Ok(())
}
