pub mod token {

    #[derive(Debug, PartialEq)]
    pub struct Token {
        kind: TokenKind,
        literal: String,
    }

    #[derive(Debug, PartialEq)]
    pub enum TokenKind {
        ILLEGAL,
        EOF,

        ASSIGN,
        PLUS,

        LPAREN,
        RPAREN,
        LBRACE,
        RBRACE,

        SEMICOLON,
        COLON,
    }

    impl Token {
        pub fn new(kind: TokenKind, literal: String) -> Token {
            Token { kind, literal }
        }
    }
}

pub mod lexer {
    use token::*;

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

            match self.ch {
                Some('=') => Token::new(TokenKind::ASSIGN, self.ch.unwrap().to_string()),
                Some('+') => Token::new(TokenKind::PLUS, self.ch.unwrap().to_string()),
                Some(';') => Token::new(TokenKind::SEMICOLON, self.ch.unwrap().to_string()),
                Some('{') => Token::new(TokenKind::LBRACE, self.ch.unwrap().to_string()),
                Some('}') => Token::new(TokenKind::RBRACE, self.ch.unwrap().to_string()),
                Some('(') => Token::new(TokenKind::LPAREN, self.ch.unwrap().to_string()),
                Some(')') => Token::new(TokenKind::RPAREN, self.ch.unwrap().to_string()),
                None => Token::new(TokenKind::EOF, "".to_string()),
                _ => Token::new(TokenKind::ILLEGAL, self.ch.unwrap().to_string()),
            }
        }

        fn read_char(&mut self) {
            if self.read_position > self.src.len() {
                self.ch = None
            } else {
                self.ch = self.src.chars().nth(self.position);
                self.position = self.read_position;
                self.read_position += 1;
            }
        }
    }

}

mod tests {
    use super::*;
    use token::*;
    use lexer::*;

    #[test]
    fn some_operand() {
        let expected = vec![
            Token::new(TokenKind::PLUS, "+".to_string()),
            Token::new(TokenKind::SEMICOLON, ";".to_string()),
            Token::new(TokenKind::LBRACE, "{".to_string()),
            Token::new(TokenKind::RBRACE, "}".to_string()),
            Token::new(TokenKind::LPAREN, "(".to_string()),
            Token::new(TokenKind::RPAREN, ")".to_string()),
            Token::new(TokenKind::EOF, "".to_string()),
        ];
        let input = "+;{}()".to_string();

        let mut l = Lexer::new(input);
        for t in expected {
            assert_eq!(l.next_token(), t);
        }
    }
}
