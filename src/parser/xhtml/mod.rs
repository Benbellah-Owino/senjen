// #region:     --- imports
use crate::parser::Error;
use std::{fs, path::Path};
use tracing::{debug, info};
use xml::reader::XmlEvent;
use xml::EventReader;
// #endregion:  --- imports

/// Extracts all the relevant texts from an xhml file and returns a vector of the tokens
pub fn parse_entire_file<P>(file_path: &P) -> Result<Vec<String>, Error>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(file_path)?; // Read from the file

    let reader = EventReader::new(file); // Parses the xhtml document

    let mut token_store: Vec<String> = Vec::new(); // Used to store the tokens

    for i in reader.into_iter() { 
        match i.unwrap() {
            XmlEvent::Characters(s) => {
                let s = s.trim().replace("\\n", "").to_owned();
                for i in s.split_ascii_whitespace().into_iter() { // Skip punctuations
                    if i == "." || i== "," || i == "\"" || i == "'" || i == ")" || i == "("{
                        continue;
                    }
                    i.replace(",", "");
                    i.replace(".", "");
                    i.replace("\"","");
                    token_store.push(i.trim().to_ascii_uppercase().to_owned());
                }
            }
            _ => {}
        }
    }
    return Ok(token_store);
}
