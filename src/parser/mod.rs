#![allow(unused)]

// #region:     --- imports
use std::{fs, io, path::{Path, PathBuf}};
use xml::{reader::XmlEvent, EventReader};
// #endregion:  --- imports


// #region:     --- modules
pub mod xhtml;
// #endregion:  --- modules 


// #region:     --- 
// #endregion:  --- 
// #region:     --- 
// #endregion:  --- 


// #region:      Type Definitions
#[derive(Debug)]
pub enum Error{
    FileError(io::Error)
}
// Error methods
impl core::fmt::Display for Error{
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}
impl std::error::Error for Error{}
impl From<io::Error> for Error{
    fn from(val: io::Error) -> Self{
        Self::FileError(val)
    }
}
// #endregion:   Type Definitions

enum FileType{}

// #region:     --- XHTML Parser

// #endregion:  --- XHTML Parser