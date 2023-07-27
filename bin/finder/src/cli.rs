use clap::Parser;
use std::path::PathBuf;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct AppArgs {
    /// Name of the person to greet
    #[arg(short, long)]
    pub name: PathBuf,

    /// Number of times to greet
    #[arg(long, default_value_t = 1)]
    pub low: u32,

    /// Number of times to greet
    #[arg(long, default_value_t = 1)]
    pub high: u32,
}
