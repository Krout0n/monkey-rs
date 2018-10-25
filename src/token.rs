#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Illegal(char),
    EOF,

    Assign,
    Plus,
    Minus,
    Star,

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
    While,
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
        "while" => Token::While,
        "return" => Token::Return,
        "true" => Token::True,
        "false" => Token::False,
        _ => Token::Ident(literal),
    }
}
