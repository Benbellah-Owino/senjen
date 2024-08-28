
use p02_senjen::{parser::xhtml::parse_entire_file, store::{get_dir_tokens, save_to_json, tf_idf_calc, FromStore, ToStore}};
use tracing::{debug, info, subscriber};
use tracing_subscriber::FmtSubscriber;
use clap::{Args, Parser, Subcommand};
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct SenjenArgs{
    #[clap(subcommand)]
    pub action: Action
}

#[derive(Debug, Subcommand)]
pub enum Action{
    Index(IndexActions),
    Retrieve(SearchActions)
}
#[derive(Debug, Args)]
pub struct IndexActions{
    pub dir_name: String,
}

#[derive(Debug, Args)]
pub struct SearchActions{
    pub file_name: String,
}

fn main() {
    let subscriber = FmtSubscriber::builder().finish();
    subscriber::set_global_default(subscriber).expect("Failed tracing");
    // let file_path ="docs_gl\\el3\\gl_FragDepth.xhtml" ;
    // let i = ToStore::new(parse_entire_file(file_path).unwrap(), file_path);
    // save_to_json(&i).unwrap();
    
    //TODO: Indexing of the whole folder
    // let _i = FromStore::read_from_json("tokens.json").unwrap();
    // let dir_path ="docs_gl\\el3" ;
    // let _i = FromStore::read_from_json("tokens.json").unwrap();
    // let i = get_dir_tokens(dir_path).unwrap();
    // let _ = save_to_json(i);

    let args = SenjenArgs::parse();

    println!("{:?}", args);

    // TODO: Add the getting of filenames where a term occurs
    match args.action{
        Action::Index(act) => {
            info!("Parsing tokens");
            let i = get_dir_tokens(&act.dir_name).unwrap();

            info!("Calculating token numbers");
            let i = tf_idf_calc(i);
            
            info!("Saving tokens to json");
            save_to_json(i).unwrap();
        },
        Action::Retrieve(act) => {
            let _i = FromStore::read_from_json(&act.file_name).unwrap();
        },
    }
}
