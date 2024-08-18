#![allow(unused)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::json;

// #region:     --- Type Def
type token_store<'a> = Vec<(&'a String, &'a i64)>;
// #endregion:  --- Type Def

// #region:     ---
// #endregion:  ---
// #region:     ---
// #endregion:  ---
// #region:     ---
// #endregion:  ---

#[derive(Debug, Serialize)]
pub struct Store<'a> {
    pub content: token_store<'a>,
}

impl<'a> Store<'a> {
    // Should probably rename
    pub fn new(content: Vec<String>, file_name: &str) -> String {
        let mut store: HashMap<String, i64> = HashMap::new();
        // Get the frequency of each word
        for s in content {
            if store.contains_key(&s) {
                store.entry(s).and_modify(|f| *f += 1);
            } else {
                store.insert(s, 1);
            }
        }

        // Store it into a vector for ordering
        let mut store: token_store = store.iter().collect::<Vec<_>>();
        store.sort_by(|a, b| {
            let a = *a.1;
            let b = *b.1;
            b.cmp(&a)
        });
        println!("{:?}", store);
        println!("   >>---------------------------| save_to_json() | ----------------------");
        let mut builder = String::new();
        for s in store {
            println!("{:?}", s);
            let s = format!(" {} : {}, ", s.0, s.1);
            builder.push_str(&s);
        }
        let store = format!("{file_name} : {{ {builder} }}");
        let for_save = json!(store);
        println!("{for_save}");
        return for_save.to_string();
        //store.sort_by_key(|a,b| a<b);
        // for i in store.into_iter().take(10) {
        //     println!("{:?}", i);
        // }
        // TODO: Store this in a json document and document the functions
    }

    // TODO: Read from file
}
