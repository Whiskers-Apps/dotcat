use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Set the directory containing the dots")]
    SetDir {
        #[arg(help = "The directory path")]
        path: PathBuf,
    },

    #[command(about = "Move the dots directory to another")]
    MigrateDir {
        #[arg(help = "The directory path")]
        path: PathBuf,
    },

    #[command(about = "Add a dot")]
    Add {
        #[arg(help = "The dot id")]
        id: String,
        #[arg(help = "The dot path")]
        path: PathBuf,
    },

    #[command(about = "Remove a dot")]
    Remove {
        #[arg(help = "The dot id")]
        id: String,
    },

    #[command(about = "Rename a dot")]
    Rename {
        #[arg(help = "The dot id")]
        id: String,

        #[arg(help = "The new dot id")]
        new_id: String,
    },

    #[command(about = "Links all the dots in the config")]
    LinkDots {},

    #[command(about = "Unlinks all the dots in the config")]
    UnlinkDots {},
}
