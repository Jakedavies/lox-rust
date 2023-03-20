use crate::{parser::Literal, interpreter::RuntimeError, environment::{self, Environment}};


pub trait Expression: std::fmt::Debug {
    fn evaluate(&self, env: &mut Environment) -> Result<Literal, RuntimeError>;
    fn children(&self) -> Vec<&Box<dyn Expression>>;
}
