use crate::tokens::{Token, TokenType};
use crate::error::error;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(token_type, text, self.line));
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source.chars().nth(self.current + 1).unwrap()
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::SemiColon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => {
                if self.match_char('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => {
                while self.peek() != '"' && !self.is_at_end() {
                    if self.peek() == '\n' {
                        self.line += 1;
                    }
                    self.advance();
                }
                if self.is_at_end() {
                    error(self.line, "Unterminated string.".to_string());
                    return;
                }
                self.advance();
                let literal = self.source[self.start + 1..self.current - 1].to_string();
                self.add_token(TokenType::String(literal));
            }
            // numbers
            '0'..='9' => {
                while self.peek().is_digit(10) {
                    self.advance();
                }
                if self.peek() == '.' && self.peek_next().is_digit(10) {
                    self.advance();
                    while self.peek().is_digit(10) {
                        self.advance();
                    }
                }
                let lexeme = self.source[self.start..self.current].to_string();
                // cast string to f64
                let f64_lexeme = lexeme.parse::<f64>().unwrap();
                self.add_token(TokenType::Number(f64_lexeme));
            }
            // identifiers
            'a'..='z' | 'A'..='Z' | '_' => {
                self.identifier()
            }
            default => {
                let msg = format!("Unexpected character. {}", default);
                error(self.line, msg);
            }
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let lexeme = self.source[self.start..self.current].to_string();
        let token_type = match lexeme.as_str() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Idenfitier(lexeme),
        };
        self.add_token(token_type);
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            // we are at the beginning of the next lexeme
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "".to_string(), self.line));

        &self.tokens
    }
}

#[cfg(test)]
mod tests {
    fn basic_token(t: super::TokenType) -> super::Token {
        super::Token::new(t, "".to_string(), 1)
    }

    #[test]
    fn it_skips_comments() {
        let mut scanner = super::Scanner::new("// this is a comment".to_string());
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 1);
        assert_eq!(*tokens.first().unwrap(), basic_token(super::TokenType::EOF));
    }

    #[test]
    fn it_handles_string_literals() {
        let mut scanner = super::Scanner::new("\"hello there\"".to_string());
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 2);
        assert_eq!(
            *tokens.first().unwrap(),
            super::Token::new(
                super::TokenType::String("hello there".to_string()),
                "\"hello there\"".to_string(),
                1
            )
        );
    }

    #[test]
    fn it_handles_number_literals() {
        let mut scanner = super::Scanner::new("10.69".to_string());
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.len(), 2);
        assert_eq!(
            *tokens.first().unwrap(),
            super::Token::new(super::TokenType::Number(10.69), "10.69".to_string(), 1)
        );
    }
}
