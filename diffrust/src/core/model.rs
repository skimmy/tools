use md5;
use std::cmp::Ordering;
use std::fs;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};

/// The types of content that a directory can contain
#[derive(Debug, PartialEq, PartialOrd)]
pub enum ContentType {
    /// Directory content
    ContentDir(Dir),
    /// File content
    ContentFile(File),
    /// Hard or symbolic link content (not yet implemented)
    ContentLink,
}

/// A directory that is indexed by diffrust
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
    /// Creates a new empty `Collection`
    ///
    /// An empty `Collection` has its fields set to some default values.
    /// In particular, string and string-type values are set to empty
    /// strings and `Option` enums are set to `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use diffrust::core::model::Collection;
    /// use std::path::PathBuf;
    /// let c = Collection::new();
    /// assert_eq!(c.name,String::new());
    /// assert_eq!(c.root, PathBuf::new());
    /// assert!(c.db.is_none());
    /// assert!(c.root_dir.is_none());
    /// ```
    pub fn new() -> Self {
        Collection {
            name: String::new(),
            root: PathBuf::new(),
            db: None,
            root_dir: None,
        }
    }

    pub fn from(path: &Path) -> Self {
        Collection {
            name: String::new(),
            root: PathBuf::from(path),
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

/// An indexed directory
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Dir {
    /// Absolute path of the directory
    pub path: PathBuf,
    /// Directory content
    pub content: Vec<ContentType>,
}

impl Eq for Dir {}

impl Ord for Dir {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.path.cmp(&other.path);
    }
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

    pub fn sorted_dirs(&self) -> Vec<&Dir> {
        let mut dirs: Vec<&Dir> = self
            .content
            .iter()
            .filter_map(|item| {
                if let ContentType::ContentDir(d) = item {
                    Some(d)
                } else {
                    None
                }
            })
            .collect();
        dirs.sort_unstable();
        dirs
    }

    pub fn sorted_files(&self) -> Vec<&File> {
        let mut files: Vec<&File> = self
            .content
            .iter()
            .filter_map(|item| {
                if let ContentType::ContentFile(f) = item {
                    Some(f)
                } else {
                    None
                }
            })
            .collect();
        files.sort_unstable();
        files
    }
}

/// And indexed file
#[derive(Debug, PartialEq, Eq)]
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

impl Ord for File {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.path.cmp(&other.path);
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn collection_new() {
        let collection = Collection::new();
        assert_eq!(collection.name, "");
        assert_eq!(collection.root, PathBuf::new());
        assert!(collection.db.is_none());
        assert!(collection.root_dir.is_none());
    }

    #[test]
    fn collection_from_path() {
        let dir = tempdir().unwrap();
        let collection = Collection::from(dir.path());
        assert_eq!(
            collection.root,
            PathBuf::from(dir.path()),
            "Unmatched paths"
        );
    }

    #[test]
    fn collection_save_ok() {
        let collection = Collection::new();
        let result = collection.save();
        assert!(result.is_ok());
        assert!(false, "Need to implement file checking test");
    }

    #[test]
    fn scan_empty() {
        let tempdir = tempdir().unwrap();
        let mut collection = Collection::new();
        collection.root = PathBuf::from(tempdir.path());
        let _ = collection.scan();

        assert_ne!(
            None, collection.root_dir,
            "root_dir is None even after scan() of valid directory"
        );
        assert_eq!(
            tempdir.path(),
            collection.root_dir.as_ref().unwrap().path,
            "Path of root Dir not matching given directory"
        );
        let content = collection.root_dir.unwrap().content;
        assert_eq!(
            0,
            content.len(),
            "Scan found some content in empty directory"
        );
    }

    #[test]
    fn scan_content() {
        assert!(false, "Need to implement scan of directory test");
    }

    #[test]
    fn dir_compare() {
        let d1 = Dir {
            path: PathBuf::from("/abc"),
            content: vec![],
        };
        let d2 = Dir {
            path: PathBuf::from("/abc/aaa"),
            content: vec![],
        };
        let d3 = Dir {
            path: PathBuf::from("/abf"),
            content: vec![],
        };
        assert_eq!(Ordering::Less, d1.cmp(&d3));
        assert_eq!(Ordering::Greater, d2.cmp(&d1));
        assert_eq!(Ordering::Equal, d3.cmp(&d3));
    }

    #[test]
    fn dir_scan() {
        assert!(false, "Need to implement Dir scan test");
    }

    #[test]
    fn sort_files() {
        let dir = Dir {
            path: PathBuf::from("/"),
            content: content_vector(),
        };
        let sorted = dir.sorted_files();
        let v = files_vector();
        assert_eq!(vec![&v[0], &v[1]], sorted, "Unsorted files in Dir")
    }

    #[test]
    fn sort_dirs() {
        let dir = Dir {
            path: PathBuf::from(""),
            content: content_vector(),
        };
        let sorted = dir.sorted_dirs();
        let v = dirs_vector();
        assert_eq!(vec![&v[0], &v[1]], sorted, "Unsorted directories in Dir");
    }

    fn content_vector() -> Vec<ContentType> {
        vec![
            ContentType::ContentFile(File {
                path: PathBuf::from("/README.md"),
                md5: md5::compute(b"README"),
            }),
            ContentType::ContentDir(Dir {
                path: PathBuf::from("/"),
                content: vec![],
            }),
            ContentType::ContentFile(File {
                path: PathBuf::from("~/Documents/hello.txt"),
                md5: md5::compute(b"Hello World!"),
            }),
            ContentType::ContentDir(Dir {
                path: PathBuf::from("/root"),
                content: vec![],
            }),
        ]
    }

    fn files_vector() -> Vec<File> {
        vec![
            File {
                path: PathBuf::from("/README.md"),
                md5: md5::compute(b"README"),
            },
            File {
                path: PathBuf::from("~/Documents/hello.txt"),
                md5: md5::compute(b"Hello World!"),
            },
        ]
    }

    fn dirs_vector() -> Vec<Dir> {
        vec![
            Dir {
                path: PathBuf::from("/"),
                content: vec![],
            },
            Dir {
                path: PathBuf::from("/root"),
                content: vec![],
            }
        ]
    }
}
