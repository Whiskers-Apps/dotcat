use std::{
    fs::{self, read},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub dots_dir: Option<String>,
    pub dots: Vec<Dot>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dot {
    pub id: String,
    pub source: String,
    pub target: String,
}

pub fn encode_path(path: &Path) -> String {
    let home_str = dirs::home_dir()
        .expect("Failed to get home dir")
        .display()
        .to_string();

    let path_str = path.display().to_string();

    return path_str.replace(&home_str, "~");
}

pub fn decode_path(path: &str) -> PathBuf {
    let home_str = dirs::home_dir()
        .expect("Failed to get home dir")
        .display()
        .to_string();

    let path_str = path.replace("~", &home_str);

    return PathBuf::from(&path_str);
}

pub fn get_config() -> Config {
    let config_path = dirs::config_dir()
        .expect("Failed to get config directory")
        .join("dotcat.json");

    let empty_config = Config {
        dots_dir: None,
        dots: vec![],
    };

    if !config_path.exists() {
        write_config(&empty_config);
        return empty_config;
    }

    let bytes = match read(&config_path) {
        Ok(bytes) => bytes,
        Err(_) => {
            log::warn!("Using default config since bytes couldn't be read");
            return empty_config;
        }
    };

    return match serde_json::from_slice(&bytes) {
        Ok(config) => config,
        Err(_) => empty_config,
    };
}

pub fn write_config(config: &Config) {
    let config_path = dirs::config_dir()
        .expect("Failed to get config directory")
        .join("dotcat.json");

    let bytes = serde_json::to_vec_pretty(&config).expect("Failed to convert config to json");

    fs::write(&config_path, &bytes).expect("Failed to write config");
}

pub fn dot_exists(id: &str) -> bool {
    let dots = get_config().dots;
    return dots.iter().any(|dot| dot.id == id);
}

pub fn add_dot(dot: &Dot) {
    let mut config = get_config();
    let mut dots = config.dots.clone();

    dots.push(dot.to_owned());
    config.dots = dots;

    write_config(&config);
}

pub fn remove_dot(id: &str) {
    let mut config = get_config();

    let dots = config
        .dots
        .clone()
        .into_iter()
        .filter(|dot| dot.id != id)
        .collect::<Vec<Dot>>();

    config.dots = dots;

    write_config(&config);
}
