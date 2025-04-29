use crate::db::{self, DB, Type};
use std::{fmt::format, fs::{self, OpenOptions}, io::Write, path::PathBuf};
use super::error::DBError::{self, *};

impl DB {
    pub fn add(&self, key: &str, value: &str) -> Result<(), DBError> {

        let path = self.path.clone();

        self.isValidKey(key)?;
        self.isValidValue(value)?;
        
        match self.storageType {
            Type::Atomic => {
                let haveKey = Self::keyExist(&path, key)?;

                if haveKey {
                    return Err(DupKey);
                }   

                let mut file = OpenOptions::new().write(true).append(true).open(path).map_err(|_| WriteError)?;
                
                let res = file.write((format!("{}\n{}\n", key, value).as_bytes()));
                if res.is_err() {
                    return Err(WriteError);
                }
                
            }
            Type::Composite => {
                let haveKey = Self::fileExist(&path, key)?;
                if haveKey {
                    return Err(DupKey);
                }

                let mut filePath = path.clone();
                filePath.push(key);

                let mut file = fs::File::create(filePath).map_err(|_| WriteError)?;
                
                let res = file.write(value.as_bytes());
                if res.is_err() {
                    return Err(WriteError);
                }
            },  
        }

        Ok(())
    }

    fn keyExist (path: &PathBuf, key: &str) -> Result<bool, DBError> {
        let file = fs::read_to_string(path);

        match file {
            Ok(file) => {
                let mut ind = -1;
                for line in file.lines() {
                    ind += 1;
                    if ind % 2 != 0 {
                        continue;
                    }

                    if line == key {
                        return Ok(true);
                    }
                }
            },

            Err(_) => {
                return Err(ReadError);
            }
        }


        Ok(false)
    }

    fn fileExist(path: &PathBuf, key: &str) -> Result<bool, DBError> {

        let dirs = path.read_dir().map_err(|_| ReadError)?;
        
        for dir in dirs {
            if dir.is_err() {
                return Err(ReadError);
            }

            let dir = dir.unwrap();

            if dir.file_name() == key {
                return Ok(true);
            }
        }

        Ok(false)

    }

    
}