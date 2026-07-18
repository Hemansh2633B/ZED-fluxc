fn main() {
    // Build the Tree-sitter Flux parser from C source
    cc::Build::new()
        .file("src/parser.c")
        .include("src")
        .compile("tree-sitter-flux");
    
    // Tell cargo to rerun if parser.c changes
    println!("cargo:rerun-if-changed=src/parser.c");
    println!("cargo:rerun-if-changed=src/tree_sitter/parser.h");
}