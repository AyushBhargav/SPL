use token::*;

pub struct Lexer {
    source_code: Vec<char>,
    read_pos: usize,
    start_pos: usize,
    line: i32,
    tokens: Vec<Token>
}

impl Lexer {

    pub fn new(source_code: &Vec<char>) -> Lexer {
        let source_code= source_code.to_vec();
        let mut tokens: Vec<Token> = Vec::new();
        Lexer {
            source_code,
            read_pos: 0,
            start_pos: 0,
            line: 1,
            tokens
        }
    }

    fn check_at_end(self) -> bool{
        self.read_pos >= self.source_code.len()
    }

    fn read_token(&mut self) -> char {
        self.read_pos += 1;
        self.source_code[self.read_pos - 1]
    }

    fn add_token(&mut self, token_type: TokenType, token_value: String) {
        let value: String = self.source_code[self.start_pos..self.read_pos].into_iter().collect();
        let line = self.line;
        self.tokens.push(Token{
            value,
            token_type,
            line
        });
    }

    fn scan_token(&mut self) {
        let ch = self.read_token();
        match ch {
            '(' => self.add_token(TokenType::LeftParen, String::from("(")),
            ')' => self.add_token(TokenType::RightParen, String::from(")")),
            '{' => self.add_token(TokenType::LeftBrace, String::from("{")),
            '}' => self.add_token(TokenType::RightBrace, String::from("}")),
            ',' => self.add_token(TokenType::Comma, String::from(",")),
            '.' => self.add_token(TokenType::Dot, String::from(".")),
            '-' => self.add_token(TokenType::Minus, String::from("-")),
            '+' => self.add_token(TokenType::Plus, String::from("+")),
            ';' => self.add_token(TokenType::Semicolon, String::from(";")),
            '*' => self.add_token(TokenType::Star, String::from("*")),
            // TODO: Add support for multi character literal.
            '!' => self.add_token(TokenType::Star, String::from("*")),
            '=' => self.add_token(TokenType::Star, String::from("*")),
            '<' => self.add_token(TokenType::Star, String::from("*")),
            '>' => self.add_token(TokenType::Star, String::from("*")),
            _ => panic!("{}, Unexpected character.{}", self.line, ch)
        }
    }
}
