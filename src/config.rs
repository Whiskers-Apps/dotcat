use std::{error::Error, fs, path::PathBuf, process::exit};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub configs_dir: PathBuf,
}

pub fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
    Ok(dirs::config_dir()
        .ok_or_else(|| "Failed to get config dir.".to_string())?
        .join("dotcat.toml"))
}

impl Config {
    pub fn get() -> Config {
        match parse_config() {
            Ok(config) => config,
            Err(_) => {
                println!("Please setup the configs dir. You can use (--setup) to do that");
                exit(1)
            }
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let content = toml::to_string_pretty(self)?;
        Ok(fs::write(&get_config_path()?, &content)?)
    }
}

pub fn parse_config() -> Result<Config, Box<dyn Error>> {
    let bytes = fs::read(get_config_path()?)?;
    let config: Config = toml::from_slice(&bytes)?;

    Ok(config)
}
