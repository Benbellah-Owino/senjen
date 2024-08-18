#![allow(unused)]

use std::{collections::HashMap, fs::File, io::{self, Read}, path::Path};

use serde::{Deserialize, Serialize};
use serde_json::json;

// #region:     --- Type Def
pub type TokenStoreRef<'a> = Vec<(&'a String, &'a i64)>;
pub type TokenStore = HashMap<String, i64>;
// #endregion:  --- Type Def

// #region:     ---
// #endregion:  ---
// #region:     ---
// #endregion:  ---
// #region:     ---
// #endregion:  ---

#[derive(Debug, Serialize)]
pub struct ToStore<'a> {
    pub content: TokenStoreRef<'a>,
}

#[derive(Debug,Deserialize)]
pub struct FromStore {
    pub content: TokenStore,
}
impl<'a> ToStore<'a> {
    // Should probably rename
    pub fn new(content: Vec<String>, file_path: &str) -> TokenStore{
        let mut store: TokenStore = HashMap::new();
        // Get the frequency of each word
        for s in content {
            if store.contains_key(&s) {
                store.entry(s).and_modify(|f| *f += 1);
            } else {
                store.insert(s, 1);
            }
        }

        // Store it into a vector for ordering
        let mut store: TokenStoreRef = store.iter().collect::<Vec<_>>();
        store.sort_by(|a, b| {
            let a = *a.1;
            let b = *b.1;
            b.cmp(&a)
        });
        println!("{:?}", store);
        println!("   >>---------------------------| save_to_json() | ----------------------");
        let mut builder = String::new();
        let mut for_save: TokenStore = HashMap::new();
        for s in store {
            println!("{:?}", s);
            for_save.insert(s.0.to_string(), s.1.to_owned());
        }
        // let store = format!("{file_path} : {{ {builder} }}");
        // let for_save = json!(store);
        // println!("{for_save}");
        return for_save;
        //store.sort_by_key(|a,b| a<b);
        // for i in store.into_iter().take(10) {
        //     println!("{:?}", i);
        // }
        // TODO: Store this in a json document and document the functions
    }

    // TODO: Read from file
}


impl FromStore{

    pub fn read_from_json<P>(file_path: P) -> Result<TokenStore, io::Error>
    where P: AsRef<Path>{
        let mut json_file = File::open(file_path)?;
        let mut buffer = String::new();
        json_file.read_to_string(&mut buffer);
        
        // let tokens = buffer.split(":").collect::<Vec<&str>>()[1..].to_vec();

        // println!("{:?}", buffer);
        let tokens: TokenStore = serde_json::from_str(&buffer).unwrap();
        // println!("{:?}", tokens);
        for i in &tokens{
            println!("{:?}", i);
        }
        
        Ok(tokens)
    }
}