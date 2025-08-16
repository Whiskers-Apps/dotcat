use std::{error::Error, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dot {
    pub name: String,
    pub is_dir: bool,
    pub target_path: PathBuf,
    pub original_path: PathBuf,
}

fn get_dots_db_path() -> Result<PathBuf, Box<dyn Error>> {
    let configs_dir = Config::get().configs_dir;
    let file_path = configs_dir.join("dots-db.json");

    Ok(file_path)
}

pub fn write_dots_db(db: &[Dot]) -> Result<(), Box<dyn Error>> {
    let content = serde_json::to_vec_pretty(&db)?;
    fs::write(&get_dots_db_path()?, &content)?;

    Ok(())
}

pub fn get_dots_db() -> Result<Vec<Dot>, Box<dyn Error>> {
    let path = get_dots_db_path()?;
    let content = fs::read(&path)?;
    let db: Vec<Dot> = serde_json::from_slice(&content)?;

    Ok(db)
}
