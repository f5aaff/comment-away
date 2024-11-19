use clap::Parser;
use comment_away_lib as lib;
use comment_away_lib::config;
use comment_away_lib::util;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::fs;
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    // config file path
    #[arg(short, long, default_value = "config.json")]
    config: String,

    // target dir/file
    #[arg(short, long, default_value = "./")]
    target: String,

}
fn main() {
    let args = Args::parse();

    let config = match config::load_config(&args.config) {
        Ok(c) => c,
        Err(e) => {
            println!("error loading config: {}", e);
            exit(1)
        }
    };

    let file_path = &args.target;
    if file_path.is_empty() {
        println!("provide a target.");
        exit(1)
    }

    let mut file_paths: Vec<PathBuf> = Vec::new();
    util::find_files(Path::new(file_path), &mut file_paths).unwrap_or_else(|_| {
        println!("error finding files, check target path.");
        exit(1);
    });

    for file in file_paths {
        // Get the file extension as Option<&OsStr>
        let ext_string:String;
        if let Some(extension) = file.extension() {
            // Convert the extension to &str
            if let Some(ext_str) = extension.to_str() {
                ext_string = String::from(ext_str);
            } else {
                println!("The extension is not valid UTF-8.");
                continue;
            }
        } else {
            println!("The file has no extension.");
            continue;
        }
        // get the correct shared object, based on the extension of the target file
        let kt_opt = config.find_struct_by_key(&ext_string);
        if kt_opt.is_none() {
            println!(
                "unable to find language support for: {}",
                file.to_str().unwrap()
            );
            continue;
        }
        let kt = kt_opt.unwrap();

        //// load the library from the shared object
        let library = lib::load_lib_so(kt.path.clone()).expect("Failed to load library");
        let symbol_opt = match lib::find_tree_sitter_function(&kt.path.clone(), &kt.language) {
            Ok(s) => s,
            Err(e) => {
                println!("error retrieving tree_sitter function: {}", e);
                exit(1);
            }
        };

        let symbol = symbol_opt.unwrap_or_else(|| {
            println!("error retrieving tree_sitter function. exiting...");
            exit(1)
        });

        // load the language from the library
        let language = lib::load_language(&library, &symbol).expect("Failed to load language");

        // instantiate a parser from the given language
        let parser = lib::create_parser(language).expect("Failed to set language");

        // Read source code from a file
        let mut source_code = lib::read_file_to_string(file.to_str().unwrap()).unwrap();
        // generate a tree_sitter::Tree from the source, using the parser
        let tree = lib::gen_tree(parser, &source_code);

        // grab the root node from the tree
        let root_node = tree.root_node();

        // Traverse and find comments
        lib::strip_nodes(root_node, &mut source_code);
        match fs::write(file, source_code.clone()){
            Ok(_) => {},
            Err(e) => {
                println!("error writing modified code to file: {}",e);
            }
        };
        println!("modified code:\n {}", source_code);
    }
}
