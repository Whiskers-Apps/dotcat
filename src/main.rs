use std::{
    error::Error,
    fs::{self, canonicalize, remove_dir, remove_file},
    path::PathBuf,
    process::exit,
};

use clap::Parser;
use cli::Cli;
use colorize::AnsiColor;

use crate::{
    config::{
        Dot, add_dot, assert_config, decode_path, dot_exists, encode_path, get_config, remove_dot,
        write_config,
    },
    link::{link, unlink},
};

mod cli;
mod config;
mod link;

fn main() -> Result<(), Box<dyn Error>> {
    let mut config = get_config();

    match Cli::parse().command {
        cli::Commands::SetDir { path } => {
            let path = match canonicalize(&path) {
                Ok(path) => path,
                Err(_) => {
                    println!("Invalid Path. Please choose a directory path.");
                    exit(1);
                }
            };

            if !path.is_dir() {
                println!("Path must be a directory.");
                exit(1);
            }

            let encoded_path = encode_path(&path);
            config.dots_dir = Some(encoded_path.clone());

            write_config(&config);

            println!("Dots directory set to: {encoded_path}");
        }
        cli::Commands::MigrateDir { path } => {
            assert_config(&config);

            let path = match canonicalize(&path) {
                Ok(path) => path,
                Err(_) => {
                    println!("Invalid Path. Please choose a directory path.");
                    exit(1);
                }
            };

            if !path.is_dir() {
                println!("Path must be a directory.");
                exit(1);
            }

            let encoded_path = encode_path(&path);
            let old_dots_dir = config.dots_dir.clone();

            config.dots_dir = Some(encoded_path.clone());

            for dot in config.dots.clone() {
                let decoded_target_path = decode_path(&dot.target);
                let new_source_dir = path.clone().join(&dot.id);
                let old_source_dir = decode_path(&old_dots_dir.clone().unwrap()).join(&dot.id);
                let source_path = path.clone().join(&dot.source);

                if !source_path.parent().unwrap().exists() {
                    fs::create_dir_all(&&new_source_dir.parent().unwrap())
                        .expect("Failed to create dot dir");
                }

                unlink(&decoded_target_path);

                fs::rename(&old_source_dir, &new_source_dir).expect("Failed to move dot dir");

                link(&source_path, &decoded_target_path);
            }

            write_config(&config);

            println!("Migrated dots to {}", &encoded_path);
        }
        cli::Commands::Add { id, path } => {
            assert_config(&config);

            let exists = dot_exists(&id);

            if exists {
                println!("A dot already has the same id.");
                exit(1);
            }

            let full_target_path = match canonicalize(&path) {
                Ok(path) => path,
                Err(_) => {
                    println!("Invalid Path. Please write a directory path.");
                    exit(1);
                }
            };

            let encoded_target_path = encode_path(&full_target_path);
            let relative_source_path = if full_target_path.is_dir() {
                id.clone()
            } else {
                format!("{id}/{}", path.file_name().unwrap().display().to_string())
            };

            let dot = Dot {
                id: id.clone(),
                source: relative_source_path.clone(),
                target: encoded_target_path.clone(),
            };

            let dots_dir = decode_path(&config.dots_dir.unwrap());
            let source_full_path = PathBuf::from(dots_dir).join(&relative_source_path);

            if !source_full_path.parent().unwrap().exists() {
                fs::create_dir_all(&source_full_path.parent().unwrap())
                    .expect("Failed to create dot dir");
            }

            if source_full_path.is_dir() {
                fs::create_dir_all(&source_full_path).expect("Failed to create dot dir");
            }

            fs::rename(&full_target_path, &source_full_path).expect("Failed to move content");

            add_dot(&dot);

            link(&source_full_path, &full_target_path);

            println!(
                "Added {} with path {}",
                &id.bold(),
                &full_target_path.display().to_string().bold()
            );
        }
        cli::Commands::Remove { id } => {
            assert_config(&config);

            if !dot_exists(&id) {
                println!("Dot not found.");
                exit(1);
            }

            let dot = config.dots.into_iter().find(|dot| dot.id == id).unwrap();
            let dots_dir = decode_path(&config.dots_dir.unwrap());
            let source_path = dots_dir.clone().join(&dot.source);
            let target_path = decode_path(&dot.target);

            remove_dot(&id);

            unlink(&target_path);

            fs::rename(&source_path, &target_path)
                .expect("Failed to copy dot to original location.");

            if source_path.is_dir() {
                fs::remove_dir_all(dots_dir.join(id.clone()))
                    .expect("Failed to remove dot directory.");
            }

            println!("Removed [{id}] from dots.");
        }
        cli::Commands::Rename { id, new_id } => {
            assert_config(&config);

            if !dot_exists(&id) {
                println!("Could not find dot with that id.");
                exit(1);
            }

            if dot_exists(&new_id) {
                println!("A dot already exists with the same id.");
                exit(1);
            }

            let mut new_dot = config
                .dots
                .clone()
                .into_iter()
                .find(|dot| dot.id == id)
                .unwrap();

            let previous_source = new_dot.source.clone();

            new_dot.id = new_id.to_owned();

            let is_file = new_dot.source.contains("/");

            let new_source = if is_file {
                let mut source = new_dot.source.clone().strip_prefix(&id).unwrap().to_owned();
                source = format!("{}{}", &new_id, &source);
                source
            } else {
                new_id.clone()
            };

            new_dot.source = new_source;

            let new_dots: Vec<Dot> = config
                .dots
                .clone()
                .into_iter()
                .map(|dot| if dot.id == id { new_dot.clone() } else { dot })
                .collect();

            config.dots = new_dots;

            write_config(&config);

            let mut full_previous_source_path =
                PathBuf::from(&config.dots_dir.as_ref().unwrap()).join(&previous_source);

            full_previous_source_path =
                decode_path(&full_previous_source_path.display().to_string());

            let mut full_source_path =
                PathBuf::from(&config.dots_dir.as_ref().unwrap()).join(&new_dot.source);

            full_source_path = decode_path(&full_source_path.display().to_string());

            let full_target_path = decode_path(&new_dot.target);

            unlink(&decode_path(&new_dot.target));

            if is_file {
                fs::rename(
                    &full_previous_source_path.parent().unwrap(),
                    &full_source_path.parent().unwrap(),
                )
                .expect("Failed to rename dot directory");
            } else {
                fs::rename(&full_previous_source_path, &full_source_path)
                    .expect("Failed to rename dot directory");
            }

            link(&full_source_path, &full_target_path);

            println!("Renamed {} to {}", &id, &new_id);
        }
        cli::Commands::LinkDots {} => {
            assert_config(&config);

            for dot in config.dots {
                let full_target_path = decode_path(&dot.target);
                let full_source_path =
                    decode_path(&config.dots_dir.clone().unwrap()).join(&dot.source);

                if full_target_path.exists() {
                    if full_target_path.is_dir() {
                        let _ = remove_dir(&full_target_path);
                    } else {
                        let _ = remove_file(&full_target_path);
                    }
                }

                link(&full_source_path, &full_target_path);
            }

            println!("Linked all dots in the config.");
        }
        cli::Commands::UnlinkDots {} => {
            assert_config(&config);

            for dot in config.dots {
                let full_target_path = decode_path(&dot.target);
                unlink(&full_target_path);
            }

            println!("Unlinked all dots in the config.")
        }
    }

    Ok(())
}
