use p02_senjen::parser::save_to_json;
use p02_senjen::parser::xhtml::parse_entire_file;
use p02_senjen::store::Store;

fn main() {
    println!("Hello, world!");
    let file_name ="docs_gl\\el3\\gl_FragDepth.xhtml" ;
    let i = Store::new(parse_entire_file(file_name).unwrap(), file_name);
    save_to_json(&i).unwrap();
    
}
