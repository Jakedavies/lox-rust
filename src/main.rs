use tree::Expression;

mod tokens;
mod scanner;
mod error;
mod tree;
mod parser;
mod expressions;

fn run(source: String) {
    // create a scanner
    let mut scanner = scanner::Scanner::new(source);
    // create a parser
    let tokens = scanner.scan_tokens();

    let mut parser = parser::Parser::new(tokens);
    let tree = parser.parse();
    // print all but last token
    tree.print(0);
    println!("> {} ", tree.evaluate());
}

fn print_ast(node: dyn Expression, indent: usize) {
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
        print!("> {}", input);
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

