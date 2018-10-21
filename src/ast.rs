use lexer::*;
use token::Token;

#[derive(Debug, PartialEq)]
pub enum ASTKind {
    Int(i32),
    Ident(String),
    Add(Box<AST>, Box<AST>),
    Multi(Box<AST>, Box<AST>),
    Let { name: Box<ASTKind>, value: Box<AST> },
    Minus(Box<AST>, Box<AST>),
    Return(Box<AST>),
}

#[derive(Debug, PartialEq)]
pub struct AST {
    pub kind: ASTKind,
}

pub struct Parser<'a> {
    tokens: &'a [Token],
    index: usize,
    pub result: Vec<AST>,
}

impl<'a> Parser<'a> {
    fn token_to_ast(t: Token) -> ASTKind {
        match t {
            Token::Int(i) => ASTKind::Int(i),
            Token::Ident(s) => ASTKind::Ident(s),
            _ => unimplemented!(),
        }
    }

    fn return_stmt(&mut self) -> AST {
        assert_eq!(self.get(), Some(Token::Return));
        AST {
            kind: ASTKind::Return(Box::new(self.expression_statement())),
        }
    }

    fn let_stmt(&mut self) -> AST {
        assert_eq!(self.get(), Some(Token::Let));
        let name = Box::new(Parser::token_to_ast(self.get().unwrap()));
        assert_eq!(self.get(), Some(Token::Assign));
        let value = Box::new(self.expression());
        assert_eq!(self.get(), Some(Token::Semicolon));
        AST {
            kind: ASTKind::Let { name, value },
        }
    }

    fn peek(&self) -> Option<Token> {
        if let Some(t) = self.tokens.get(self.index) {
            Some(t.clone())
        } else {
            None
        }
    }

    fn get(&mut self) -> Option<Token> {
        if let Some(t) = self.tokens.get(self.index) {
            self.index += 1;
            Some(t.clone())
        } else {
            None
        }
    }

    pub fn new(tokens: &'a [Token]) -> Self {
        Parser {
            tokens,
            index: 0,
            result: vec![],
        }
    }

    fn primary(&mut self) -> AST {
        if let Some(token) = self.peek() {
            self.get();
            let kind = Parser::token_to_ast(token);
            AST { kind }
        } else {
            panic!("error!");
            AST {
                kind: ASTKind::Int(-114514),
            }
        }
    }

    fn additive(&mut self) -> AST {
        let mut left = self.multiplicative();
        loop {
            let peeked = self.peek();
            if peeked != Some(Token::Plus) {
                break;
            }
            self.get();
            let right = self.multiplicative();
            left = AST {
                kind: ASTKind::Add(Box::new(left), Box::new(right)),
            }
        }
        left
    }

    fn multiplicative(&mut self) -> AST {
        let mut left = self.primary();
        loop {
            let peeked = self.peek();
            if peeked != Some(Token::Star) {
                break;
            }
            self.get();
            let right = self.primary();
            left = AST {
                kind: ASTKind::Multi(Box::new(left), Box::new(right)),
            }
        }
        left
    }

    fn expression(&mut self) -> AST {
        self.additive()
    }

    fn expression_statement(&mut self) -> AST {
        let ast = self.expression();
        assert_eq!(self.get(), Some(Token::Semicolon));
        ast
    }

    pub fn parse(&mut self) {
        let node = self.expression();
        self.result.push(node);
    }
}

mod tests {
    use super::*;
    #[test]
    fn parse_one_plus_two() {
        let tokens: [Token; 4] = [Token::Int(1), Token::Plus, Token::Int(2), Token::EOF];
        let mut p = Parser::new(&tokens);
        assert_eq!(
            p.additive(),
            AST {
                kind: ASTKind::Add(
                    Box::new(AST {
                        kind: ASTKind::Int(1)
                    }),
                    Box::new(AST {
                        kind: ASTKind::Int(2)
                    })
                )
            }
        )
    }

    #[test]
    fn parse_one_plus_two_plus_three() {
        let t = vec![
            Token::Int(1),
            Token::Plus,
            Token::Int(2),
            Token::Plus,
            Token::Int(3),
            Token::EOF,
        ];
        let mut p = Parser::new(&t);
        assert_eq!(
            p.additive(),
            AST {
                kind: ASTKind::Add(
                    Box::new(AST {
                        kind: ASTKind::Add(
                            Box::new(AST {
                                kind: ASTKind::Int(1)
                            }),
                            Box::new(AST {
                                kind: ASTKind::Int(2)
                            })
                        )
                    }),
                    Box::new(AST {
                        kind: ASTKind::Int(3)
                    })
                )
            }
        )
    }

    #[test]
    fn parse_one_times_two() {
        let t = vec![Token::Int(1), Token::Star, Token::Int(2), Token::EOF];
        let mut p = Parser::new(&t);
        assert_eq!(
            p.multiplicative(),
            AST {
                kind: ASTKind::Multi(
                    Box::new(AST {
                        kind: ASTKind::Int(1)
                    }),
                    Box::new(AST {
                        kind: ASTKind::Int(2)
                    })
                )
            }
        )
    }

    #[test]
    fn parse_one_plus_two_times_three() {
        let t = vec![
            Token::Int(1),
            Token::Plus,
            Token::Int(2),
            Token::Star,
            Token::Int(3),
            Token::EOF,
        ];
        let mut p = Parser::new(&t);
        assert_eq!(
            p.additive(),
            AST {
                kind: ASTKind::Add(
                    Box::new(AST {
                        kind: ASTKind::Int(1)
                    }),
                    Box::new(AST {
                        kind: ASTKind::Multi(
                            Box::new(AST {
                                kind: ASTKind::Int(2)
                            }),
                            Box::new(AST {
                                kind: ASTKind::Int(3)
                            })
                        )
                    }),
                )
            }
        )
    }

    #[test]
    fn parse_one_plus_two_times_three_plus_four() {
        let t = vec![
            Token::Int(1),
            Token::Plus,
            Token::Int(2),
            Token::Star,
            Token::Int(3),
            Token::Plus,
            Token::Int(4),
            Token::EOF,
        ];
        let mut p = Parser::new(&t);
        assert_eq!(
            p.additive(),
            AST {
                kind: ASTKind::Add(
                    Box::new(AST {
                        kind: ASTKind::Add(
                            Box::new(AST {
                                kind: ASTKind::Int(1)
                            }),
                            Box::new(AST {
                                kind: ASTKind::Multi(
                                    Box::new(AST {
                                        kind: ASTKind::Int(2)
                                    }),
                                    Box::new(AST {
                                        kind: ASTKind::Int(3)
                                    })
                                )
                            })
                        )
                    }),
                    Box::new(AST {
                        kind: ASTKind::Int(4)
                    }),
                )
            }
        )
    }

    #[test]
    fn test_peek() {
        let tokens: [Token; 4] = [Token::Int(1), Token::Plus, Token::Int(2), Token::EOF];
        let mut p = Parser::new(&tokens);
        assert_eq!(p.peek(), Some(Token::Int(1)));
        assert_eq!(p.index, 0);
    }

    #[test]
    fn test_get() {
        let tokens: [Token; 4] = [Token::Int(1), Token::Plus, Token::Int(2), Token::EOF];
        let mut p = Parser::new(&tokens);
        assert_eq!(p.get(), Some(Token::Int(1)));
        assert_eq!(p.get(), Some(Token::Plus));
        assert_eq!(p.index, 2);
    }

    #[test]
    fn test_letstmt() {
        let t = vec![
            Token::Let,
            Token::Ident("x".to_string()),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::EOF,
        ];
        let mut p = Parser::new(&t);
        assert_eq!(
            p.let_stmt(),
            AST {
                kind: ASTKind::Let {
                    name: Box::new(ASTKind::Ident("x".to_string())),
                    value: Box::new(AST {
                        kind: ASTKind::Int(10)
                    })
                }
            }
        );

        let t = vec![
            Token::Let,
            Token::Ident("x".to_string()),
            Token::Assign,
            Token::Int(10),
            Token::Plus,
            Token::Int(20),
            Token::Semicolon,
            Token::EOF,
        ];

        let mut p = Parser::new(&t);
        assert_eq!(
            p.let_stmt(),
            AST {
                kind: ASTKind::Let {
                    name: Box::new(ASTKind::Ident("x".to_string())),
                    value: Box::new(AST {
                        kind: ASTKind::Add(
                            Box::new(AST {
                                kind: ASTKind::Int(10)
                            }),
                            Box::new(AST {
                                kind: ASTKind::Int(20)
                            })
                        )
                    })
                }
            }
        );
    }

    #[test]
    fn parse_return_stmt() {
        let t = vec![
            Token::Return,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Int(1),
            Token::Semicolon,
        ];
        let mut p = Parser::new(&t);
        assert_eq!(
            p.return_stmt(),
            AST {
                kind: ASTKind::Return(Box::new(AST {
                    kind: ASTKind::Add(
                        Box::new(AST {
                            kind: ASTKind::Ident("x".to_string())
                        }),
                        Box::new(AST {
                            kind: ASTKind::Int(1)
                        })
                    )
                }))
            }
        )
    }
}
