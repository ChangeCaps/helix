use std::path::Path;

use lasagna::{CharsLexer, Parser, SkipWhitespace, SourcePath};

use crate::ast::stmt::NewVarStmt;

pub mod expr;
pub mod stmt;
pub mod token;
pub mod ty;

pub fn parse_ast(source: &str, path: &'static Path) -> Result<(), lasagna::Error> {
    let mut parser = SkipWhitespace::new(CharsLexer::new(source, SourcePath::Path(path)));

    let stmt = parser.parse::<NewVarStmt>()?;

    println!("{:?}", stmt);

    todo!()
}
