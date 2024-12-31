use std::{collections::HashSet, error::Error};
use std::fs;
use std::path::PathBuf;

use chrono::{DateTime, Local};
use md5::Digest;
use serde_json::Value;

use args::Config;
use core::model::{self, Collection, ContentType, Dir};

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

    let mut collection = Collection::new();
    collection.root = root.clone(); // # TODO: should this be a "method" or a "property"?
    if config.is_file() {
        if let Ok(content) = fs::read_to_string(&config) {
            let _ = parse_config(&content, &mut collection);
            collection.db = Some(config);
        }
    }
    collection
}

fn print_welcome() {
    println!("\nWelcome to FS-Diff compare 🔄` file system directories 📁");
}

fn print_content(content: &Vec<ContentType>) {
    // TODO: still need to implement sorting
    let mut dirs: Vec<&Dir> = content.iter().filter_map(|item|
        if let ContentType::ContentDir(d) = item {
            Some(d)
        } else {
            None
        }
    ).collect();
    dirs.sort_unstable();

    let mut files: Vec<&core::model::File> = content.iter().filter_map(|item|
        if let ContentType::ContentFile(f) = item {
            Some(f)
        } else {
            None
        }
    ).collect();
    files.sort_unstable();

    // print directories first
    println!();
    for item in dirs.iter() {
        println!(" 📁 {:}", item.path.file_name().unwrap().to_str().unwrap());
    }

    // print files next
    let mut md5_set: HashSet<&Digest> = HashSet::new();
    for item in files.iter() {
        md5_set.insert(&item.md5);
        println!(" 🗄  {:} ({:?})", item.path.file_name().unwrap().to_str().unwrap(), &item.md5);
    }
    println!();
    println!("{} total directories", dirs.len());
    println!("{} total files ({} unique)", files.len(), md5_set.len());
    println!();
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    print_welcome();
    let mut collection: model::Collection = open_or_create_config(&config.path);
    match collection.name.as_str() {
        "" => println!(
            "Unnamed collection opened at {}",
            &config.path.to_str().unwrap_or("")
        ),
        _ => println!(
            "Opened {} collection at {}",
            collection.name,
            &config.path.to_str().unwrap_or("")
        ),
    }
    collection.scan()?;
    match collection.root_dir {
        Some(c) => {
            print_content(&c.content);
            let last_modified_time = fsutil::get_last_modified_time(collection.root.as_path())?;
            let local_time: DateTime<Local> = last_modified_time.into();
            println!("Last modified: {:}", local_time.format("%Y-%m-%d %H:%M:%S"));
        }
        _ => (),
    };
    Ok(())
}
