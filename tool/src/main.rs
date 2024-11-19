use comment_away_lib as lib;

fn main() {
    // Path to the shared library
    let library_path = "/home/f/dev/cas/comment-away/src/shared_libs/libjavascript.so";

    // load the library from the shared object
    let library = lib::load_lib_so(library_path.to_string()).expect("Failed to load library");

    // find the symbol name within the shared object.
    let symbol = lib::find_tree_sitter_function(library_path, "javascript")
        .unwrap()
        .unwrap();

    // load the language from the library
    let language = lib::load_language(&library, &symbol).expect("Failed to load language");

    // instantiate a parser from the given language
    let parser = lib::create_parser(language).expect("Failed to set language");

    // Read source code from a file
    let file_path = "/home/f/dev/cas/comment-away/test_source/test.js"; // Change this to your file path
    let mut source_code = lib::read_file_to_string(file_path).unwrap();
    // generate a tree_sitter::Tree from the source, using the parser
    let tree = lib::gen_tree(parser, &source_code);

    // grab the root node from the tree
    let root_node = tree.root_node();

    // Traverse and find comments
    lib::strip_nodes(root_node, &mut source_code);

    println!("modified code:\n {}", source_code);
}
