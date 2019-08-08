extern crate cc;

use std::path::PathBuf;

fn main() {
    let tree_sitter_rust: PathBuf = std::fs::canonicalize::<PathBuf>([".", "tree-sitter-rust", "src"].iter().collect()).unwrap();
    
    cc::Build::new()
        .include(&tree_sitter_rust)
        .file(tree_sitter_rust.join("parser.c"))
        .file(tree_sitter_rust.join("scanner.c"))
        .flag("-Wno-unused-parameter")
        .compile("tree-sitter-rust");
}