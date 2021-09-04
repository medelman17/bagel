pub mod token;
use token::{Token, TokenType};

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: i16,
    current: i16,
    line: i16,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            source: source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 0,
        }
    }

    fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
        }
    }

    fn scan_token(&mut self) {}

    fn is_at_end(&mut self) -> bool {
        return self.current >= self.source.len() as i16;
    }

    fn advance(&mut self) -> char {
        return self.source.chars().nth(self.current as usize).unwrap();
    }

    fn add_token(&mut self, t: TokenType) {
        // Slice source to add to token based off start and current
        let token = Token::new(t, self.line, "".to_string());
        self.tokens.push(token)
    }

    // Need to add overloaded functionality in scanner class in book
}
