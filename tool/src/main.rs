use comment_away_lib as lib;
use comment_away_lib::config as config;
use clap::Parser;
use serde::Deserialize;
use std::fs;

#[derive(Parser, Debug)]
#[command(author,version,about)]
struct Args {
    // config file path
    #[arg(short,long,default_value = "config.json")]
    config: String,

    // target dir/file
    #[arg(short,long,default_value = "./")]
    target: String,

    #[arg(short,long,default_value = "./*.so")]
    lib_path: String

}
fn main() {

    let args = Args::parse();

    let config = config::load_config(&args.config).unwrap();
    let file_path = &args.target;

    let extension = file_path.rfind('.').map(|pos| &file_path[pos + 1..]).unwrap();
    // get the correct shared object, based on the extension of the target file
    let kt = config.find_struct_by_key(extension).unwrap();

    //// load the library from the shared object
    let library = lib::load_lib_so(kt.path.clone()).expect("Failed to load library");
    let symbol = lib::find_tree_sitter_function(&kt.path.clone(), &kt.language).unwrap().unwrap();

    // load the language from the library
    let language = lib::load_language(&library, &symbol).expect("Failed to load language");

    // instantiate a parser from the given language
    let parser = lib::create_parser(language).expect("Failed to set language");

    // Read source code from a file
    let mut source_code = lib::read_file_to_string(file_path).unwrap();
    // generate a tree_sitter::Tree from the source, using the parser
    let tree = lib::gen_tree(parser, &source_code);

    // grab the root node from the tree
    let root_node = tree.root_node();

    // Traverse and find comments
    lib::strip_nodes(root_node, &mut source_code);

    println!("modified code:\n {}", source_code);
    //// load the library from the shared object
    //let library = lib::load_lib_so(library_path.to_string()).expect("Failed to load library");

    //// find the symbol name within the shared object.
    //let symbol = lib::find_tree_sitter_function(library_path, "javascript")
    //    .unwrap()
    //    .unwrap();

    //// load the language from the library
    //let language = lib::load_language(&library, &symbol).expect("Failed to load language");

    //// instantiate a parser from the given language
    //let parser = lib::create_parser(language).expect("Failed to set language");

    //// Read source code from a file
    //let mut source_code = lib::read_file_to_string(file_path).unwrap();
    //// generate a tree_sitter::Tree from the source, using the parser
    //let tree = lib::gen_tree(parser, &source_code);

    //// grab the root node from the tree
    //let root_node = tree.root_node();

    //// Traverse and find comments
    //lib::strip_nodes(root_node, &mut source_code);

    //println!("modified code:\n {}", source_code);
}
