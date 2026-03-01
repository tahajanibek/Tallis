use clap::{ArgAction, Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tallis")]
#[command(version = "mk-1.0")]
#[command(author = "tahajanibek")]
#[command(about = "2026 🄯 Tallis - Advanced batch image processing and OCR-based renaming program")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Forge(ForgeArgs),
    Destiny(DestinyArgs),
    Quozart7(Quozart7Args),
}

#[derive(Args)]
pub struct ForgeArgs {
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub jpg: bool,

    #[arg(short, long, default_value_t = false)]
    pub top: bool,

    pub prefix: String,
    pub directory: String,

    #[arg(short, long, default_value_t = false, help = "%90 CPU usage")]
    pub omega: bool,
}

#[derive(Args)]
pub struct DestinyArgs {
    pub extension: String,

    pub prefix: String,

    pub directory: String,
}

#[derive(Args)]
pub struct Quozart7Args {
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub jpg: bool,

    #[arg(short, long, default_value_t = false)]
    pub top: bool,

    pub prefix: String,
    pub directory: String,

    pub output_dir: Option<String>,

    #[arg(short, long, default_value_t = false, help = "%90 CPU usage")]
    pub omega: bool,
}
