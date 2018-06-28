#![feature(exclusive_range_pattern)]
#![feature(if_while_or_patterns)]
#![feature(range_contains)]

pub mod token {

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
}

pub mod lexer {
    use token::*;

    #[derive(Debug)]
    pub struct Lexer {
        src: String,
        position: usize,
        read_position: usize,
        ch: Option<char>,
    }

    impl Lexer {
        pub fn new(src: String) -> Lexer {
            let mut l = Lexer {
                src,
                position: 0,
                read_position: 0,
                ch: None,
            };
            l.read_char();
            l
        }

        pub fn next_token(&mut self) -> Token {
            self.read_char();
            self.skip_whitespace();

            match self.ch {
                Some('=') => {
                    if let Some('=') = self.peek_char() {
                        self.read_char();
                        Token::Eq
                    } else {
                        Token::Assign
                    }
                }
                Some('!') => {
                    if let Some('=') = self.peek_char() {
                        self.read_char();
                        Token::NotEq
                    } else {
                        Token::Bang
                    }
                }
                Some('+') => Token::Plus,
                Some('<') => Token::LT,
                Some('>') => Token::GT,
                Some(';') => Token::Semicolon,
                Some('{') => Token::LBrace,
                Some('}') => Token::RBrace,
                Some('(') => Token::LParen,
                Some(')') => Token::RParen,
                Some(',') => Token::Comma,
                None => Token::EOF,
                Some(c) => match c {
                    'a'..='z' | 'A'..='Z' | '_' => {
                        let mut literal = String::new();
                        while let Some('a'..'z') | Some('A'..'Z') | Some('_') = self.ch {
                            literal.push_str(&self.ch.unwrap().to_string());
                            self.read_char();
                        }
                        self.backtick();
                        lookup_keyword(literal)
                    }
                    '0'..='9' => {
                        let mut literal = String::new();
                        while let Some('0'..='9') = self.ch {
                            literal.push_str(&self.ch.unwrap().to_string());
                            self.read_char();
                        }
                        self.backtick();
                        Token::Int(literal.parse::<i32>().unwrap())
                    }
                    _ => Token::Illegal(c),
                },
            }
        }

        fn skip_whitespace(&mut self) {
            while let Some('\t') | Some(' ') | Some('\n') | Some('\r') = self.ch {
                self.read_char();
            }
        }

        fn read_char(&mut self) {
            self.ch = self.src.chars().nth(self.position);
            self.position = self.read_position;
            self.read_position += 1;
        }

        fn backtick(&mut self) {
            self.read_position -= 1;
            self.position = self.read_position - 1;
            self.ch = self.src.chars().nth(self.position);
        }

        fn peek_char(&self) -> Option<char> {
            self.src.chars().nth(self.position)
        }
    }

}

mod tests {
    use super::*;
    use lexer::*;
    use token::*;

    #[test]
    fn some_operand() {
        let expected = vec![
            Token::Plus,
            Token::Semicolon,
            Token::LBrace,
            Token::RBrace,
            Token::LParen,
            Token::RParen,
            Token::EOF,
        ];
        let input = "+;{}()".to_string();

        let mut l = Lexer::new(input);
        for t in expected {
            assert_eq!(l.next_token(), t);
        }
    }

    #[test]
    fn let_ident() {
        let input = "\
                     let five = 5;\n\
                     let ten = 10;\n\
                     let add = fn (x ,y) {\n\
                     x + y;\n\
                     };\n\
                     let result = add(five, ten);"
            .to_string();

        let expected = vec![
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_string()),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".to_string()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident('x'.to_string()),
            Token::Comma,
            Token::Ident('y'.to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Ident('x'.to_string()),
            Token::Plus,
            Token::Ident('y'.to_string()),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".to_string()),
            Token::Assign,
            Token::Ident("add".to_string()),
            Token::LParen,
            Token::Ident("five".to_string()),
            Token::Comma,
            Token::Ident("ten".to_string()),
            Token::RParen,
            Token::Semicolon,
        ];
        let mut l = Lexer::new(input);
        for (i, t) in expected.into_iter().enumerate() {
            let result = l.next_token();
            assert_eq!(result, t);
        }
    }

    #[test]
    fn add_some_keywords() {
        let input = "\
                     if (5 < 10) {\
                     return true;\
                     } else {\
                     return false;\
                     }"
            .to_string();
        let expected = vec![
            Token::If,
            Token::LParen,
            Token::Int(5),
            Token::LT,
            Token::Int(10),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RBrace,
        ];
        let mut l = Lexer::new(input);
        for (i, t) in expected.into_iter().enumerate() {
            let result = l.next_token();
            assert_eq!(result, t);
        }
    }

    #[test]
    fn add_eq_not_eq() {
        let input = "\
        9
        !true;
        10==10;\
        10 != 9;"
            .to_string();

        let expected = vec![
            Token::Int(9),
            Token::Bang,
            Token::True,
            Token::Semicolon,
            Token::Int(10),
            Token::Eq,
            Token::Int(10),
            Token::Semicolon,
            Token::Int(10),
            Token::NotEq,
            Token::Int(9),
            Token::Semicolon,
        ];

        let mut l = Lexer::new(input);
        for (i, t) in expected.into_iter().enumerate() {
            let result = l.next_token();
            assert_eq!(result, t);
        }
    }

}

mod repl {
    use lexer::*;
    use token::*;

    use std::io::{self, stdin, Read, Write};
    // 入/出力が標準入/出力じゃなくてもいいようにする
    pub fn start() {
        println!("Yo this is a Monkey programming language REPL!");
        println!("Feel free to type some statement!");
        loop {
            print!(">> ");
            io::stdout().flush().unwrap();
            let mut src = String::new();
            let mut l = Lexer::new(read_input());
            loop {
                let t = l.next_token();
                println!("{:?}", t);
                if t == Token::EOF {
                    break;
                }
            }
        }
    }

    fn read_input() -> String {
        let mut s = String::new();
        stdin().read_line(&mut s).expect("failed to read stdin");
        s
    }
}

use lexer::*;
use token::*;

fn main() {
    repl::start();
}
