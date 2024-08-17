#![allow(unused)]

use std::collections::HashMap;

// #region:     --- Type Def
type token_store<'a> = Vec<(&'a String, &'a i64)> ;
// #endregion:  --- Type Def

// #region:     --- 
// #endregion:  --- 
// #region:     --- 
// #endregion:  --- 
// #region:     --- 
// #endregion:  --- 
pub struct Store<'a>{
    pub content: token_store<'a>
}

impl <'a> Store <'a>{
    pub fn new(content: Vec<String>) {
        
        let mut store: HashMap<String, i64> = HashMap::new();
        // Get the frequency of each word
        for s in content{
            if store.contains_key(&s){
                store.entry(s).and_modify(|f| *f += 1);
            }else{
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
        //store.sort_by_key(|a,b| a<b);
        for i in store.into_iter().take(10){
            println!("{:?}", i);
        }
        // TODO: Store this in a json document and document the functions
    }
}