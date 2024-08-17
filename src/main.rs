use p02_senjen::parser::xhtml::parse_entire_file;
use p02_senjen::store::Store;

fn main() {
    println!("Hello, world!");
    let i = Store::new(parse_entire_file("docs_gl\\el3\\gl_FragDepth.xhtml").unwrap());

}
