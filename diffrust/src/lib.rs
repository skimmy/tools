use std::error::Error;
use std::fs;
use std::path::PathBuf;

use chrono::{DateTime, Local};
use serde_json::Value;

use args::Config;
use core::model::{self, Collection, ContentType};

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
    let mut dir_count = 0;
    let mut file_count = 0;
    // print directories first
    for item in content {
        match &item {
            ContentType::ContentDir(d) => {
                println!("📁 {:?}", d.path.file_name());
                dir_count += 1;
            }
            _ => ()
        }
        
    }

    // print files next
    for item in content {
        match &item {
            ContentType::ContentFile(f) => {
                println!("🗄  {:?} ({:?})", f.path.file_name(), f.md5);
                file_count += 1;
            }
            _ => ()
        }
    }
    println!("{dir_count} total directories");
    println!("{file_count} total files (?? duplicates)");
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    print_welcome();
    let mut collection: model::Collection = open_or_create_config(&config.path);
    match collection.name.as_str() {
        "" => println!("Unnamed collection opened at {}", &config.path.to_str().unwrap_or("")),
        _ => println!("Opened {} collection at {}", collection.name, &config.path.to_str().unwrap_or(""))
    }
    collection.scan()?;
    match collection.root_dir {
        Some(c) => {
            print_content(&c.content);
            let last_modified_time = fsutil::get_last_modified_time(collection.root.as_path())?;
            let local_time: DateTime<Local> = last_modified_time.into();
            println!("Last modified: {:}", local_time.format("%Y-%m-%d %H:%M:%S"));
        },
        _ => (),
    };
    Ok(())
}