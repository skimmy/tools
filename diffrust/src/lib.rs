use core::algorithm;
use std::fs;
use std::ops::Range;
use std::path::PathBuf;
use std::{collections::HashSet, error::Error};

use chrono::{DateTime, Local};
use colored::Colorize;
use md5::Digest;
use serde_json::Value;

use algorithm::dice_coefficient;
use args::Config;
use core::model::{self, Collection, ContentType, Dir};

pub mod args;
pub mod core;

mod fsutil {
    use std::fs;
    use std::path::Path;
    use std::time::SystemTime;

    pub fn get_last_modified_time(path: &Path) -> Result<SystemTime, std::io::Error> {
        fs::metadata(path)?.modified()
    }
}

enum Command {
    Show,
    Find,
    // Nop,
}

impl Command {
    fn from(config: &Config) -> Self {
        if let Some(_) = &config.find {
            return Command::Find;
        }
        Command::Show
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    print_welcome();
    let command = Command::from(&config);
    let mut collection: model::Collection = open_or_create_config(&config.path);
    // TODO: replace with a `print_collection_info`` function
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
    // TODO: If all commands require unwrapping root_dir, then do it once
    // propagating (returning) an error when something goes wrong
    match command {
        Command::Show => {
            if let Some(c) = collection.root_dir {
                print_content(&c);
            };
        }
        Command::Find => {
            if let Some(c) = collection.root_dir {
                // unwrap should be safe, if clap is used properly
                let pattern = &config.find.unwrap();
                let matches = find(&c, &pattern);
                print_find_matches(matches, &pattern);
            }
        }
    }

    Ok(())
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
    println!(
        "\nWelcome to {} compare 🔄 file system directories 📁",
        "DIFFRUST".cyan().bold()
    );
}

fn print_content(dir: &Dir) {
    let dirs = dir.sorted_dirs();
    let files = dir.sorted_files();

    // print directories first
    println!();
    for item in dirs.iter() {
        println!(" 📁 {:}", item.path.file_name().unwrap().to_str().unwrap());
    }

    // print files next counting unique md5's
    let mut md5_set: HashSet<&Digest> = HashSet::new();
    for item in files.iter() {
        md5_set.insert(&item.md5);
        println!(
            " 🗄  {:} ({:?})",
            item.path.file_name().unwrap().to_str().unwrap(),
            &item.md5
        );
    }
    println!("\n{} total directories", dirs.len());
    println!("{} total files ({} unique)\n", files.len(), md5_set.len());

    // finally, prints last modification time for dir
    let last_modified_time = fsutil::get_last_modified_time(dir.path.as_path()).unwrap();
    let local_time: DateTime<Local> = last_modified_time.into();
    println!("Last modified: {:}", local_time.format("%Y-%m-%d %H:%M:%S"));
}

fn print_find_matches(matches: Vec<&ContentType>, pattern: &str) {
    // Ideally this function would work like this
    // - iterate over the input
    // - for each input, extract matches
    // - format each extracted match
    // - print formatted match of the input
    // Notice that this would make the second parameter superfluous
    for m in matches {
        match m {
            ContentType::ContentDir(d) => {
                let path = d.path.as_path();
                let name = path.file_name().unwrap().to_str().unwrap();
                println!(" 📁 {}", name.to_string());
            }
            ContentType::ContentFile(f) => {
                let path = f.path.as_path();
                let name = path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace(pattern, pattern.to_string().red().to_string().as_str());
                println!(" 🗄  {}", name.to_string());
            }
            _ => (),
        }
    }
}

fn find<'a>(dir: &'a Dir, pattern: &str) -> Vec<&'a ContentType> {
    dir.content
        .iter()
        .filter(|f| match f {
            ContentType::ContentDir(d) => algorithm::substrings_in_name(&d.path, pattern).len() > 0,
            ContentType::ContentFile(f) => {
                algorithm::substrings_in_name(&f.path, pattern).len() > 0
            }
            _ => false,
        })
        .collect()
}

/// Returns a vector of ranges, entry i contains the range where the
/// pattern matched the input i. If no such match is found the position
/// contains None.
fn _substring_match_content(
    content: &Vec<ContentType>,
    _pattern: &str,
) -> Vec<Option<Range<usize>>> {
    content
        .iter()
        .map(|c| match c {
            ContentType::ContentDir(_) => None,
            ContentType::ContentFile(_) => None,
            _ => None,
        })
        .collect()
}

/// Returns a vector of (score, index) pairs indicating that input at
/// indicated index obtained the corresponding fuzzy matching score
/// (Dice coefficient is used) against the provided pattern.
fn _fuzzy_match_content(content: &Vec<ContentType>, pattern: &str) -> Vec<(f64, usize)> {
    content
        .iter()
        .enumerate()
        .map(|(i, val)| {
            let text = match &val {
                ContentType::ContentFile(f) => f.path.to_str().unwrap(),
                ContentType::ContentDir(d) => d.path.to_str().unwrap(),
                _ => "",
            };
            (dice_coefficient(text, pattern), i)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_from_config() {
        let mut config = Config {
            path: PathBuf::from("/tmp"),
            find: Some(String::from("*document*.txt")),
        };
        let command = Command::from(&config);
        assert!(matches!(command, Command::Find));
        config.find = None;
        let command = Command::from(&config);
        assert!(matches!(command, Command::Show));
    }

    #[test]
    fn substring_matching() {
        let content = vec![
            ContentType::ContentFile(model::File {
                path: PathBuf::from("/tmp/a/abc.txt"),
                md5: md5::compute(b"abc"),
            }),
            ContentType::ContentDir(model::Dir {
                path: PathBuf::from("Documents/books/"),
                content: vec![],
            }),
            ContentType::ContentFile(model::File {
                path: PathBuf::from("~/Abbey.jpg"),
                md5: md5::compute(b"123"),
            }),
            ContentType::ContentFile(model::File {
                path: PathBuf::from("lab/test/result.csv"),
                md5: md5::compute(b"lab"),
            }),
            ContentType::ContentFile(model::File {
                path: PathBuf::from("~/a/b.txt"),
                md5: md5::compute(b"version=0.1\n"),
            }),
            ContentType::ContentDir(model::Dir {
                path: PathBuf::from("abracadabra.abb"),
                content: vec![],
            }),
        ];
        let pattern = "ab";
        let matched = _substring_match_content(&content, pattern);
        // check number of matched items
        assert_eq!(
            matched.len(),
            content.len(),
            "Returned length don't match input length"
        );
        // check an exact-case match
        assert_eq!(
            matched[0],
            Some(Range { start: 6, end: 9 }),
            "Match not found"
        );
        // check a mismatch
        assert!(matches!(matched[1], None), "Found wrong matching");
        // check a mixed-case match
        assert_eq!(
            matched[2],
            Some(Range { start: 2, end: 3 }),
            "Match with mixed case not found",
        );
        // check match not in name part of the path
        assert!(
            matches!(matched[3], None),
            "Found match in prefix not in name"
        );
        // check match that crosses path parts
        assert!(
            matches!(matched[4], None),
            "Found wrong match crossing path parts"
        );
        // Check multiple matches (expect the first)
        assert_eq!(
            matched[5],
            Some(Range { start: 0, end: 1 }),
            "Found wrong match in multiple"
        )
    }
}
