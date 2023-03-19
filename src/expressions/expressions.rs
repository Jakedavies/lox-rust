use crate::{parser::Literal, interpreter::RuntimeError};


pub trait Expression: std::fmt::Debug {
    fn evaluate(&self) -> Result<Literal, RuntimeError>;
    fn children(&self) -> Vec<&Box<dyn Expression>>;
}
