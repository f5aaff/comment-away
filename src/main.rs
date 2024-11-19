use libloading::{Library, Symbol};
use object::{Object, ObjectSymbol};
use std::ffi::CString;
use std::fs;
use std::path::Path;
use tree_sitter::{Language, Parser};

use anyhow::{Context, Result};
// loads the library from the given shared object, wrapped to produce a Result.
fn load_lib_so(path: String) -> Result<Library, anyhow::Error> {
    let library = unsafe { Library::new(path)? };

    Ok(library)
}

// programmatically find the tree_sitter function, to use for the Language
// load function. This searches the shared object for symbols containing both the
// target langage and tree_sitter, and returns the shortest result, as that is likely
// to be the target (e.g tree_sitter_javascript or python_tree_sitter)
fn find_tree_sitter_function(path: &str, target: &str) -> Result<Option<String>> {
    // Read the shared library file
    let file = fs::read(path)
        .with_context(|| format!("Failed to read shared library file: {}", path))?;

    // Parse the shared library as an object file
    let obj_file = object::File::parse(&*file)
        .with_context(|| format!("Failed to parse object file: {}", path))?;

    // Collect all symbol names containing both "tree_sitter" and the target substring
    let mut symbols: Vec<String> = obj_file
        .symbols()
        .filter_map(|symbol| symbol.name().ok())  // Filter out invalid symbol names
        .filter(|name| name.contains("tree_sitter") && name.contains(target))
        .map(|name| name.to_string())
        .collect();

    // Return the shortest symbol or None if the list is empty
    symbols.sort_by_key(|name| name.len());
    Ok(symbols.into_iter().next())
}

// load the language from the library, converting the name str to a CString
// for the null terminating byte.
fn load_language(library: &Library, name: &str) -> Result<Language, anyhow::Error> {
    // Append a null terminator to the name and convert it to CString
    let c_name = CString::new(name)?;

    // Get the byte slice with a null terminator
    let bytes = c_name.as_bytes_with_nul();

    let language: Language = unsafe {
        let func: Symbol<unsafe extern "C" fn() -> Language> = library
            .get(bytes)
            .expect("Failed to load language function");
        func()
    };

    Ok(language)
}

// creates a parser from the given language, wrapped to produce a Result.
fn create_parser(language: Language) -> Result<Parser, anyhow::Error> {
    let mut parser = Parser::new();
    parser.set_language(&language)?;

    Ok(parser)
}

// generates a Tree_sitter::Tree from the source code, using the parser.
fn gen_tree(mut parser: Parser, source_code: &str) -> tree_sitter::Tree {
    let tree = parser
        .parse(source_code, None)
        .expect("Failed to parse source code");
    tree
}

// traverses the tree, finding comments.
fn traverse_tree(node: tree_sitter::Node, source_code: &str) {
    if node.kind() == "comment" {
        println!(
            "Found comment: {}",
            &source_code[node.start_byte()..node.end_byte()]
        );
    }

    for child in node.children(&mut node.walk()) {
        traverse_tree(child, source_code);
    }
}
// Helper function to read a file into a String
fn read_file_to_string<P: AsRef<Path>>(path: P) -> Result<String, anyhow::Error> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}

fn main() {
    // Path to the shared library
    let library_path = "/home/f/dev/cas/comment-away/src/shared_libs/libjavascript.so";

    // load the library from the shared object
    let library = load_lib_so(library_path.to_string()).expect("Failed to load library");

    // find the symbol name within the shared object.
    let symbol = find_tree_sitter_function(library_path, "javascript").unwrap().unwrap();

    // load the language from the library
    let language =
        load_language(&library, &symbol).expect("Failed to load language");

    // instantiate a parser from the given language
    let parser = create_parser(language).expect("Failed to set language");

    // Read source code from a file
    let file_path = "/home/f/dev/cas/comment-away/test_source/test.js"; // Change this to your file path
    let source_code = read_file_to_string(file_path).unwrap();

    // generate a tree_sitter::Tree from the source, using the parser
    let tree = gen_tree(parser, &source_code);

    // grab the root node from the tree
    let root_node = tree.root_node();

    // Traverse and find comments
    traverse_tree(root_node, &source_code);
}
