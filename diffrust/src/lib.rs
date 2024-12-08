use std::error::Error;
use std::fs;
use std::path::PathBuf;

use serde_json::Value;

use args::Config;
use core::model::{self, Collection};

pub mod args;
mod core;

mod fsutil {
    use std::fs;
    use std::path::Path;
    use std::time::SystemTime;

    pub fn get_last_modified_time(path: &Path) -> Result<SystemTime, std::io::Error> {
        fs::metadata(path)?.modified()
    }
}

/// Attempts to parse content as a configuration storing result into collection
fn parse_config(content: &str, collection: &mut Collection) -> serde_json::Result<()> {
    let v: Value = serde_json::from_str(content)?;
    if let Some(name) = v.get("name").and_then(|n| n.as_str()) {
        collection.name = name.to_string();
    }
    Ok(())
}

pub fn open_or_create_config(root: &PathBuf) -> Collection {
    let config = root.join(".diffrust.conf");

    let mut collection = Collection {
        name: String::from(""),
        root: root.clone(),
        db: None,
        root_dir: None,
    };
    if config.is_file() {
        if let Ok(content) = fs::read_to_string(&config) {
            let _ = parse_config(&content, &mut collection);
            collection.db = Some(config);
        }
    }
    collection
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("{:?}", config.path);
    println!("Welcome to FS-Diff compare 🔄` file system directories 📁");
    let mut collection: model::Collection = open_or_create_config(&config.path);
    match collection.name.as_str() {
        "" => println!("Unnamed collection opened at"),
        _ => println!("Opened {} collection at", collection.name)
    }
    collection.scan()?;
    println!("{collection:?}");

    // DEV CODE BELOW
    println!("{:?}", fsutil::get_last_modified_time(collection.root.as_path()));
    Ok(())
}