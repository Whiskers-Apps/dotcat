use std::{fs, path::Path, process::Command};

pub fn link(source: &Path, target: &Path) {
    Command::new("ln")
        .args([
            "-s",
            source.display().to_string().as_str(),
            target.display().to_string().as_str(),
        ])
        .spawn()
        .expect("Failed to link file");
}

pub fn unlink(link_path: &Path) {
    if link_path.is_file() {
        fs::remove_file(&link_path).expect("Failed to remove link");
        return;
    }

    if link_path.is_dir() {
        fs::remove_dir_all(&link_path).expect("Failed to remove link");
    }
}
