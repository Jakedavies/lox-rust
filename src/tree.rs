use std::fmt;

use crate::tokens::Token;


#[derive(Debug)]
pub enum Literal {
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

#[derive(Debug)]
pub enum Node {
    UnaryExpr {
        op: Token,
        child: Box<Node>,
    },
    BinaryExpr {
        op: Token,
        left: Box<Node>,
        right: Box<Node>,
    },
    GroupingExpr {
        child: Box<Node>,
    },
    LiteralExpr {
        value: Literal,
    },
}


impl Node {
    pub fn print(&self, indent: usize) {
        match self {
            Node::UnaryExpr { op, child } => {
                println!("{}UnaryExpr", " ".repeat(indent));
                println!("{}op: {:?}", " ".repeat(indent + 2), op);
                child.print(indent + 2);
            }
            Node::BinaryExpr { op, left, right } => {
                println!("{}BinaryExpr", " ".repeat(indent));
                println!("{}op: {:?}", " ".repeat(indent + 2), op);
                left.print(indent + 2);
                right.print(indent + 2);
            }
            Node::GroupingExpr { child } => {
                println!("{}GroupingExpr", " ".repeat(indent));
                child.print(indent + 2);
            }
            Node::LiteralExpr { value } => {
                println!("{}LiteralExpr", " ".repeat(indent));
                println!("{}value: {}", " ".repeat(indent + 2), value);
            }
        }
    }
}
