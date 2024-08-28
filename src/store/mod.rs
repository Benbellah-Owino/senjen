// #![allow(unused)]

use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Read},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use tracing::{debug, error, info};

use crate::parser::xhtml::parse_entire_file;

// #region:     --- Type Def
pub type TokenStoreRef<'a> = Vec<(&'a String, &'a Frequency)>;
pub type TokenStore = HashMap<String, Frequency>;

pub type FileStoreRef<'a> = HashMap<String, TokenStoreRef<'a>>;
pub type FileStore = HashMap<PathBuf, TokenStore>;
// #endregion:  --- Type Def

// #region:     ---
// #endregion:  ---
// #region:     ---
// #endregion:  ---
// #region:     ---
// #endregion:  ---

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Frequency {
    pub f: f64,
    pub tf: f64,
    pub idf: f64,
    pub idf_tdf: f64,
    pub checked: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct ToStore {
    pub content: TokenStore,
}

#[derive(Debug, Deserialize)]
pub struct FromStore {
    pub content: TokenStore,
}
impl ToStore {
    // Should probably rename
    pub fn tokenizer(content: Vec<String>) -> TokenStore {
        let mut store: TokenStore = HashMap::new();
        // Get the frequency of each word
        for s in content {
            if store.contains_key(&s) {
                store.entry(s).and_modify(|f| f.f += 1.0);
            } else {
                store.insert(
                    s,
                    Frequency {
                        f: 1.0,
                        tf: 0.0,
                        idf: 0.0,
                        idf_tdf: 0.0,
                        checked: false,
                    },
                );
            }
        }

        let len = store.len() as f64;
        eprintln!("LEN = {len}");
        for (_, freq) in store.iter_mut() {
            freq.tf = freq.f / len;
        }
        return store;
    }

    // TODO: Read from file
}

impl FromStore {
    /// Reads the json file containting the tokens and returns a hashmap containing filenames and the associated tokens
    pub fn read_from_json<P>(file_path: P) -> Result<FileStore, io::Error>
    where
        P: AsRef<Path>,
    {
        let mut json_file = File::open(file_path)?;
        let mut buffer = String::new();
        json_file.read_to_string(&mut buffer)?; //Read file contents to string

        let tokens: FileStore = serde_json::from_str(&buffer).unwrap(); // Convert the json tokens to rust suitable format(Hashmap)

        Ok(tokens)
    }
}

/// Iterates over the files in a directory and converts the words into tokens
pub fn get_dir_tokens<P>(dir_path: P) -> Result<FileStore, io::Error>
where
    P: AsRef<Path>,
{
    let dir = fs::read_dir(dir_path).unwrap(); // Get the files in a directory
    let mut file_store: FileStore = HashMap::new(); // Stores the tokens of each files as the values to the filename

    for entry in dir {
        // Iterate over the directory files
        let e = entry?.path();

        info!("Parsing : {:?}", &e);
        if let Ok(content) = parse_entire_file(&e) {
            //Get the words from document
            let ts = ToStore::tokenizer(content); // Tokenize each word
            file_store.insert(e, ts); // Store the relationship between the tokens and the files
        } else {
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
pub fn save_to_json(for_save: FileStore) -> Result<(), io::Error> {
    let new_file = File::create("tokens.json")?;
    // println!("{:#?}", for_save);
    serde_json::to_writer_pretty(new_file, &for_save)?;

    Ok(())
}
// Read from all the directories

pub fn find_total_freq(tokens: &mut FileStore, term: &str) -> f64 {
    let mut total_freq = 0.0;
    for (_, k) in tokens.iter_mut() {
        if let Some(t) = k.get_mut(term) {
            t.checked = true;
            total_freq += t.f;
        };
    }

    total_freq
}

pub fn find_idf(f: &mut FileStore, term: &str) -> f64 {
    let len = f.len() as f64;
    let total = f.values().filter(|k| k.contains_key(term)).count().max(1) as f64;
    let _ = f
        .iter_mut()
        .map(|(_, t)| t.get_mut(term).unwrap().checked = false); //Change this so that we can skip checked
                                                                 // for (path ,k) in f.iter_mut(){
                                                                 //     //k.contains_key(term)
                                                                 // }
    return (len / total).log10();
}
// #region:     --- Helper functions
pub fn tf_idf_calc(mut tokens: FileStore) -> FileStore {
    info!("TF_IDF_CALC");
    let mut tk2 = tokens.clone();
    let total_len = tokens.len();
    let total_items = tokens.values().map(|c| c.len() as f64).sum::<f64>();

    for (path, k) in tk2.iter() {
        info!("Checking  {:?}", path.as_path());
        for (term, freq) in k.iter() {
            info!("Checking  {term}");
            if freq.checked {
                //debug!("skipped");
                // info!("SKIPPED! {term} in  {:?}", path.as_path());
                continue;
            } else {
                let total_freq = find_total_freq(&mut tokens, term);
                let idf = find_idf(&mut tokens, term);
                let tf = total_freq / total_items;

                let idf_tf = tf + idf;
                //info!("{term} values include tf = {tf}, idf = {idf}, tf_idf = {idf_tf}");
                for (_, t) in tokens.iter_mut() {
                    println!("{term} iter");
                    // t.get_mut(term).unwrap().idf_tdf = idf_tf;
                    // t.get_mut(term).unwrap().idf = idf;
                    // t.get_mut(term).unwrap().tf = tf;
                    match t.get_mut(term) {
                        Some(entry) => {
                            entry.idf_tdf = idf_tf;
                            entry.idf = idf;
                            entry.tf = tf;
                        }
                        None => {
                            // Handle the case where `term` is not found in the `HashMap`.
                            println!("Term not found in the HashMap.");
                            continue;
                        }
                    }

                    //info!("-----------------------------------------------------");
                    // info!("{term} in  {:?}", path.as_path());
                    //info!("tf = {tf}, idf = {idf}, tf_idf = {idf_tf}");
                }
                eprint!("finished\n")
                //Change this so that we can skip checked
                // tk2.iter_mut().filter(|t| t.1.contains_key(term));
            }
        }
    }
    eprintln!("Finished calc");
    tokens
}
// #endregion:  --- Helper functions
