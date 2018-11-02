
pub struct Lexer {
    source_code: Vec<char>,
    read_pos: int,
    peek_pos: int
}

impl Lexer {
    pub fn new(source_code: &Vec<char>) -> Lexer {
        Lexer {
            source_code,
            read_pos: 0,
            peek_pos: 0
        }
    }
}
