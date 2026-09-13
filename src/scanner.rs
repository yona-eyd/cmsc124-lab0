use crate :: tokens :: {Literal, Token, TokenList};

pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    emitted_eof: bool,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Scanner {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            emitted_eof: false,
        }
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current +=1;
        c
    }

    fn peek_match(&mut self, expected: char) -> bool {
        if self.at_end() || self.source[self.current] != expected {
            return false
        }
        self.current += 1;
        true
    }

    fn one_or_two(&mut self, second:char, one: TokenList, two:TokenList) -> TokenList {
        if self.peek_match(second){two} else {one}
    }

    fn make_token(&self, token_type: TokenList) -> Token {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        Token {
            token_type,
            lexeme,
            literal: Literal::None,
            line: self.line,
         }
    }

    fn scan_token(&mut self) -> Result<Option<Token>, ScanError>{
        let c = self.advance();
        
        let token = match c {
            '(' => (self.make_token(TokenList::LeftParen)),
            ')' => (self.make_token(TokenList::RightParen)),
            '{' => (self.make_token(TokenList::LeftBrace)),
            '}' => (self.make_token(TokenList::RightBrace)),
            ',' => (self.make_token(TokenList::Comma)),
            '.' => (self.make_token(TokenList::Dot)),
            '+' => (self.make_token(TokenList::Plus)),
            '-' => (self.make_token(TokenList::Minus)),
            '*' => (self.make_token(TokenList::Star)),
            '/' => (self.make_token(TokenList::Slash)),
            '%' => (self.make_token(TokenList::Modulo)),
            ';' => (self.make_token(TokenList::Semicolon)),

            '!' => {let kind = self.one_or_two('=',TokenList::Not, TokenList::NotEqual); 
                    Some(self.make_token(kind))}
            '=' => {let kind = self.one_or_two('=',TokenList::Assign, TokenList::Equal); 
                    Some(self.make_token(kind))}
            '>' => {let kind = self.one_or_two('=',TokenList::Great, TokenList::Greater); 
                    Some(self.make_token(kind))}
            '<' => {let kind = self.one_or_two('=',TokenList::Less, TokenList::Lesser); 
                    Some(self.make_token(kind))}

            ' ' | '\r' | '\t' => None,
            '\n' =>{self.line += 1;
                    None }

            '"' => return self.string().map(Some),

            other => {
                return Err(ScanError {
                    line: self.line,
                    message: format!("Unexpected character '{other}'"),
                })
            }

        };
        Ok(token)
    }
}

