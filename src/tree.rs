use std::fmt::Debug;
use std::{fmt, default};

use crate::tokens::{Token, TokenType};
use crate::expressions::literal_expression::LiteralExpression;

#[derive(Debug, Clone)]
pub enum Literal{
    Number(f64),
    String(String),
    Boolean(bool),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Number(n) => write!(f, "{}", n),
            Literal::String(s) => write!(f, "{}", s),
            Literal::Boolean(b) => write!(f, "{}", b),
        }
    }
}

pub trait Expression {
    fn evaluate(&self) -> Literal;
}

impl Debug for dyn Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Evaluatable")
    }
}
