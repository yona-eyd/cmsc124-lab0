use crate::tokens::{Literal, Token, TokenList};

#[derive(Debug)]
pub enum Expr {
    Literal(f64),
    Binary {
        left:   Box<Expr>,
        op:     TokenList,
        right:  Box<Expr>,
    },
    Grouping(Box<Expr>)
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {tokens, current: 0 }
    }
    
    // ** HELPER FUNCTIONS **
    
    // returns current (unconsumed) token
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    // returns token just consumed
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    // checks whether we're at EOF
    fn at_end(&self) -> bool {
        self.peek().token_type == TokenList::Engk
    }

    // consumes token and returns it to caller
    fn advance(&mut self) -> &Token {
        if !self.at_end() {
            self.current += 1;
        }
        self.previous()
    }

    // checks if cuttrnt token is of a certain type
    fn check(&self, token_type: TokenList) -> bool {
        if self.at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    // consumes token if it matches one of the following conditions
    fn match_token(&mut self, types: &[TokenList]) -> Option<TokenList> {
        for t in types {
            if self.check(t.clone()) {
                self.advance();
                return Some(t.clone());
            }
        }
        None
    }

    // ask for the token or throw error
    fn consume (&mut self, token_type: TokenList, message: &str) -> Result<&Token, ParseError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        Err(self.error(self.peek(), message))
    }

    fn error (&self, token: &Token, message: &str) -> ParseError {
        ParseError {
            line:token.line,
            message: message.to_string(),
        }
    }

    // ** RECURSIVE DESCENT FUNCTIONS **
    
    // expression → term
    pub fn expression(&mut self) -> Expr {
        self.term()
    }

    // term → factor ( ("+" | "-") factor )*
    fn term(&mut self) -> Expr {
        let mut node = self.factor();

        while let Some(op) = self.match_token(&[TokenList::Plus, TokenList::Minus]) {
            let right = self.factor();
            node = Expr::Binary {
                left: Box::new(node),
                op,
                right: Box::new(right),
            };
        }
        node
    }
    
    // factor → NUMBER | "(" expression ")"
    fn factor(&mut self) -> Expr {
        match &self.peek().token_type {
            TokenList::Number => {
                let val = match &self.peek().literal {
                    Literal::Num(n) => *n,
                    _ => panic!("Expected numeric literal"),
                };
                self.advance();
                Expr::Literal(val)
            }
            TokenList::LeftParen => {
                self.advance();     // consumes left paren
                let expr = self.expression();
                self.match_token(&[TokenList::RightParen]);   // consumes left paren
                Expr::Grouping(Box::new(expr))
            }
            _ => panic!("Unexpected token: {:?}", self.peek()),
        }
    }
}

impl Expr {
    pub fn print(&self) -> String {
        match self {
            Expr::Literal(n) => format!("{}", n),
            Expr::Binary { left, op, right } => {
                let op_str = match op {
                    TokenList::Plus => "+",
                    TokenList::Minus => "-",
                    _=> "?",
                };
                format!("({} {} {})", op_str, left.print(), right.print())
            }
            Expr::Grouping(expr) => format!("(group {})", expr.print()),
        }
    }
}