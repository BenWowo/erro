use erro::scan::{Lexer, Token};

#[test]
fn read_char() {
    let data: &'static str = "abcdefg 1234";
    let mut lexer = Lexer::new(data.chars().collect());

    for ch in data.chars() {
        lexer.read_char();
        assert_eq!(lexer.ch, ch);
    }
    lexer.read_char();
    assert_eq!(lexer.ch, '\0');
}

#[test]
fn next_token() {
    let data: &'static str = "1234";
    let mut lexer = Lexer::new(data.chars().collect());

    assert_eq!(lexer.next_token(), Token::Int(1234));
}

// fn scan() {}
