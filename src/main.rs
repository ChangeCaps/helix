use std::{fs, path::Path};

use ast::parse_ast;

mod ast;

fn main() {
    let source = fs::read_to_string("example.hlx").unwrap();

    parse_ast(&source, &Path::new("ashdk")).unwrap();
}
