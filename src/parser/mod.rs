#![allow(unused)]


// #region:     --- imports
use serde::Serialize;
use serde_json::json;
use std::{
    fs::{self, File}, io::{self, Write},
    path::{Path, PathBuf}, str::Bytes,
};
use xml::{reader::XmlEvent, EventReader};

use crate::store::Store;
// #endregion:  --- imports

// #region:     --- modules
pub mod xhtml;
// #endregion:  --- modules

// #region:     ---
// #endregion:  ---
// #region:     ---
// #endregion:  ---

// #region:     ---
// #endregion:  ---

// #region:     ---
// #endregion:  ---
// #region:     ---
// #endregion:  ---
// #region:     ---
// #endregion:  ---
// #region:      Type Definitions
#[derive(Debug)]
pub enum Error {
    FileError(io::Error),
}
// Error methods
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}
impl std::error::Error for Error {}
impl From<io::Error> for Error {
    fn from(val: io::Error) -> Self {
        Self::FileError(val)
    }
}
// #endregion:   Type Definitions

enum FileType {}

// #region:     --- XHTML Parser

// #endregion:  --- XHTML Parser

// #region:     --- Json
pub fn save_to_json(tokens: &str) -> Result<(), Error>{
    let mut new_file = File::create("tokens.json")?;
    new_file.write_all(tokens.as_bytes())?;
    Ok(())
}
// #endregion:  --- Json
