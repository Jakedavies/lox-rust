use crate::{tokens::{Token, TokenType}, tree::{Expression, Literal}, expressions::{literal_expression::LiteralExpression, grouping_expression::GroupingExpression, unary_expression::UnaryExpression, binary_expression::BinaryExpression}};


pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: &Vec<Token>) -> Parser {
        Parser {
            tokens: tokens.clone(),
            pos: 0,
        }
    }

    pub fn parse(&mut self) -> Box<dyn Expression> {
        return self.expression();
    }

    fn expression (&mut self) -> Box<dyn Expression> {
        return self.equality();
    }

    fn equality (&mut self) -> Box<dyn Expression> {
        let mut expr = self.comparison();

        while self.match_tokens(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous().clone();
            let right = self.comparison();
            expr = Box::new(BinaryExpression::new(
                op,
                expr,
                right,
            ));
        }

        return expr;
    }

    fn comparison (&mut self) -> Box<dyn Expression> {
        let mut expr = self.term();

        while self.match_tokens(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let op = self.previous().clone();
            let right = self.term();
            expr = Box::new(BinaryExpression::new(
                op,
                right,
                expr,
            ));
        }

        return expr;
    }

    fn term (&mut self) -> Box<dyn Expression> {
        let mut expr = self.factor();

        while self.match_tokens(vec![TokenType::Minus, TokenType::Plus]) {
            let op = self.previous().clone();
            let right = self.factor();
            expr = Box::new(BinaryExpression::new(
                op,
                right,
                expr
            ));
        }

        return expr;
    }

    fn factor (&mut self) -> Box<dyn Expression> {
        let mut expr = self.unary();

        while self.match_tokens(vec![TokenType::Slash, TokenType::Star]) {
            let op = self.previous().clone();
            let right = self.unary();
            expr = Box::new(BinaryExpression::new(
                op,
                right,
                expr
            ));
        }

        return expr;
    }

    fn unary (&mut self) -> Box<dyn Expression> {
        if self.match_tokens(vec![TokenType::Bang, TokenType::Minus]) {
            let op = self.previous().clone();
            let right = self.unary();
            return Box::new(UnaryExpression::new(
                op,
                right
            ));
        }

        return self.primary();
    }

    fn primary (&mut self) -> Box<dyn Expression> {
        match &self.tokens[self.pos].token_type {
            TokenType::False => {
                self.advance();
                return Box::new(LiteralExpression::new(
                    Literal::Boolean(false),
                ));
            }
            TokenType::True => {
                self.advance();
                return Box::new(LiteralExpression::new(
                    Literal::Boolean(true),
                ));
            }
            TokenType::Number(val) => {
                let v = val.clone();
                self.advance();
                return Box::new(LiteralExpression::new(
                    Literal::Number(v),
                ));
            }
            TokenType::String(val) => {
                let v = val.clone();
                self.advance();
                return Box::new(LiteralExpression::new(
                    Literal::String(v),
                ));
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression();
                self.consume(TokenType::RightParen, "Expect ')' after expression.");
                return Box::new(GroupingExpression::new(
                        expr
                ));
            }
            _ => {
                panic!("Unexpected token: {:?}", self.tokens[self.pos]);
            }

        }
    }

    // ********** HELPER FUNCTIONS ********** //
    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn consume(&mut self, token: TokenType, message: &str) -> &Token {
        if self.check(token) {
            return self.advance();
        } else {
            panic!("{}", message);
        }
    }

    fn check(&self, token: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.tokens[self.pos].token_type == token
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn previous (&self) -> &Token {
        &self.tokens[self.pos - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.pos += 1;
        }
        return self.previous();
    }

    fn match_tokens(&mut self, tokens: Vec<TokenType>) -> bool {
        for token in tokens {
            if self.check(token) {
                self.pos += 1;
                return true;
            }
        }
        return false;
    }
}
