pub mod token {

    #[derive(Debug, PartialEq)]
    pub enum Token {
        ILLEGAL(char),
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
                Some('=') => Token::ASSIGN,
                Some('+') => Token::PLUS,
                Some(';') => Token::SEMICOLON,
                Some('{') => Token::LBRACE,
                Some('}') => Token::RBRACE,
                Some('(') => Token::LPAREN,
                Some(')') => Token::RPAREN,
                None => Token::EOF,
                Some(c) => Token::ILLEGAL(c)
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
            Token::PLUS,
            Token::SEMICOLON,
            Token::LBRACE,
            Token::RBRACE,
            Token::LPAREN,
            Token::RPAREN,
            Token::EOF,
        ];
        let input = "+;{}()".to_string();

        let mut l = Lexer::new(input);
        for t in expected {
            assert_eq!(l.next_token(), t);
        }
    }
}
