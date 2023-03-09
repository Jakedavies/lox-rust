pub fn error(line: usize, message: String) {
    report(line, "".to_string(), message);
}

pub fn report(line: usize, where_: String, message: String) {
    println!("[line {}] Error {}: {}", line, where_, message);
}
