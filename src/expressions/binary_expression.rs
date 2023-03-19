use crate::{tokens::{Token, TokenType}, tree::{Expression, Literal, RuntimeError}};


#[derive(Debug)]
pub struct BinaryExpression {
    op: Token,
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl BinaryExpression {
    pub fn new(op: Token, left: Box<dyn Expression>, right: Box<dyn Expression>) -> Self {
        Self { op, left, right }
    }
}

impl Expression for BinaryExpression {
    fn evaluate(&self) -> Result<Literal, RuntimeError> {
        let left = self.left.evaluate()?;
        let right = self.right.evaluate()?;

        match &self.op.token_type {
            TokenType::Plus => match (&left, &right) {
                (Literal::Number(n), Literal::Number(m)) => Ok(Literal::Number(n + m)),
                (Literal::String(n), Literal::String(m)) => Ok(Literal::String(n.to_owned() + m.as_str())),
                _ => Err(RuntimeError::new(format!("Expected two numbers or two strings, got: {:?} {:?}", left, right)))
            },
            TokenType::Minus => match (&left, &right) {
                (Literal::Number(n), Literal::Number(m)) => Ok(Literal::Number(n - m)),
                _ => Err(RuntimeError::new(format!("Expected two numbers, got: {:?} {:?}", left, right)))
            },
            TokenType::Star => match (&left, &right) {
                (Literal::Number(n), Literal::Number(m)) => Ok(Literal::Number(n * m)),
                _ => Err(RuntimeError::new(format!("Expected two numbers, got: {:?} {:?}", left, right)))
            },
            TokenType::Slash => match (&left, &right) {
                (Literal::Number(n), Literal::Number(m)) => Ok(Literal::Number(n / m)),
                _ => Err(RuntimeError::new(format!("Expected two numbers, got: {:?} {:?}", left, right)))
            },
            TokenType::Greater => match (&left, &right) {
                (Literal::Number(n), Literal::Number(m)) => Ok(Literal::Boolean(n > m)),
                _ => Err(RuntimeError::new(format!("Expected two numbers, got: {:?} {:?}", left, right)))
            },
            TokenType::GreaterEqual => match (&left, &right) {
                (Literal::Number(n), Literal::Number(m)) => Ok(Literal::Boolean(n >= m)),
                _ => Err(RuntimeError::new(format!("Expected two numbers, got: {:?} {:?}", left, right)))
            },
            TokenType::Less => match (&left, &right) {
                (Literal::Number(n), Literal::Number(m)) => Ok(Literal::Boolean(n < m)),
                _ => Err(RuntimeError::new(format!("Expected two numbers, got: {:?} {:?}", left, right)))
            },
            TokenType::LessEqual => match (&left, &right) {
                (Literal::Number(n), Literal::Number(m)) => Ok(Literal::Boolean(n <= m)),
                _ => Err(RuntimeError::new(format!("Expected two numbers, got: {:?} {:?}", left, right)))
            },
            TokenType::BangEqual => match (&left, &right) {
                (Literal::Number(n), Literal::Number(m)) => Ok(Literal::Boolean(n != m)),
                (Literal::String(n), Literal::String(m)) => Ok(Literal::Boolean(n != m)),
                (Literal::Boolean(n), Literal::Boolean(m)) => Ok(Literal::Boolean(n != m)),
                _ => Err(RuntimeError::new(format!("Expected two numbers, two strings, or two booleans, got: {:?} {:?}", left, right)))
            },
            TokenType::EqualEqual => {
                return match (&left, &right) {
                    (Literal::Number(n), Literal::Number(m)) => Ok(Literal::Boolean(n == m)),
                    (Literal::String(n), Literal::String(m)) => Ok(Literal::Boolean(n == m)),
                    (Literal::Boolean(n), Literal::Boolean(m)) => Ok(Literal::Boolean(n == m)),
                    _ => Err(RuntimeError::new(format!("Expected two numbers, two strings, or two booleans, got: {:?} {:?}", left, right)))
                };
            }
            _ => {
                panic!("Unexpected token: {:?}", self.op);
            }
        }
    }

    fn children(&self) -> Vec<&Box<dyn Expression>> {
        vec![&self.left, &self.right]
    }
}
