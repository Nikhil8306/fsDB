#![allow(non_snake_case)]
#![allow(dead_code)]

pub mod error;
mod config;

use std::{path::PathBuf, fs};
use crate::error::DBError::{self, *};
use config::DB_DIR;


pub enum Type {
    Normal,
    Document
}

pub struct DB {
    name: String,
    dbPath: PathBuf,
    dbType: Type
}

impl DB {

    // Check if db is already present
    pub(crate) fn haveDB(name: &str, dbDir: &PathBuf) -> Result<bool, DBError> {

        let mut dbDir = PathBuf::from(dbDir);
        dbDir.push(name);

        if dbDir.exists() {
            return Ok(true);
        }

        return Ok(false);

    }

}

impl DB {

    pub fn new(name: &str, dbType: Type) -> Result<Self, DBError> {
        
        return Self::from(name, dbType, DB_DIR);

    }

    pub fn from(name: &str, dbType: Type, dbDir: &str) -> Result<Self, DBError> {

        let path = PathBuf::from(dbDir);
        if !path.exists() { // create database dir if doesn't exist
            fs::create_dir(&path).map_err(|e| CreateError(e.to_string()))?;
        }

        if Self::haveDB(name, &path)? { // If database is already present
            return Err(DbConflict);
        }

        return Ok(Self {
            name: String::from(name),
            dbPath: path,
            dbType
        });

    }

}