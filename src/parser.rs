use std::fmt::{self, Display};

use crate::{
    expressions::{
        assignment_expression::AssignmentExpression, binary_expression::BinaryExpression,
        expressions::Expression, grouping_expression::GroupingExpression,
        literal_expression::LiteralExpression, unary_expression::UnaryExpression,
        var_expression::VarExpression,
    },
    statement::{ExpressionStatement, PrintStatement, Statement, VarStatement, BlockStatement},
    tokens::{Token, TokenType},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
}

#[derive(Debug, Clone)]
struct ParseError {
    message: String,
}

impl ParseError {
    fn new(token: Token, message: &str) -> ParseError {
        ParseError {
            message: format!("{} at '{}' on line {}", message, token.lexeme, token.line),
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

type Result<T> = std::result::Result<Box<T>, Box<ParseError>>;

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Number(n) => write!(f, "{}", n),
            Literal::String(s) => write!(f, "{}", s),
            Literal::Boolean(b) => write!(f, "{}", b),
        }
    }
}

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

    pub fn parse(&mut self) -> Vec<Box<dyn Statement>> {
        let mut statements: Vec<Box<dyn Statement>> = vec![];
        while !self.is_at_end() {
            let result = self.declaration();
            if result.is_ok() {
                statements.push(result.unwrap());
            } else {
                println!("Statements so far: {:?}", statements);
                panic!("Error: {}", result.err().unwrap());
            }
        }
        return statements;
    }

    fn declaration(&mut self) -> Result<dyn Statement> {
        if self.match_tokens(vec![TokenType::Var]) {
            return self.var_declaration();
        }

        return self.statement();
    }

    fn var_declaration(&mut self) -> Result<dyn Statement> {
        let identifier = self.consume(
            TokenType::Idenfitier("".to_string()),
            "Expect variable name.",
        );
        let identifier_lexeme = identifier.lexeme.clone();
        self.consume(TokenType::Equal, "Expect '=' after variable name.");

        let initializer = self.expression()?;

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        );
        return Ok(Box::new(VarStatement::new(identifier_lexeme, initializer)));
    }

    fn statement(&mut self) -> Result<dyn Statement> {
        if self.match_tokens(vec![TokenType::Print]) {
            return self.print_statement();
        }
        if self.match_tokens(vec![TokenType::LeftBrace]) {
            return self.block_statement();
        }

        return self.expression_statement();
    }

    fn print_statement(&mut self) -> Result<dyn Statement> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.");
        return Ok(Box::new(PrintStatement::new(value)));
    }

    fn block_statement(&mut self) -> Result<dyn Statement> {
        let mut statements: Vec<Box<dyn Statement>> = vec![];
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            let result = self.declaration();
            if result.is_ok() {
                statements.push(result.unwrap());
            } else {
                panic!("Error: {}", result.err().unwrap());
            }
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.");
        return Ok(Box::new(BlockStatement::new(statements)));
    }

    fn expression_statement(&mut self) -> Result<dyn Statement> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.");
        return Ok(Box::new(ExpressionStatement::new(expr)));
    }

    fn expression(&mut self) -> Result<dyn Expression> {
        return self.assignment();
    }

    fn assignment(&mut self) -> Result<dyn Expression> {
        let expr = self.equality()?;

        if self.match_tokens(vec![TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            // check if expr is a VarExpression
            if let Some(var_expr) = expr.downcast_ref::<VarExpression>() {
                let name = var_expr.name.clone();
                return Ok(Box::new(AssignmentExpression::new(name, value)));
            }

            return Err(Box::new(ParseError::new(
                equals,
                "Invalid assignment target.",
            )));
        }

        return Ok(expr);
    }

    fn equality(&mut self) -> Result<dyn Expression> {
        let mut expr = self.comparison();

        while self.match_tokens(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous().clone();
            let right = self.comparison();
            expr = Ok(Box::new(BinaryExpression::new(op, expr?, right?)));
        }

        return expr;
    }

    fn comparison(&mut self) -> Result<dyn Expression> {
        let mut expr = self.term();

        while self.match_tokens(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let op = self.previous().clone();
            let right = self.term();
            expr = Ok(Box::new(BinaryExpression::new(op, right?, expr?)));
        }

        return expr;
    }

    fn term(&mut self) -> Result<dyn Expression> {
        let mut expr = self.factor();

        while self.match_tokens(vec![TokenType::Minus, TokenType::Plus]) {
            let op = self.previous().clone();
            let right = self.factor();
            expr = Ok(Box::new(BinaryExpression::new(op, right?, expr?)));
        }

        return expr;
    }

    fn factor(&mut self) -> Result<dyn Expression> {
        let mut expr = self.unary();

        while self.match_tokens(vec![TokenType::Slash, TokenType::Star]) {
            let op = self.previous().clone();
            let right = self.unary();
            expr = Ok(Box::new(BinaryExpression::new(op, right?, expr?)));
        }

        return expr;
    }

    fn unary(&mut self) -> Result<dyn Expression> {
        if self.match_tokens(vec![TokenType::Bang, TokenType::Minus]) {
            let op = self.previous().clone();
            let right = self.unary();
            return Ok(Box::new(UnaryExpression::new(op, right?)));
        }

        return self.primary();
    }

    fn primary(&mut self) -> Result<dyn Expression> {
        let token_type = self.tokens[self.pos].token_type.clone();
        match token_type {
            TokenType::False => {
                self.advance();
                return Ok(Box::new(LiteralExpression::new(Literal::Boolean(false))));
            }
            TokenType::True => {
                self.advance();
                return Ok(Box::new(LiteralExpression::new(Literal::Boolean(true))));
            }
            TokenType::Number(val) => {
                let v = val.clone();
                self.advance();
                return Ok(Box::new(LiteralExpression::new(Literal::Number(v))));
            }
            TokenType::String(val) => {
                let v = val.clone();
                self.advance();
                return Ok(Box::new(LiteralExpression::new(Literal::String(v))));
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression();
                self.consume(TokenType::RightParen, "Expect ')' after expression.");
                return Ok(Box::new(GroupingExpression::new(expr?)));
            }
            TokenType::Idenfitier(val) => {
                self.advance();
                return Ok(Box::new(VarExpression::new(val.clone())));
            }
            _ => {
                return Err(Box::new(ParseError::new(
                    self.peek().clone(),
                    "Expect expression.",
                )));
            }
        }
    }

    // ********** HELPER FUNCTIONS ********** //
    fn is_at_end(&self) -> bool {
        return self.peek().token_type == TokenType::EOF;
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
        std::mem::discriminant(&self.tokens[self.pos].token_type) == std::mem::discriminant(&token)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn previous(&self) -> &Token {
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
