use std::{error::Error, path::PathBuf, process::exit};

use crate::{
    config::{Config, parse_config},
    dot::{get_dots_db, write_dots_db},
};

pub fn on_setup(path: PathBuf) -> Result<(), Box<dyn Error>> {
    if !path.exists() {
        println!("Invalid Path");
        exit(1);
    }

    let path = path.canonicalize()?;

    Config {
        configs_dir: path.to_owned(),
    }
    .save()?;

    println!("Dots directory set to {}", &path.display());

    Ok(())
}

pub fn check_setup() -> Result<(), Box<dyn Error>> {
    match parse_config() {
        Ok(config) => {
            if !config.configs_dir.exists() {
                println!("Dots directory doesn't exist");
                exit(1);
            }

            match get_dots_db() {
                Ok(_) => {}
                Err(_) => write_dots_db(&vec![])?,
            }

            Ok(())
        }
        Err(_) => {
            println!("Configuration is required. Use setup command along with a path");
            exit(1);
        }
    }
}
