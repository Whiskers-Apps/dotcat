use std::error::Error;

use clap::Parser;
use cli::Cli;
use link::{on_link, on_link_db};
use list::on_list;
use migrate::on_migrate;
use setup::on_setup;
use unlink::{on_unlink, on_unlink_db};

mod cli;
mod config;
mod dot;
mod general;
mod link;
mod list;
mod migrate;
mod setup;
mod unlink;

fn main() -> Result<(), Box<dyn Error>> {
    match Cli::parse().command {
        cli::Commands::Setup { path } => {
            on_setup(path)?;
        }
        cli::Commands::Link { path, name } => {
            on_link(path, name)?;
        }
        cli::Commands::Unlink { name } => {
            on_unlink(&name)?;
        }
        cli::Commands::LinkDB {} => {
            on_link_db()?;
        }
        cli::Commands::UnlinkDB {} => {
            on_unlink_db()?;
        }
        cli::Commands::Migrate { path } => {
            on_migrate(path)?;
        }
        cli::Commands::List {} => {
            on_list()?;
        }
    }

    Ok(())
}
