use crate::tree::{Literal, Expression};

#[derive(Debug)]
pub struct LiteralExpression {
    value: Literal,
}

impl LiteralExpression {
    pub fn new(value: Literal) -> Self {
        Self { value }
    }
}

impl Expression for LiteralExpression {
    fn evaluate(&self) -> Literal {
        self.value.clone()
    }

    fn children(&self) -> Vec<&Box<dyn Expression>> {
        vec![]
    }
}
