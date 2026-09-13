use std::fmt;

// define alphabet of token categories for scanner to identify
#[derive(Debug, Clone, PartialEq)]
pub enum TokenList{
    //single char tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Plus,
    Minus,
    Star,
    Slash,
    Modulo,
    Semicolon,
    
    // need lookahead
    Not, 
    NotEqual,
    Assign,
    EqualTo,
    Greater,
    GreaterEql, 
    Less,
    LessEql,

    Engk, // EOF

    //identifiers
    And,
    If,
    Else,
    False,
    True,
    For,
    While,
    Nil,
    Or,
    Print,
    Return,
    Var,

    Number,
    StringLit,
    Identifier,
}

// define what values a token can have
#[derive(Debug, Clone)]
pub enum Literal {
    Str(String),
    Num(f64),
    None,
}

impl fmt :: Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Str(s) => write!(f, "{s}"),
            Literal::Num(n) => write!(f, "{n}"),
            Literal::None   => write!(f, "null"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token{
    pub token_type: TokenList,
    pub lexeme: String,
    pub literal: Literal,
    pub line: usize,
}

impl fmt:: Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token(type={:?}, lexeme={}, literal={}, line={})",
            self.token_type, self.lexeme, self.literal, self.line
        )
    }
}