use clap::Parser;
use std::ffi::OsStr;
// use md5::{Md5, Digest};
// use log::info;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "FS Diff")]
#[command(version = "0.1")]
#[command(about = "Diff for the File System", long_about = None)]
struct Cli {
    #[arg(long, short, default_value = "/")]
    path: String,
}

fn find_dirs(dir: &Path, dir_list: &mut Vec<Dir>) -> io::Result<()> {
    if dir.is_dir() {
        // TODO: We need some code to add the current directory di `dir_list`
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                // TODO: compute name from path
                let d = Dir {
                    path: path.clone(),
                    name: match path.file_name() {
                        Some(c) => c,
                        _ => OsStr::new(""),
                    },
                    dir_type: (),
                    files: Vec::new(),
                };
                dir_list.push(d);
                // If it's a directory, recurse into it
                find_dirs(&path, dir_list)?;
            }
        }
    }
    Ok(())
}

// fn bytes_to_hex(bytes: &[u8]) -> String {
//     let mut md5 = String::new();
//     for byte in bytes {
//         md5.push_str(&format!("{byte:02x}"));
//     }
//     md5
// }

/// A structure representing a monitored directory
#[derive(Debug)]
struct Dir {
    /// The directory path
    path: PathBuf,
    /// The name of the directory
    name: OsStr,
    /// Will be a future custom Enum
    dir_type: (),
    /// The files present in the directory
    files: Vec<File>,
}

impl Dir {
    /// updates the content of the `files` property by accessing the filesystem
    /// The updates happens by:
    /// - adding non-existing files
    /// - removing files not present anymore on the filesystem
    /// - updated files present
    /// Files in the property and in the filesystem are compared by name.
    fn update_files(&self) -> io::Result<()> {
        for item in fs::read_dir(&self.path)? {
            println!("{item:?}");
        }
        Ok(())
    }
}

/// A structure representing a tracked file
#[derive(Debug)]
struct File {
    /// The file path relative to the 'root'
    rpath: PathBuf,
    /// The file name
    name: String,
    /// File extension
    ext: String,
    /// The has of the file
    md5: [u8; 16],
}

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli.path);
    println!("Welcome to FS-Diff compare file system directories");
    // let p = "/Users/skimmy/src/aoc";
    let mut dirs: Vec<Dir> = Vec::new();
    match find_dirs(Path::new(&cli.path), &mut dirs) {
        Err(e) => println!("There was some Error: {e}"),
        _ => {
            for d in dirs {
                let _ = d.update_files();

                println!(
                    "{}\n  {} ({} files)",
                    d.path.to_str().unwrap_or("[EMPTY PATH]"),
                    d.name.to_str().unwrap_or("[NO NAME]"),
                    d.files.len()
                );
            }
        }
    }

    // let f = File {
    //     rpath: PathBuf::from(p),
    //     name: String::from("abc"),
    //     md5: [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16],
    //     ext: String::from("txt"),
    // };
    // println!("{}", f.to_string());
    // println!("{f:?}");

    // let mut hasher = Md5::new();
    // hasher.update("Hello World!");
    // println!("{:x?}", hasher.finalize());

    // println!("{:?}", fs::metadata("/tmp/text.html"));
}
/*
We are now at the point where our code (find_dirs functions) creates a list
of directories starting from a root path. The function needs further improvements
(see above), but we are ready for the next step.

The next step is to scan directories one-by-one and looking for files within
them. For each file that we find we create a new File instance and put it into
the `files` field of the Dir struct. During this creation we should compute
all the useful information like `last_update` and `md5` of the file.

Improvements:
- Start adding the last_update field to File and Dir (this requires finding a
good way to manage date and time in Rust).

*/
