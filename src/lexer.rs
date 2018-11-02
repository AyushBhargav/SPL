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

    fn check_at_end(&self) -> bool{
        self.read_pos >= self.source_code.len()
    }

    fn read_token(&mut self) -> char {
        self.read_pos += 1;
        self.source_code[self.read_pos - 1]
    }

    fn add_token(&mut self, token_type: TokenType, token_value: String) {
        let line = self.line;
        self.tokens.push(Token{
            value: token_value,
            token_type,
            line
        });
        self.start_pos = self.read_pos - 1;
    }

    fn peek(&self) -> char {
        if self.check_at_end() {
            return '\0';
        }
        self.source_code[self.read_pos]
    }

    fn peek_next(&self) -> char {
        if self.read_pos + 1 >= self.source_code.len() {
            return '\0';
        }
        self.source_code[self.read_pos + 1]
    }

    fn check_next_token(&mut self, ch: char) -> bool {
        if self.check_at_end() {
            return false
        }

        if self.source_code[self.read_pos] == ch {
            self.read_token();
            return true
        }

        return false
    }

    fn check_if_digit(ch: char) -> bool {
        if ch >= '0' && ch <= '9' {
            return true;
        }
        false
    }

    fn check_if_alpha(ch: char) -> bool {
        if (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') {
            return true;
        }
        return false
    }

    fn check_if_alphanumeric(ch: char) -> bool {
        if Lexer::check_if_digit(ch) || Lexer::check_if_alpha(ch) {
            return true;
        }
        return false
    }

    fn read_identifier(identifier: &String) -> TokenType {
        match identifier.as_ref() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "func" => TokenType::Func,
            "if" => TokenType::If,
            "null" => TokenType::Null,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier
        }
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
                    while !self.check_at_end() && self.peek() != '\n' {
                        self.read_token();
                    }
                }
                else {
                    self.add_token(TokenType::Slash, String::from("/"))
                }
            },
            '\n' => self.line += 1,
            // Do nothing for spaces.
            ' ' | '\t' | '\r' => (),
            '"' => {
                while !self.check_at_end() && self.peek() != '"' {
                    if self.peek() == '\n' {
                        self.line += 1;
                    }
                    self.read_token();
                }
                if self.check_at_end() {
                    panic!("{}, Unterminated String.", self.line)
                } else {
                    let value: String = self.source_code[self.start_pos + 1..self.read_pos - 1]
                                        .into_iter().collect();
                    self.add_token(TokenType::String, value)
                }
            },
            _ => {
                if Lexer::check_if_digit(ch) {
                    while !self.check_at_end() && Lexer::check_if_digit(self.peek()) {
                        self.read_token();
                    }
                    // Decimal number
                    if self.peek() == '.' && Lexer::check_if_digit(self.peek_next()) {
                        self.read_token(); // Consume .
                        while !self.check_at_end() && Lexer::check_if_digit(self.peek()) {
                            self.read_token();
                        }
                    }

                    let value: String = self.source_code[self.start_pos..self.read_pos].into_iter()
                                        .collect();
                    self.add_token(TokenType::Number, value)

                } else if Lexer::check_if_alpha(ch) {
                    while Lexer::check_if_alphanumeric(self.peek()) {
                        self.read_token();
                    }
                    let value: String = self.source_code[self.start_pos..self.read_pos].into_iter()
                        .collect();

                    // TokenType can be reserved word too.
                    self.add_token(Lexer::read_identifier(&value), value)
                } else {
                    panic!("{}, Unexpected character.{}", self.line, ch)
                }
            }
        }
    }

    // TODO: Complete this function to scan whole code.
    pub fn lex_source_code(&mut self) -> Vec<Token> {
        while !self.check_at_end() {
            self.scan_token();
        }
        self.tokens
    }
}
