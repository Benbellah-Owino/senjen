use std::{fs, io};
use std::path::Path;

use p02_senjen::parser::save_to_json;
use p02_senjen::parser::xhtml::parse_entire_file;
use p02_senjen::store::{FromStore, ToStore};

fn main() {
    // let file_path ="docs_gl\\el3\\gl_FragDepth.xhtml" ;
    // let i = ToStore::new(parse_entire_file(file_path).unwrap(), file_path);
    // save_to_json(&i).unwrap();
    
    //TODO: Indexing of the whole folder
    // let _i = FromStore::read_from_json("tokens.json").unwrap();
    let dir_path ="docs_gl\\el3" ;
    get_files_from_dir(dir_path);
}

// Read from all the directories
fn get_files_from_dir<P>(dir_path: P) -> Result<(), io::Error> where P:AsRef<Path>{
    let dir = fs::read_dir(dir_path).unwrap();
    for entry in dir{
        eprintln!("{:#?}", entry?);
    }
    Ok(())
}