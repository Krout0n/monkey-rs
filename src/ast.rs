use token::Token;

#[derive(Debug, PartialEq)]
pub enum ASTKind {
    Int(i32),
    Ident(String),
    Add(Box<AST>, Box<AST>),
    Multi(Box<AST>, Box<AST>),
    Let {
        name: Box<ASTKind>,
        value: Box<AST>,
    },
    Minus(Box<AST>, Box<AST>),
    Return(Box<AST>),
    Compound(Vec<AST>),
    If {
        cond: Box<AST>,
        stmt: Box<AST>,
        else_stmt: Option<Box<AST>>,
    },
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

impl AST {
    fn int(i: i32) -> AST {
        AST {
            kind: ASTKind::Int(i),
        }
    }

    fn add(left: AST, right: AST) -> AST {
        AST {
            kind: ASTKind::Add(Box::new(left), Box::new(right)),
        }
    }

    fn return_stmt(expr: AST) -> AST {
        AST {
            kind: ASTKind::Return(Box::new(expr)),
        }
    }

    fn compound_statement(stmts: Vec<AST>) -> AST {
        AST {
            kind: ASTKind::Compound(stmts),
        }
    }

    fn if_stmt(cond: AST, stmt: AST, else_stmt: Option<AST>) -> AST {
        if let Some(e) = else_stmt {
            AST {
                kind: ASTKind::If {
                    cond: Box::new(cond),
                    stmt: Box::new(stmt),
                    else_stmt: Some(Box::new(e)),
                },
            }
        } else {
            AST {
                kind: ASTKind::If {
                    cond: Box::new(cond),
                    stmt: Box::new(stmt),
                    else_stmt: None,
                },
            }
        }
    }
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

    fn compound_statement(&mut self) -> AST {
        self.get();
        let mut stmts = vec![];
        while self.peek() != Some(Token::RBrace) {
            stmts.push(self.statement());
        }
        self.get();
        AST {
            kind: ASTKind::Compound(stmts),
        }
    }

    fn if_stmt(&mut self) -> AST {
        self.get();
        let cond = self.expression();
        let stmt = self.statement();
        let else_stmt = match self.peek() {
            Some(Token::Else) => {
                self.get();
                Some(self.statement())
            },
            _ => None
        };
        AST::if_stmt(cond, stmt, else_stmt)
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
        let t = self.get();
        match t {
            Some(Token::Int(_)) | Some(Token::Ident(_)) => AST {
                kind: Parser::token_to_ast(t.unwrap()),
            },
            Some(_) => panic!(
                "unexpected token! {:?}, btw next one is {:?}",
                t.unwrap(),
                self.tokens.get(self.index + 1)
            ),
            None => panic!("parse error: try to parse primary but got None."),
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

    fn statement(&mut self) -> AST {
        match self.peek() {
            Some(Token::Let) => self.let_stmt(),
            Some(Token::Return) => self.return_stmt(),
            Some(Token::LBrace) => self.compound_statement(),
            Some(Token::If) => self.if_stmt(),
            Some(_) => self.expression_statement(),
            None => panic!("parse error: try to parse statement but got None"),
        }
    }

    pub fn parse(&mut self) {
        while self.peek() != Some(Token::EOF) {
            let node = self.statement();
            self.result.push(node);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ASTKind, Parser, Token, AST};
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
        let p = Parser::new(&tokens);
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

    #[test]
    fn parse_some_statements() {
        let t = vec![
            Token::Int(1),
            Token::Plus,
            Token::Int(2),
            Token::Semicolon,
            Token::Int(3),
            Token::Star,
            Token::Int(4),
            Token::Semicolon,
            Token::EOF,
        ];
        let mut p = Parser::new(&t);
        p.parse();
        assert_eq!(
            p.result,
            vec![
                AST {
                    kind: ASTKind::Add(
                        Box::new(AST {
                            kind: ASTKind::Int(1)
                        }),
                        Box::new(AST {
                            kind: ASTKind::Int(2)
                        })
                    )
                },
                AST {
                    kind: ASTKind::Multi(
                        Box::new(AST {
                            kind: ASTKind::Int(3)
                        }),
                        Box::new(AST {
                            kind: ASTKind::Int(4)
                        })
                    )
                }
            ]
        );
    }

    #[test]
    fn parse_compound() {
        let t = vec![
            Token::LBrace,
            Token::Int(1),
            Token::Plus,
            Token::Int(2),
            Token::Semicolon,
            Token::Int(3),
            Token::Star,
            Token::Int(4),
            Token::Semicolon,
            Token::RBrace,
            Token::EOF,
        ];
        let mut p = Parser::new(&t);
        assert_eq!(
            p.compound_statement(),
            AST {
                kind: ASTKind::Compound(vec![
                    AST {
                        kind: ASTKind::Add(
                            Box::new(AST {
                                kind: ASTKind::Int(1)
                            }),
                            Box::new(AST {
                                kind: ASTKind::Int(2)
                            })
                        )
                    },
                    AST {
                        kind: ASTKind::Multi(
                            Box::new(AST {
                                kind: ASTKind::Int(3)
                            }),
                            Box::new(AST {
                                kind: ASTKind::Int(4)
                            })
                        )
                    }
                ])
            }
        );
    }

    #[test]
    fn parse_if_stmt() {
        let t = vec![
            Token::If,
            Token::Int(1),
            Token::Return,
            Token::Int(10),
            Token::Semicolon,
        ];
        let mut p = Parser::new(&t);
        assert_eq!(
            p.if_stmt(),
            AST::if_stmt(AST::int(1), AST::return_stmt(AST::int(10)), None)
        );

        // if 1 {
        //     1;
        //     2;
        // }
        let t = vec![
            Token::If,
            Token::Int(1),
            Token::LBrace,
            Token::Int(1),
            Token::Semicolon,
            Token::Int(2),
            Token::Semicolon,
            Token::RBrace,
        ];
        let mut p = Parser::new(&t);
        assert_eq!(
            p.if_stmt(),
            AST::if_stmt(
                AST::int(1),
                AST::compound_statement(vec![AST::int(1), AST::int(2)]),
                None
            )
        );

        let t = vec![
            Token::If,
            Token::Int(1),
            Token::Return,
            Token::Int(10),
            Token::Semicolon,
            Token::Else,
            Token::Return,
            Token::Int(20),
            Token::Semicolon,
            Token::RBrace,
        ];
        let mut p = Parser::new(&t);
        assert_eq!(
            p.if_stmt(),
            AST::if_stmt(
                AST::int(1),
                AST::return_stmt(AST::int(10)),
                Some(AST::return_stmt(AST::int(20)))
            )
        );
    }
}
