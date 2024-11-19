use libloading::{Library, Symbol};
use std::ffi::CString;
use tree_sitter::{Language, Parser};

// loads the library from the given shared object, wrapped to produce a Result.
fn load_lib_so(path: String) -> Result<Library, anyhow::Error> {
    let library = unsafe { Library::new(path)? };

    Ok(library)
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

fn gen_tree(mut parser:Parser,source_code: &str) -> tree_sitter::Tree {

    let tree = parser
        .parse(source_code, None)
        .expect("Failed to parse source code");
    tree
}

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

fn main() {
    // Path to the shared library
    let library_path = "/home/f/dev/cas/comment-away/src/shared_libs/libjavascript.so";

    // load the library from the shared object
    let library = load_lib_so(library_path.to_string()).expect("Failed to load library");

    // load the language from the library
    let language =
        load_language(&library, "tree_sitter_javascript").expect("Failed to load language");

    // instantiate a parser from the given language
    let parser = create_parser(language).expect("Failed to set language");

    let source_code = r#"
    // This is a comment
    function test() {
        console.log("Hello, world!"); // Another comment
    }
    "#;

    // generate a tree_sitter::Tree from the source, using the parser
    let tree = gen_tree(parser, source_code);

    // grab the root node from the tree
    let root_node = tree.root_node();

    // Traverse and find comments
    traverse_tree(root_node, source_code);
}

