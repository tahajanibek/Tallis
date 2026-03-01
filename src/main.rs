mod clie;
mod storme;
mod commanders {
    pub mod destiny;
    pub mod forge;
    pub mod quozart7;
}

use clap::Parser;
use clie::{Cli, Commands};
use std::io;

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Forge(args) => commanders::forge::run(args)?,
        Commands::Destiny(args) => commanders::destiny::run(args)?,
        Commands::Quozart7(args) => commanders::quozart7::run(args)?,
    }

    Ok(())
}
