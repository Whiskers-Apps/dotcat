use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Set the directory where all the dots should be")]
    Setup {
        #[arg(help = "The folder where the dots should be")]
        path: PathBuf,
    },

    #[command(about = "Add a dot and symlink it")]
    Link {
        #[arg(help = "The directory/file to link")]
        path: PathBuf,

        #[arg(help = "The dot name. If not specified it will prompt one")]
        name: Option<String>,
    },

    #[command(about = "Remove a dot and it's symlink")]
    Unlink {
        #[arg(help = "The dot name")]
        name: String,
    },

    #[command(about = "Links all the dots in the dots database")]
    LinkDB {},

    #[command(about = "Unlinks all the dots in the dots database")]
    UnlinkDB {},

    #[command(about = "Migrate the dots to another location")]
    Migrate {
        #[arg(help = "The new dots directory")]
        path: PathBuf,
    },

    #[command(about = "List the active symlinks")]
    List {},
}
