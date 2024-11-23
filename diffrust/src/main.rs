use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(name = "FS Diff")]
#[command(version = "0.1")]
#[command(about = "Diff for the File System", long_about = None)]
struct Cli {
    /// The path to the opened collection
    #[arg(long, short, default_value = "/")]
    path: PathBuf,
}

pub mod fsutil {
    use std::fs;
    use std::path::Path;
    use std::time::SystemTime;

    pub fn get_last_modified_time(path: &Path) -> Result<SystemTime, std::io::Error> {
        fs::metadata(path)?.modified()
    }
}

pub mod core {
    use std::fs;
    use std::io;
    use std::io::Read;
    use std::path::PathBuf;
    use serde_json::Value;
    use md5;

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

    #[derive(Debug)]
    pub struct Collection {
        /// Collection name
        pub name: String,
        /// The root path of the collection
        pub root: PathBuf,
        /// Path to db file. If None path is root/.diffrust.conf
        db: Option<PathBuf>,
        /// The root Dir struct. Can be None if not present or initialized.
        root_dir: Option<Dir>,
    }

    impl Collection {
        pub fn save(&self) -> Result<(), std::io::Error> {
            // let config_path = self.db.as_ref().unwrap_or(&self.root.join(".diffrust.conf"));
            // let json = serde_json::to_string(self)?;
            // fs::write(config_path, json)
            Ok(())
        }

        pub fn scan(&mut self) {
            let dir = self.root_dir.get_or_insert_with(|| {
                Dir {
                    files: Vec::new(),
                    path: self.root.clone(),
                }
            });
            if let Err(e) = dir.scan() {
                eprint!("Error on scanning root dir");
            }
        }
    }
    #[derive(Debug)]
    pub struct Dir {
        pub path: PathBuf,
        pub files: Vec<File>,
    }

    impl Dir {
        pub fn scan(&mut self) -> Result<(), io::Error> {
            for entry in fs::read_dir(self.path.as_path())? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    let mut file = fs::File::open(&path)?;
                    let mut contents = Vec::new();
                    file.read_to_end(&mut contents)?;
                    self.files.push(File {
                        path: PathBuf::from(path),
                        md5: md5::compute(&contents),
                    });
                }
            } 
            Ok(())
        }
    }

    #[derive(Debug)]
    pub struct File {
        /// File path
        path: PathBuf,
        /// The hash of the file
        md5: md5::Digest,
    }
}

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli.path);
    println!("Welcome to FS-Diff compare 🔄` file system directories 📁");
    let mut collection: core::Collection = core::open_or_create_config(&cli.path);
    match collection.name.as_str() {
        "" => println!("Unnamed collection opened at"),
        _ => println!("Opened {} collection at", collection.name)
    }
    collection.scan();
    println!("{collection:?}");

    // DEV CODE BELOW
    println!("{:?}", fsutil::get_last_modified_time(collection.root.as_path()));
}
