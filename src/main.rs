use std::{fs, io};
use std::path::Path;

use p02_senjen::store::{get_dir_tokens, save_to_json, FromStore};



fn main() {
    // let file_path ="docs_gl\\el3\\gl_FragDepth.xhtml" ;
    // let i = ToStore::new(parse_entire_file(file_path).unwrap(), file_path);
    // save_to_json(&i).unwrap();
    
    //TODO: Indexing of the whole folder
    // let _i = FromStore::read_from_json("tokens.json").unwrap();
    let dir_path ="docs_gl\\el3" ;
    let _i = FromStore::read_from_json("tokens.json").unwrap();
    // let i = get_dir_tokens(dir_path).unwrap();
    // let _ = save_to_json(i);
}
