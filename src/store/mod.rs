// #![allow(unused)]

use std::{collections::HashMap, fs::{self, File}, io::{self, Read}, path::{Path, PathBuf}};

use serde::{Deserialize, Serialize};
use tracing::info;

use crate::parser::xhtml::parse_entire_file;

// #region:     --- Type Def
pub type TokenStoreRef<'a> = Vec<(&'a String, &'a i64)>;
pub type TokenStore = HashMap<String, i64>;

pub type FileStoreRef<'a> = HashMap<String, TokenStoreRef<'a>>;
pub type FileStore = HashMap<PathBuf, TokenStore>;
// #endregion:  --- Type Def

// #region:     ---
// #endregion:  ---
// #region:     ---
// #endregion:  ---
// #region:     ---
// #endregion:  ---

#[derive(Debug, Serialize)]
pub struct ToStore {
    pub content: TokenStore,
}

#[derive(Debug,Deserialize)]
pub struct FromStore {
    pub content: TokenStore,
}
impl ToStore{
    // Should probably rename
    pub fn tokenizer(content: Vec<String>) -> TokenStore{
        let mut store: TokenStore = HashMap::new();
        // Get the frequency of each word
        for s in content {
            if store.contains_key(&s) {
                store.entry(s).and_modify(|f| *f += 1);
            } else {
                store.insert(s, 1);
            }
        }

        let mut store: TokenStoreRef = store.iter().collect::<Vec<_>>(); // Store it into a vector for ordering
        store.sort_by(|a, b| {
            let a = *a.1;
            let b = *b.1;
            b.cmp(&a)
        }); // Sort it
        
        let mut for_save: TokenStore = HashMap::new();
        for s in store {
            for_save.insert(s.0.to_string(), s.1.to_owned());
        } // Create token store

        return for_save;
    }

    // TODO: Read from file
}


impl FromStore{
    
    /// Reads the json file containting the tokens and returns a hashmap containing filenames and the associated tokens
    pub fn read_from_json<P>(file_path: P) -> Result<FileStore, io::Error>
    where P: AsRef<Path>{
        
        let mut json_file = File::open(file_path)?; 
        let mut buffer = String::new();
        json_file.read_to_string(&mut buffer)?; //Read file contents to string
        
        let tokens: FileStore = serde_json::from_str(&buffer).unwrap(); // Convert the json tokens to rust suitable format(Hashmap)
        
        Ok(tokens)
    }
}

/// Iterates over the files in a directory and converts the words into tokens
pub fn get_dir_tokens<P>(dir_path: P) -> Result< FileStore,io::Error>
where P: AsRef<Path> {
    let dir = fs::read_dir(dir_path).unwrap(); // Get the files in a directory
    let mut file_store: FileStore = HashMap::new(); // Stores the tokens of each files as the values to the filename

    for entry in dir{ // Iterate over the directory files
        let e = entry?.path();

        info!("Parsing : {:?}", &e);
        if let Ok(content) = parse_entire_file(&e){ //Get the words from document
            let ts = ToStore::tokenizer(content); // Tokenize each word
            file_store.insert(e, ts); // Store the relationship between the tokens and the files
        }else{
            continue;
        };
    }

    // println!("\n --------------------| File Names |---------------------------");
    // for (k, v) in &file_store{
    //     println!("{:#?}", k);
    // }

    Ok(file_store)
}

/// Save the tokens to a json file
pub fn save_to_json(for_save: FileStore) -> Result<(), io::Error>{
    let new_file = File::create("tokens.json")?;
    println!("{:#?}", for_save);
    serde_json::to_writer_pretty(new_file, &for_save)?;
    
    Ok(())
}
// Read from all the directories