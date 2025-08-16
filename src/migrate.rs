use std::{collections::HashMap, error::Error, fs, path::PathBuf, process::exit};

use crate::{
    config::Config, dot::get_dots_db, general::resolve_path, link::on_link, setup::check_setup,
    unlink::unlink_dot,
};

pub fn on_migrate(path: PathBuf) -> Result<(), Box<dyn Error>> {
    check_setup()?;

    if !path.exists() {
        println!("Invalid Path");
        exit(1);
    }

    Ok(migrate(path)?)
}

pub fn migrate(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let dots_db = get_dots_db()?;

    let mut paths = HashMap::new();

    for dot in &dots_db {
        paths.insert(dot.clone().name, resolve_path(dot.clone().original_path)?);
    }

    for dot in &dots_db {
        unlink_dot(dot)?;
    }

    let mut config = Config::get();

    fs::rename(
        config.configs_dir.clone().join("dots-db.json"),
        path.clone().join("dots-db.json"),
    )?;

    config.configs_dir = path;
    config.save()?;

    for (name, path) in paths {
        on_link(path, Some(name))?;
    }

    Ok(())
}
