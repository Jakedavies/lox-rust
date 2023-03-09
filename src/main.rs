
mod tokens;
mod scanner;
mod error;

fn run(source: String) {
    // create a scanner
    let mut scanner = scanner::Scanner::new(source);
    // create a parser
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
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

