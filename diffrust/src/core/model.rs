use md5;
use std::fs;
use std::io;
use std::io::Read;
use std::path::PathBuf;

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
    pub fn save(&self) -> Result<(), std::io::Error> {
        // let config_path = self.db.as_ref().unwrap_or(&self.root.join(".diffrust.conf"));
        // let json = serde_json::to_string(self)?;
        // fs::write(config_path, json)
        Ok(())
    }

    pub fn scan(&mut self) -> Result<(), std::io::Error>  {
        let dir = self.root_dir.get_or_insert_with(|| Dir {
            files: Vec::new(),
            path: self.root.clone(),
        });
        dir.scan()
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
    pub path: PathBuf,
    /// The hash of the file
    pub md5: md5::Digest,
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn dummy_test() {
        assert_eq!(0,0);
    }
}
