use std::{error::Error, fs, os::unix::fs::symlink, path::PathBuf, process::exit};

use inquire::Text;

use crate::{
    config::Config,
    dot::{Dot, get_dots_db, write_dots_db},
    general::{dot_exists, resolve_path, truncate_path},
    setup::check_setup,
};

pub fn on_link(original_path: PathBuf, name: Option<String>) -> Result<(), Box<dyn Error>> {
    check_setup()?;

    let original_path = original_path.canonicalize()?;

    // User Input
    if !original_path.exists() {
        println!("Invalid path");
        exit(1);
    }

    let name = match name {
        Some(name) => name,
        None => Text::new("What's the dot name?").prompt()?,
    };

    if dot_exists(&name)? {
        println!("Dot already exists");
        exit(1);
    }

    // Add Dot
    let mut db = get_dots_db()?;

    let is_dir = original_path.is_dir();

    let target_path = if is_dir {
        Config::get().configs_dir.join(&name)
    } else {
        let path = original_path.clone();

        let file_name = path
            .file_name()
            .ok_or_else(|| "Failed to get file name".to_string())?;

        Config::get().configs_dir.join(&name).join(file_name)
    };

    let target_path_truncated = truncate_path(&target_path)?;
    let original_path_truncated = truncate_path(&original_path)?;

    let dot = Dot {
        name,
        is_dir,
        target_path: target_path_truncated,
        original_path: original_path_truncated,
    };

    db.push(dot.clone());

    write_dots_db(&db)?;

    if is_dir {
        fs::create_dir_all(&target_path)?;
    } else {
        fs::create_dir_all(
            &target_path
                .parent()
                .ok_or_else(|| "Failed to get parent dir".to_string())?,
        )?;
    }

    fs::rename(&original_path, &target_path)?;

    link_dot(&dot)?;

    Ok(())
}

pub fn on_link_db() -> Result<(), Box<dyn Error>> {
    check_setup()?;

    for dot in get_dots_db()? {
        link_dot(&dot)?;
    }

    Ok(())
}

pub fn link_dot(dot: &Dot) -> Result<(), Box<dyn Error>> {
    let original_path = resolve_path(&dot.original_path)?;
    let target_path = resolve_path(&dot.target_path)?;

    Ok(symlink(&target_path, &original_path)?)
}
