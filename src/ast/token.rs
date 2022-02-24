use lasagna::*;

#[derive(Clone, Debug)]
pub struct Ident {
    string: String,
    span: Span,
}

impl Spanned for Ident {
    fn span(&self) -> Span {
        self.span
    }
}

impl Ident {
    pub fn as_str(&self) -> &str {
        &self.string
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

impl Lex for Ident {
    fn lex(lexer: &mut impl Lexer<Output = char>) -> Result<Self, Error> {
        let span = lexer.span(0);
        let mut ident = String::new();

        loop {
            if let Some(&ch) = lexer.peek() {
                if ch.is_alphabetic() {
                    ident.push(ch);
                    lexer.next();
                } else {
                    break;
                }
            }
        }

        Ok(Self {
            string: ident,
            span: span | lexer.span(0),
        })
    }
}

#[derive(Token)]
pub enum Token {
    #[token = "int"]
    Int,
    #[token = "float"]
    Float,
    #[token = "raw"]
    Raw,
    #[token = "=="]
    EqualEqual,
    #[token = "="]
    Equal,
    #[token = ":"]
    Colon,
    #[token = ";"]
    Semicolon,
    #[token]
    Ident(Ident),
}
