use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(name = "FS Diff")]
#[command(version = "0.1")]
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