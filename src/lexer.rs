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

    fn peek(self) -> char {
        if self.check_at_end() {
            None
        }
        self.source_code[self.read_pos]
    }

    fn check_next_token(&mut self, ch: char) -> bool {
        if self.check_at_end() {
            return false
        }

        if self.source_code[self.read_pos] == char {
            self.read_pos();
            return true
        }

        return false
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
            '!' => {
                if self.check_next_token('=') {
                    self.add_token(TokenType::BangEqual, String::from("!="))
                } else {
                    self.add_token(TokenType::Bang, String::from("!"))
                }
            },
            '=' => {
                if self.check_next_token('=') {
                    self.add_token(TokenType::EqualEqual, String::from("=="))
                } else {
                    self.add_token(TokenType::Equal, String::from("="))
                }
            },
            '<' => {
                if self.check_next_token('=') {
                    self.add_token(TokenType::LesserEqual, String::from("<="))
                } else {
                    self.add_token(TokenType::Lesser, String::from("<"))
                }
            },
            '>' => {
                if self.check_next_token('=') {
                    self.add_token(TokenType::GreaterEqual, String::from(">="))
                } else {
                    self.add_token(TokenType::Greater, String::from(">"))
                }
            },
            '/' => {
                if self.check_next_token('/') {
                    // It is a comment. Skip till newline.
                    while self.check_at_end() && self.peek() != '\n' {
                        self.read_token();
                    }
                }
                else {
                    self.add_token(TokenType::Slash, String::from("/"))
                }
            },
            '\n' => self.line += 1,
            // Do nothing for spaces.
            ' ' | '\t' | '\r' => None,
            _ => panic!("{}, Unexpected character.{}", self.line, ch)
        }
    }
}
