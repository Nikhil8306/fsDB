use crate::db::{DB, Type};
use std::{collections::HashMap, fs, hash::Hash, path::PathBuf};
use super::error::DBError::{self, *};

impl DB {
    pub fn get(&self, key: &str) -> Result<Option<String>, DBError> {

        let path = self.path.clone();
        
        let value:Option<String> = match &self.storageType {
            Type::Atomic => {
                Self::findKey(&path, key)
            },  

            Type::Composite => {     
                Self::findFile(&path, key)
            }
        };


        Ok(value)
    }

    pub fn getDB(&self) -> Result<HashMap<String, String>, DBError> {

        match self.storageType {
            Type::Atomic => {

                let file = fs::read_to_string(&self.path).map_err(|_| ReadError)?;

                let lines:Vec<&str> = file.split("\n").into_iter().collect();
                let mut ind = 0;

                let mut map: HashMap<String, String> = HashMap::new();

                while ind < lines.len()-1 {
                    map.insert(lines[ind].to_string(), lines[ind+1].to_string());

                    ind += 2;
                }   

                Ok(map)
            },  

            Type::Composite => {
                todo!();
            }
        }

    }

    fn findKey(path: &PathBuf, key: &str) -> Result<Option<String>, DBError> {

        let file = fs::read_to_string(path).map_err(|_| ReadError)?;

        let lines:Vec<&str> = file.split("\n").into_iter().collect();

        let mut ind = 0;

        
        while ind < lines.len()-1 {
            if lines[ind] == key {
                return Ok(Some(lines[ind+1].to_string()));
            }
            ind += 1;
        }


        Ok(None)
    }

    fn findFile(path: &PathBuf, key: &str) -> Result<Option<String>, DBError> {

        let dirs = fs::read_dir(path).map_err(|_| ReadError)?;

        for dir in dirs {
            let dir = dir.map_err(|_| ReadError)?;
            
            if dir.file_name() == key {
                let content = fs::read_to_string(dir.path()).map_err(|_| ReadError)?;
                return Ok(Some(content));
            }
        
        }   


        Ok(None)
    }
}