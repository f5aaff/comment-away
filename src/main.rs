use libloading::{Library, Symbol};
use tree_sitter::{Language, Parser};


// load the so using the library::new() from libloading.
fn load_lib_so(path: String) -> Result<Library, anyhow::Error> {
    let library = unsafe { Library::new(path)? };

    Ok(library)
}

// load the language from the library taken through libloading,
// uses an unsafe extern call to C, getting the library by name.
fn load_language(library: Library, name: &[u8]) -> Result<Language, anyhow::Error> {
    let language: Language = unsafe {
        let func: Symbol<unsafe extern "C" fn() -> Language> = library.get(name)?;
        func()
    };

    Ok(language)
}

// create a parser for the given language
fn create_parser(language: Language) -> Result<Parser, anyhow::Error> {
    let mut parser = Parser::new();
    match parser.set_language(&language){
        Ok(()) => {
            return Ok(parser)
        },
        Err(e) => {
            return Err(e.into())
        }
    }
}

// generate a tree using the parser, produces a tree_sitter::Tree, from which
// the root_node can be obtained.
fn gen_tree(mut parser: Parser, source_code: &str) -> tree_sitter::Tree {
    let tree = parser
        .parse(source_code, None)
        .expect("Failed to parse source code");
    tree
}

fn traverse_tree(node: tree_sitter::Node, source_code: &str,node_kind:String) {
    if node.kind() == &node_kind {
        println!(
            "Found comment: {}",
            &source_code[node.start_byte()..node.end_byte()]
        );
    }

    for child in node.children(&mut node.walk()) {
        traverse_tree(child, source_code,node_kind.clone());
    }
}

fn main() {
    // Path to the shared library
    println!("attempting to load library...");
    let library_path = "/home/f/dev/cas/comment-away/src/shared_libs/libjavascript.so";
    let library = match load_lib_so(library_path.to_string()) {
        Ok(l) => l,
        Err(e) => {
            panic!("error loading library from so: {}", e)
        }
    };
    println!("attempting to instantiate language...");
    // Correctly load the language function
    let language = match load_language(library, b"tree_sitter_javascript") {
        Ok(l) => l,
        Err(e) => {
            panic!("error loading language from library: {}", e)
        }
    };
    println!("attempting to instantiate parser...");
    let parser = match create_parser(language) {
        Ok(p) => p,
        Err(e) => {
            panic!("error generating parser: {}", e)
        }
    };

    let source_code = r#"
    // This is a comment
    function test() {
        console.log("Hello, world!"); // Another comment
    }
    "#;

    println!("attempting to generate tree...");
    let tree = gen_tree(parser, source_code);
    let root_node = tree.root_node();

    println!("attempting to traverse tree...");
    // Traverse and find comments
    traverse_tree(root_node, source_code,String::from("comment"));
}
