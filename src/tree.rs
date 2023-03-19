use std::any::Any;
use std::fmt::Debug;
use std::{fmt, default};

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

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub message: String,
}

impl RuntimeError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

pub trait Expression: std::fmt::Debug {
    fn evaluate(&self) -> Result<Literal, RuntimeError>;
    fn children(&self) -> Vec<&Box<dyn Expression>>;
}
