use lasagna::*;

use super::token::*;

#[derive(Spanned, Parse, Debug)]
pub enum Type {
    Int(Int),
}
