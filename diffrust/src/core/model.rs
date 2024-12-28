use md5;
use std::cmp::Ordering;
use std::fs;
use std::io;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum ContentType {
    ContentDir(Dir),
    ContentFile(File),
    ContentLink,
}

#[derive(Debug)]
pub struct Collection {
    /// Collection name
    pub name: String,
    /// The root path of the collection
    pub root: PathBuf,
    /// Path to db file. If None path is root/.diffrust.conf
    pub db: Option<PathBuf>,
    /// The root Dir struct. Can be None if not present or initialized.
    pub root_dir: Option<Dir>,
}

impl Collection {
    pub fn new() -> Self {
        Collection {
            name: String::from(""),
            root: PathBuf::new(),
            db: None,
            root_dir: None,
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        // let config_path = self.db.as_ref().unwrap_or(&self.root.join(".diffrust.conf"));
        // let json = serde_json::to_string(self)?;
        // fs::write(config_path, json)
        Ok(())
    }

    pub fn scan(&mut self) -> Result<(), std::io::Error> {
        let dir = self.root_dir.get_or_insert_with(|| Dir {
            path: self.root.clone(),
            content: Vec::new(),
        });
        dir.scan()
    }
}
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Dir {
    pub path: PathBuf,
    pub content: Vec<ContentType>,
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
                self.content.push(ContentType::ContentFile(File {
                    path: PathBuf::from(path),
                    md5: md5::compute(&contents),
                }));
            } else {
                if path.is_dir() {
                    self.content.push(ContentType::ContentDir(Dir {
                        path: PathBuf::from(path),
                        content: Vec::new(),
                    }));
                }
            }
        }
        Ok(())
    }

    // pub fn sorted_subdir(&self) -> Vec<Dir> {
    //     let mut sub: Vec<Dir> = self.content
    //         .iter()
    //         .filter_map(|item| {
    //             match item {
    //                 ContentType::ContentDir(d) => Some(*d),
    //                 _ => None
    //             }
    //         }).collect();
    //         // sub.sort_by();
    //         sub
    // }
}

#[derive(Debug, PartialEq)]
pub struct File {
    /// File path
    pub path: PathBuf,
    /// The hash of the file
    pub md5: md5::Digest,
}

impl PartialOrd for File {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.path.cmp(&other.path))
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn dummy_test() {
        assert_eq!(0, 0);
    }
}
