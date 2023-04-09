use std::fmt::{self, Display};

use crate::{
    expressions::{
        assignment_expression::AssignmentExpression,
        binary_expression::BinaryExpression,
        call_expression::CallExpression,
        expressions::Expression,
        grouping_expression::GroupingExpression,
        literal_expression::LiteralExpression,
        logical_expression::{LogicalExpression, LogicalExpressionOperator},
        unary_expression::UnaryExpression,
        var_expression::VarExpression,
    },
    statement::{
        BlockStatement, BreakStatement, ExpressionStatement, FunctionStatement,
        IfStatement, PrintStatement, Statement, VarStatement, WhileStatement,
    },
    tokens::{Token, TokenType},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
}

impl Literal {
    pub fn is_truthy(&self) -> bool {
        match self {
            Literal::Boolean(b) => *b,
            Literal::Number(n) => *n != 0.0,
            Literal::String(s) => !s.is_empty(),
        }
    }
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

    pub fn parse(&mut self) -> Vec<Box<Statement>> {
        let mut statements: Vec<Box<Statement>> = vec![];
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

    fn declaration(&mut self) -> Result<Statement> {
        if self.match_tokens(vec![TokenType::Var]) {
            return self.var_declaration();
        }

        if self.match_tokens(vec![TokenType::Fun]) {
            return self.fun_declaration();
        }

        return self.statement();
    }

    fn var_declaration(&mut self) -> Result<Statement> {
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
        return Ok(Box::new(Statement::Var(VarStatement::new(identifier_lexeme, initializer))));
    }

    fn fun_declaration(&mut self) -> Result<Statement> {
        let identifier = self.consume(
            TokenType::Idenfitier("".to_string()),
            "Expect function name.",
        );
        let identifier_lexeme = identifier.lexeme.clone();
        self.consume(TokenType::LeftParen, "Expect '(' after function name.");
        let mut parameters: Vec<String> = vec![];
        if !self.check(TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    panic!("Cannot have more than 255 parameters.");
                }
                parameters.push(
                    self.consume(
                        TokenType::Idenfitier("".to_string()),
                        "Expect parameter name.",
                    )
                    .lexeme
                    .clone(),
                );
                if !self.match_tokens(vec![TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters.");
        self.consume(TokenType::LeftBrace, "Expect '{' before function body.");
        let body = self.block_statement()?;
        return Ok(Box::new(Statement::Function(FunctionStatement::new(
            identifier_lexeme,
            parameters,
            body,
        ))));
    }

    fn statement(&mut self) -> Result<Statement> {
        if self.match_tokens(vec![TokenType::For]) {
            return self.for_statement();
        }
        if self.match_tokens(vec![TokenType::If]) {
            return self.if_statement();
        }
        if self.match_tokens(vec![TokenType::Print]) {
            return self.print_statement();
        }
        if self.match_tokens(vec![TokenType::While]) {
            return self.while_statement();
        }

        if self.match_tokens(vec![TokenType::Break]) {
            return self.break_statement();
        }

        if self.match_tokens(vec![TokenType::LeftBrace]) {
            return self.block_statement();
        }

        return self.expression_statement();
    }

    fn if_statement(&mut self) -> Result<Statement> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.");
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.");

        let then_branch = self.statement()?;
        let else_branch = if self.match_tokens(vec![TokenType::Else]) {
            Some(self.statement()?)
        } else {
            None
        };

        return Ok(Box::new(Statement::If(IfStatement::new(
            condition,
            then_branch,
            else_branch,
        ))));
    }

    fn break_statement(&mut self) -> Result<Statement> {
        self.consume(TokenType::Semicolon, "Expect ';' after break.");
        return Ok(Box::new(Statement::Break(BreakStatement::new())));
    }

    fn print_statement(&mut self) -> Result<Statement> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.");
        return Ok(Box::new(Statement::Print(PrintStatement::new(value))));
    }

    fn block_statement(&mut self) -> Result<Statement> {
        let mut statements: Vec<Box<Statement>> = vec![];
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            let result = self.declaration();
            if result.is_ok() {
                statements.push(result.unwrap());
            } else {
                panic!("Error: {}", result.err().unwrap());
            }
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.");
        return Ok(Box::new(Statement::Block(BlockStatement::new(statements))));
    }

    fn while_statement(&mut self) -> Result<Statement> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.");
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.");
        let body = self.statement()?;
        return Ok(Box::new(Statement::While(WhileStatement::new(condition, body))));
    }

    fn for_statement(&mut self) -> Result<Statement> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.");
        let initializer = if self.match_tokens(vec![TokenType::Semicolon]) {
            None
        } else if self.match_tokens(vec![TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if !self.check(TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.");

        let increment = if !self.check(TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.");

        let mut body = self.statement()?;

        // if there is a increment, do it after the main body
        if let Some(increment) = increment {
            body = Box::new(Statement::Block(BlockStatement::new(vec![
                body,
                Box::new(Statement::Expression(ExpressionStatement::new(increment))),
            ])));
        };

        // if there is a condition, we wrap the statement in a loop
        if let Some(condition) = condition {
            body = Box::new(Statement::While(WhileStatement::new(condition, body)));
        } else {
            body = Box::new(Statement::While(WhileStatement::new(
                Box::new(Expression::Literal(LiteralExpression::new(Literal::Boolean(true)))),
                body,
            )));
        };

        // if there is an initializer, we wrap the statement in a block
        if let Some(initializer) = initializer {
            body = Box::new(Statement::Block(BlockStatement::new(vec![initializer, body])));
        };

        return Ok(body);
    }

    fn expression_statement(&mut self) -> Result<Statement> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.");
        return Ok(Box::new(Statement::Expression(ExpressionStatement::new(expr))));
    }

    fn expression(&mut self) -> Result<Expression> {
        return self.assignment();
    }

    fn assignment(&mut self) -> Result<Expression> {
        let expr = self.or()?;

        if self.match_tokens(vec![TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            // check if expr is a VarExpression
            if let Expression::Var(var_expr) = *expr {
                let name = var_expr.name.clone();
                return Ok(Box::new(Expression::Assignment(AssignmentExpression::new(name, value))));
            }

            return Err(Box::new(ParseError::new(
                equals,
                "Invalid assignment target.",
            )));
        }

        return Ok(expr);
    }

    fn or(&mut self) -> Result<Expression> {
        let mut expr = self.and();

        while self.match_tokens(vec![TokenType::Or]) {
            let op = self.previous().clone();
            let right = self.and();
            expr = Ok(Box::new(Expression::Logical(LogicalExpression::new(
                expr?,
                right?,
                LogicalExpressionOperator::Or,
            ))));
        }

        return expr;
    }

    fn and(&mut self) -> Result<Expression> {
        let mut expr = self.equality();

        while self.match_tokens(vec![TokenType::And]) {
            let op = self.previous().clone();
            let right = self.equality();
            expr = Ok(Box::new(Expression::Logical(LogicalExpression::new(
                expr?,
                right?,
                LogicalExpressionOperator::And,
            ))));
        }

        return expr;
    }

    fn equality(&mut self) -> Result<Expression> {
        let mut expr = self.comparison();

        while self.match_tokens(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let op = self.previous().clone();
            let right = self.comparison();
            expr = Ok(Box::new(Expression::Binary(BinaryExpression::new(op, expr?, right?))));
        }

        return expr;
    }

    fn comparison(&mut self) -> Result<Expression> {
        let mut expr = self.term();

        while self.match_tokens(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let op = self.previous().clone();
            let right = self.term();
            expr = Ok(Box::new(Expression::Binary(BinaryExpression::new(op, expr?, right?))));
        }

        return expr;
    }

    fn term(&mut self) -> Result<Expression> {
        let mut expr = self.factor();

        while self.match_tokens(vec![TokenType::Minus, TokenType::Plus]) {
            let op = self.previous().clone();
            let right = self.factor();
            expr = Ok(Box::new(Expression::Binary(BinaryExpression::new(op, expr?, right?))));
        }

        return expr;
    }

    fn factor(&mut self) -> Result<Expression> {
        let mut expr = self.unary();

        while self.match_tokens(vec![TokenType::Slash, TokenType::Star]) {
            let op = self.previous().clone();
            let right = self.unary();
            expr = Ok(Box::new(Expression::Binary(BinaryExpression::new(op, expr?, right?))));
        }

        return expr;
    }

    fn unary(&mut self) -> Result<Expression> {
        if self.match_tokens(vec![TokenType::Bang, TokenType::Minus]) {
            let op = self.previous().clone();
            let right = self.unary();
            return Ok(Box::new(Expression::Unary(UnaryExpression::new(op, right?))));
        }

        return self.call();
    }

    fn call(&mut self) -> Result<Expression> {
        let mut expr = self.primary()?;

        loop {
            if self.match_tokens(vec![TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        return Ok(expr);
    }

    fn finish_call(&mut self, callee: Box<Expression>) -> Result<Expression> {
        let mut arguments = Vec::new();

        if !self.check(TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    return Err(Box::new(ParseError::new(
                        self.peek().clone(),
                        "Can't have more than 255 arguments.",
                    )));
                }

                arguments.push(self.expression()?);

                if !self.match_tokens(vec![TokenType::Comma]) {
                    break;
                }
            }
        };

        let paren = self.consume(TokenType::RightParen, "Expect ) after arguments.");

        let c = CallExpression::new(callee, (*paren).clone(), arguments);
        return Ok(Box::new(Expression::Call(c)));
    }

    fn primary(&mut self) -> Result<Expression> {
        let token_type = self.tokens[self.pos].token_type.clone();
        match token_type {
            TokenType::False => {
                self.advance();
                return Ok(Box::new(Expression::Literal(LiteralExpression::new(
                    Literal::Boolean(false),
                ))));
            }
            TokenType::True => {
                self.advance();
                return Ok(Box::new(Expression::Literal(LiteralExpression::new(Literal::Boolean(true)))));
            }
            TokenType::Number(val) => {
                let v = val.clone();
                self.advance();
                return Ok(Box::new(Expression::Literal(LiteralExpression::new(Literal::Number(v)))));
            }
            TokenType::String(val) => {
                let v = val.clone();
                self.advance();
                return Ok(Box::new(Expression::Literal(LiteralExpression::new(Literal::String(v)))));
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression();
                self.consume(TokenType::RightParen, "Expect ')' after expression.");
                return Ok(Box::new(Expression::Grouping(GroupingExpression::new(expr?))));

            }
            TokenType::Idenfitier(val) => {
                self.advance();
                return Ok(Box::new(Expression::Var(VarExpression::new(val.clone()))));
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
