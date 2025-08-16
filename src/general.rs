use std::{
    error::Error,
    path::{Path, PathBuf},
};

use crate::dot::{Dot, get_dots_db};

pub fn dot_exists(name: &str) -> Result<bool, Box<dyn Error>> {
    let db = get_dots_db()?;
    let exists = db.iter().any(|dot| dot.name == name);

    Ok(exists)
}

pub fn get_dot(name: &str) -> Result<Dot, Box<dyn Error>> {
    let db = get_dots_db()?;

    let dot = db
        .into_iter()
        .find(|dot| dot.name == name)
        .ok_or_else(|| "Failed to find dot".to_string())?;

    Ok(dot)
}

pub fn get_home_dir() -> Result<PathBuf, Box<dyn Error>> {
    Ok(dirs::home_dir().ok_or_else(|| "Failed to get home dir".to_string())?)
}

pub fn truncate_path<P: AsRef<Path>>(path: P) -> Result<PathBuf, Box<dyn Error>> {
    let path_str = path.as_ref().display().to_string();
    let home_dir_str = get_home_dir()?.display().to_string();
    let truncated_path_str = path_str.replace(&home_dir_str, "~");
    let truncated_path = PathBuf::from(&truncated_path_str);

    Ok(truncated_path)
}

pub fn resolve_path<P: AsRef<Path>>(path: P) -> Result<PathBuf, Box<dyn Error>> {
    let home_str = get_home_dir()?.display().to_string();
    let path_str = path.as_ref().display().to_string().replace("~", &home_str);
    let resolved_path = PathBuf::from(&path_str);

    Ok(resolved_path)
}
