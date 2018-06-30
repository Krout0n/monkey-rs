#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal(char),
    EOF,

    Assign,
    Plus,

    GT,
    LT,

    Bang,
    Eq,
    NotEq,

    LParen,
    RParen,
    LBrace,
    RBrace,

    Semicolon,
    Comma,

    Let,
    Function,
    Ident(String),
    Int(i32),
    If,
    Else,
    Return,
    True,
    False,
}

pub fn lookup_keyword(literal: String) -> Token {
    match &*literal {
        "let" => Token::Let,
        "fn" => Token::Function,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        "true" => Token::True,
        "false" => Token::False,
        _ => Token::Ident(literal),
    }
}
