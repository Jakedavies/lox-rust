use crate::{tree::{Expression, Literal}, tokens::{Token, TokenType}};


#[derive(Debug)]
pub struct UnaryExpression {
    op: Token,
    child: Box<dyn Expression>,
}

impl UnaryExpression {
    pub fn new(op: Token, child: Box<dyn Expression>) -> Self {
        Self { op, child }
    }
}

impl Expression for UnaryExpression {
    fn evaluate(&self) -> Literal {
        let child = self.child.evaluate();
        match &self.op.token_type {
            TokenType::Minus => {
                if let Literal::Number(n) = child {
                    return Literal::Number(-n);
                }

                panic!("Expected number, got: {:?}", child);
            }
            TokenType::Bang => {
                return match child {
                    Literal::Boolean(b) => Literal::Boolean(!b),
                    Literal::Number(n) => Literal::Boolean(n == 0.0),
                    Literal::String(s) => Literal::Boolean(s.len() == 0),
                };
            }
            _ => {
                panic!("Unexpected token: {:?}", self.op);
            }
        }
    }

    fn children(&self) -> Vec<&Box<dyn Expression>> {
        vec![&self.child]
    }
}
