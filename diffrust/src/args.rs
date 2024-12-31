use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Diff for the File System", long_about = None)]
pub struct Config {
    /// The path to the opened collection
    pub path: PathBuf,
}

impl Config {
    pub fn build() -> Config {
        Config::parse()
    }
}
