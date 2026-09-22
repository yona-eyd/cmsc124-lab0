use crate::tokens::{Literal, Token, TokenList};

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    Binary {
        left:   Box<Expr>,
        op:     Token,
        right:  Box<Expr>,
    },
    Grouping(Box<Expr>)
}

#[derive(Debug)]
pub struct ParseError {
    pub line: usize,
    pub message: String,
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
    pub fn expression(&mut self) -> Result<Expr , ParseError> {
        self.term()
    }

    // term → factor ( ("+" | "-") factor )*
    fn term(&mut self) -> Result<Expr , ParseError> {
        let mut node = self.factor()?;

        while let Some(_) = self.match_token(&[TokenList::Plus, TokenList::Minus]) {
            let op = self.previous().clone();
            let right = self.factor()?;
            node = Expr::Binary {
                left: Box::new(node),
                op,
                right: Box::new(right),
            };
        }
        Ok(node)
    }
    
    // factor → NUMBER | "(" expression ")"
    fn factor(&mut self) -> Result<Expr , ParseError> {
        match &self.peek().token_type {
            TokenList::Number => {
                let val = self.peek().literal.clone();
                self.advance();
                Ok(Expr::Literal(val))
            }
            
            TokenList::StringLit => {
                let val = self.peek().literal.clone();
                self.advance();
                Ok(Expr::Literal(val))
            }
            

            TokenList::LeftParen => {
                self.advance(); // consumes '('
                let expr = self.expression()?;
                self.consume(TokenList::RightParen, "Expect ')' after expression.")?;
                Ok(Expr::Grouping(Box::new(expr)))
            }
            _ => Err(self.error(self.peek(), "Expect expression.")),
        }
    }
}


impl Expr {
    pub fn print(&self) -> String {
        match self {
            Expr::Literal(lit) => match lit {
                Literal::Str(s) => s.clone(),
                Literal::Num(n) => format!("{n}"),
                Literal::None => "nil".to_string(),
            },
            Expr::Binary { left, op, right } => {
                format!("({} {} {})", left.print(), op.lexeme, right.print())
            }
            Expr::Grouping(expr) => format!("(group {})", expr.print()),
        }
    }
}

