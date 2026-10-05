use crate::tokens::{Object, Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        let token = Token {
            token_type: TokenType::Eof,
            lexeme: String::new(),
            literal: None,
            line: self.line,
        };
        self.tokens.push(token);
        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.char_at(self.current);
        self.current += 1;
        c
    }

    fn char_at(&self, index: usize) -> char {
        self.source
            .chars()
            .nth(index)
            .expect("Tried to scan past the end of the source string!")
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_literal_token(token_type, None);
    }

    fn add_literal_token(&mut self, token_type: TokenType, literal: Option<Object>) {
        let lexeme = self.source[self.start..self.current].to_string();
        let token = Token {
            token_type,
            lexeme,
            literal,
            line: self.line,
        };
        self.tokens.push(token);
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            _ => unimplemented!("Only parens are implemented for now. Found: {}", c),
        }
    }
}
