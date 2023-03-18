use crate::{tokens::{Token, TokenType}, tree::{Expression, Literal}};


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
    fn evaluate(&self) -> Literal {
        let left = self.left.evaluate();
        let right = self.right.evaluate();

        match &self.op.token_type {
            TokenType::Plus => {
                return match (&left, &right) {
                    (Literal::Number(n), Literal::Number(m)) => Literal::Number(n + m),
                    (Literal::String(n), Literal::String(m)) => {
                        return Literal::String(n.to_owned() + m.as_str())
                    }
                    _ => {
                        panic!("Expected two numbers or two strings, got: {:?} {:?}", left, right);
                    }
                };
            }
            TokenType::Minus => {
                return match (&left, &right) {
                    (Literal::Number(n), Literal::Number(m)) => Literal::Number(n - m),
                    _ => {
                        panic!("Expected two numbers, got: {:?} {:?}", left, right);
                    }
                };
            }
            TokenType::Star => {
                return match (&left, &right) {
                    (Literal::Number(n), Literal::Number(m)) => Literal::Number(n * m),
                    _ => {
                        panic!("Expected two numbers, got: {:?} {:?}", left, right);
                    }
                };
            }
            TokenType::Slash => {
                return match (&left, &right) {
                    (Literal::Number(n), Literal::Number(m)) => Literal::Number(n / m),
                    _ => {
                        panic!("Expected two numbers, got: {:?} {:?}", left, right);
                    }
                };
            }
            TokenType::Greater => {
                return match (&left, &right) {
                    (Literal::Number(n), Literal::Number(m)) => Literal::Boolean(n > m),
                    _ => {
                        panic!("Expected two numbers, got: {:?} {:?}", left, right);
                    }
                };
            }
            TokenType::GreaterEqual => {
                return match (&left, &right) {
                    (Literal::Number(n), Literal::Number(m)) => Literal::Boolean(n >= m),
                    _ => {
                        panic!("Expected two numbers, got: {:?} {:?}", left, right);
                    }
                };
            }
            TokenType::Less => {
                return match (&left, &right) {
                    (Literal::Number(n), Literal::Number(m)) => Literal::Boolean(n < m),
                    _ => {
                        panic!("Expected two numbers, got: {:?} {:?}", left, right);
                    }
                };
            }
            TokenType::LessEqual => {
                return match (&left, &right) {
                    (Literal::Number(n), Literal::Number(m)) => Literal::Boolean(n <= m),
                    _ => {
                        panic!("Expected two numbers, got: {:?} {:?}", left, right);
                    }
                };
            }
            TokenType::BangEqual => {
                return match (&left, &right) {
                    (Literal::Number(n), Literal::Number(m)) => Literal::Boolean(n != m),
                    (Literal::String(n), Literal::String(m)) => Literal::Boolean(n != m),
                    (Literal::Boolean(n), Literal::Boolean(m)) => Literal::Boolean(n != m),
                    _ => {
                        panic!("Expected two numbers, got: {:?} {:?}", left, right);
                    }
                };
            }
            TokenType::EqualEqual => {
                return match (&left, &right) {
                    (Literal::Number(n), Literal::Number(m)) => Literal::Boolean(n == m),
                    (Literal::String(n), Literal::String(m)) => Literal::Boolean(n == m),
                    (Literal::Boolean(n), Literal::Boolean(m)) => Literal::Boolean(n == m),
                    _ => {
                        panic!("Expected two numbers, got: {:?} {:?}", left, right);
                    }
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

