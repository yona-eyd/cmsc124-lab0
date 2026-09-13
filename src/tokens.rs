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
    //NotEqual,
    Assign,
    //EqualTo,
    Greater,
    //GreaterEql, 
    Less,
    //LessEql,

    Engk, //EOF
}

#[derive(Debug, Clone)]
pub struct Token{
    pub token_type: TokenList,
    //pub lexeme: String,
    //pub literal: char,
    //pub line: usize,
}

impl Token {
    pub fn to_string(&self) -> String {
        //let str = if self.literal == '\0' {
        //    "null".to_string()
       // } 
        //else {
        //    self.literal.to_string()
        //};

        format!(
            "Token (type={:?})",
            self.token_type
        )
    }
}