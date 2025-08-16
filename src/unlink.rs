use std::{error::Error, fs, process::exit};

use crate::{
    dot::{Dot, get_dots_db, write_dots_db},
    general::{dot_exists, get_dot, resolve_path},
    setup::check_setup,
};

pub fn on_unlink(name: &str) -> Result<(), Box<dyn Error>> {
    check_setup()?;

    if !dot_exists(name)? {
        println!("Dot not found");
        exit(1);
    }

    let dot = get_dot(name)?;

    Ok(unlink_dot(&dot)?)
}

pub fn on_unlink_db() -> Result<(), Box<dyn Error>> {
    check_setup()?;

    for dot in get_dots_db()? {
        unlink_dot(&dot)?;
    }

    Ok(())
}

pub fn unlink_dot(dot: &Dot) -> Result<(), Box<dyn Error>> {
    let original_path = resolve_path(dot.clone().original_path)?;
    let target_path = resolve_path(dot.clone().target_path)?;

    fs::remove_dir_all(&original_path)?;
    fs::rename(&target_path, &original_path)?;

    if !dot.is_dir {
        let parent = target_path.clone();
        let parent = parent
            .parent()
            .ok_or_else(|| "Failed to get parent".to_string())?;

        fs::remove_dir_all(&parent)?;
    }

    let db: Vec<Dot> = get_dots_db()?
        .into_iter()
        .filter(|d| d.name != dot.name)
        .collect();

    Ok(write_dots_db(&db)?)
}
