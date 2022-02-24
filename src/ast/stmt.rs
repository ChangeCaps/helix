use lasagna::*;

use super::{token::*, ty::Type};

#[derive(Spanned, Parse, Debug)]
pub struct NewVarStmt {
    pub ty: Type,
    pub ident: Ident,
    pub equal: Equal,
}
