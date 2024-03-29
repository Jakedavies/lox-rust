use std::{rc::Rc, cell::RefCell};

use expressions::expressions::{Expression, ExpressionResult, Callable};
use statement::Statement;

mod tokens;
mod scanner;
mod error;
mod parser;
mod expressions;
mod interpreter;
mod statement;
mod environment;

fn interpret(statements: Vec<Box<Statement>>) {
    let env = &mut environment::Environment::new();

    // Define built-in functions
    env.define("clock".to_string(), ExpressionResult::Callable(Callable::Clock));

    for statement in statements {
        if let Err(e) = statement.execute(env) {
            println!("RuntimeError: {}", e.message);
        }
    }
}

fn run(source: String) {
    // create a scanner
    let mut scanner = scanner::Scanner::new(source);
    // create a parser
    let tokens = scanner.scan_tokens();


    let mut parser = parser::Parser::new(tokens);
    let tree = parser.parse();

    //println!("{:?}", tree);

    // print the result
    interpret(tree)
}

// helper method for debugging AST
fn print_ast(node: &Expression, indent: usize) {
    // print the node
    for _ in 0..indent {
        print!("  ");
    }
    println!("{:?}", node);
    // print the children
    for child in node.children() {
        print_ast(child, indent + 2);
    }
}


fn run_file(filename: String) {
    // read the file
    let contents = std::fs::read_to_string(filename).expect("Something went wrong reading the file");
    // run the file
    run(contents);
}

fn run_prompt() {
    // loop until user types exit
    loop {
        // get input from user
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        // run the input
        //print!("> {}", input);
        run(input);
    }
}

fn main() {
    // take a file to read first, first arg
    let filename = std::env::args().nth(1);

    // if there is a file, run it
    if filename.is_some() {
        run_file(filename.unwrap());
    } else {
        // if there is no file, run the prompt
        run_prompt();
    }
}

