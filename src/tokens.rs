#[derive(Debug, Clone, PartialEq)]
pub enum TokenList{
    //single char tokens
    LEFTPAREN,
    RIGHTPAREN,
    LEFTBRACE,
    RIGHTBRACE,
    COMMA,
    DOT,
    PLUS,
    MINUS,
    STAR,
    SLASH,
    MODULO,
    SEMICOLON,
    
    // need lookahead
    NOT, 
    //NOTEQUAL,
    ASSIGN,
    //EQUALTO,
    GREATER,
    //GREATEREQL, 
    LESS,
    //LESSEQL,

    ENGK, //EOF
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