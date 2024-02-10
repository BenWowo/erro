#[derive(Debug, PartialEq)]
pub enum Token {
    // LogAnd,
    // LogOr,
    // BitAnd,
    // BitOr,
    Plus,
    Minus,
    Asterick,
    Forwardslash,
    // Backslash,
    // LParen,
    // RParen,
    // LBracket,
    // RBracket,
    // LBrace,
    // RBrace,
    Ident(String),
    Int(i64),
    // Float(f64),
    Eof,
    Illegal,
}

pub struct Lexer {
    input: Vec<char>,         // Source code
    pub position: usize,      // Reading position
    pub read_position: usize, // Current moving reading position
    pub ch: char,             // Current read character
}

fn scan_file() {}

impl Lexer {
    pub fn new(input: Vec<char>) -> Self {
        Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        }
    }
    pub fn read_char(&mut self) {
        if self.read_position > self.input.len() - 1 {
            self.ch = '\0';
        } else {
            self.position = self.read_position;
            self.ch = self.input[self.position];
            self.read_position += 1;
        }
    }
    fn peek_char(&mut self) -> char {
        if self.read_position > self.input.len() - 1 {
            '\0'
        } else {
            self.input[self.read_position]
        }
    }
    fn update_position(&mut self) {
        self.position = self.read_position;
        self.read_position += 1;
    }
    fn next_num(&mut self) -> Token {
        while self.peek_char().is_numeric() {
            self.read_position += 1;
        }
        let s: String = self.input[self.position..self.read_position]
            .iter()
            .collect();

        self.update_position();
        Token::Int(s.parse().unwrap())
    }
    pub fn next_token(&mut self) -> Token {
        self.read_char();
        use Token::*;

        match self.ch {
            '+' => Plus,
            '-' => Minus,
            '*' => Asterick,
            '/' => Forwardslash,
            n if n.is_numeric() => self.next_num(),
            '\0' => Eof,
            _ => Illegal,
        }
    }
}
