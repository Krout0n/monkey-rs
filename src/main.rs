pub mod token {

    #[derive(Debug, PartialEq)]
    pub enum Token {
        Illegal(char),
        EOF,

        Assign,
        Plus,

        LParen,
        RParen,
        LBrace,
        RBrace,

        Semicolon,
        Comma,
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
                Some('=') => Token::Assign,
                Some('+') => Token::Plus,
                Some(';') => Token::Semicolon,
                Some('{') => Token::LBrace,
                Some('}') => Token::RBrace,
                Some('(') => Token::LParen,
                Some(')') => Token::RParen,
                None => Token::EOF,
                Some(c) => Token::Illegal(c)
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
}
