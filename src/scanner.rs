use crate :: tokens :: {Token, TokenList};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
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

    fn add_token(&mut self, token_type: TokenList){
        self.tokens.push(Token {
            token_type,
            //lexeme: text,
            //literal: '\0',
            //line: self.line,
        });
    }

    fn scan_token(&mut self){
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenList::LEFTPAREN),
            ')' => self.add_token(TokenList::RIGHTPAREN),
            '{' => self.add_token(TokenList::LEFTBRACE),
            '}' => self.add_token(TokenList::RIGHTBRACE),
            ',' => self.add_token(TokenList::COMMA),
            '.' => self.add_token(TokenList::DOT),
            '+' => self.add_token(TokenList::PLUS),
            '-' => self.add_token(TokenList::MINUS),
            '*' => self.add_token(TokenList::STAR),
            '/' => self.add_token(TokenList::SLASH),
            '%' => self.add_token(TokenList::MODULO),
            ';' => self.add_token(TokenList::SEMICOLON),
            '!' => self.add_token(TokenList::NOT),
            '=' => self.add_token(TokenList::ASSIGN),
            '>' => self.add_token(TokenList::GREATER),
            '<' => self.add_token(TokenList::LESS),
            _ => println!("Unexpected character"),
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            token_type: TokenList::ENGK,
            //lexeme: String::new(),
            //literal: '\0',
            //line: self.line,
        });
        &self.tokens
    }
}


