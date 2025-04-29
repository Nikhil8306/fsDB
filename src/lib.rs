// Database to store key value Atomic

use std::{env, fs, io::Read, path::PathBuf};
use crate::config::{self, dataDir};

pub mod create;
pub mod delete;
pub mod read;
pub mod error;

use error::DBError::{self, *};

#[derive(Debug)]
pub enum Type{ 
    Atomic,
    Composite
}

#[derive(Debug)]
pub struct DB{
    name:String,
    path:PathBuf,
    storageType:Type
}

impl DB {
    pub fn new(name: &str, storageType:Type) -> Result<Self, DBError> {

        let mut dbPath = dataDir();
        if dbPath.is_err() {
            return Err(Misc((dbPath.unwrap_err())));
        }

        let mut dbPath = dbPath.unwrap();

        let files = dbPath.read_dir().map_err(|_| DirError)?;

        for file in files {
            let file = file.map_err(|_| DirError)?;
            
            if file.file_name().to_str().unwrap_or("") == name {
                return Err(DbConflict);
            }
        }
        
        dbPath.push(name);
        
        match &storageType {
            Type::Atomic => {
                fs::File::create(&dbPath).map_err(|_| CreateError)?;
            },
            Type::Composite => {
                let dir = fs::create_dir(&dbPath).map_err(|_| CreateError)?;
            }
        }

        return Ok(DB {
            name:name.to_string(), 
            path:dbPath,
            storageType
        });
        
    }

    pub fn open(name: &str) -> Result<Self, DBError> {
        let mut dbDir = config::dataDir();

        if dbDir.is_err() {
            return Err(Misc(dbDir.unwrap_err()));
        }

        let mut dbDir = dbDir.unwrap();

        dbDir.push(name);

        if !dbDir.exists() {
            return Err(DbNotFound);
        }

        let storageType = match dbDir.as_path().is_file() {
            true => {
                let file = fs::File::open(&dbDir).map_err(|_| ReadError)?;
                Type::Atomic
            },
            false => {
                let dir = fs::read_dir(&dbDir).map_err(|_| ReadError)?;

                Type::Composite
            }
        };

        return Ok(Self {
            name: name.to_string(), 
            path:dbDir,
            storageType,
        });
    }



    // 
    const EXCLUDEDCHARS: [char;2] = ['\n', '\r'];
    fn isValidKey(&self, key: &str) -> Result<(), DBError>{

        if key.contains(Self::EXCLUDEDCHARS){
            return Err(Misc(String::from("The key contains invalid characters")));
        }

        Ok(())
    }
    
    fn isValidValue(&self, key: &str) -> Result<(), DBError> {

        if let Type::Atomic = self.storageType {
            
            if key.contains(Self::EXCLUDEDCHARS){
                return Err(Misc(String::from("Value contains invalid characters")));
            }

        }

        Ok(())

    }

}


