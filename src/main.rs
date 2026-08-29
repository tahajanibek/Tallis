mod clie;
mod indi;
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
    let dil = cli.dil;

    match cli.command {
        Commands::Forge(args) => commanders::forge::run(args, dil)?,
        Commands::Destiny(args) => commanders::destiny::run(args, dil)?,
        Commands::Quozart7(args) => commanders::quozart7::run(args, dil)?,
    }

    Ok(())
}
