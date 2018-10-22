use token::Token;

#[derive(Debug, PartialEq)]
pub enum ASTKind {
    Int(i32),
    Ident(String),
    Bool(bool),
    Add(Box<AST>, Box<AST>),
    Multi(Box<AST>, Box<AST>),
    Let {
        name: String,
        expr: Box<AST>,
    },
    Minus(Box<AST>, Box<AST>),
    Return(Box<AST>),
    Compound(Vec<AST>),
    If {
        cond: Box<AST>,
        stmt: Box<AST>,
        else_stmt: Option<Box<AST>>,
    },
    FnCall {
        name: String,
        args: Vec<AST>,
    },
    FnDef {
        args: Vec<String>,
        stmts: Vec<AST>,
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
    pub fn int(i: i32) -> AST {
        AST {
            kind: ASTKind::Int(i),
        }
    }

    pub fn ident(s: String) -> AST {
        AST {
            kind: ASTKind::Ident(s),
        }
    }

    pub fn bool(b: bool) -> AST {
        AST {
            kind: ASTKind::Bool(b),
        }
    }

    pub fn add(left: AST, right: AST) -> AST {
        AST {
            kind: ASTKind::Add(Box::new(left), Box::new(right)),
        }
    }

    pub fn multi(left: AST, right: AST) -> AST {
        AST {
            kind: ASTKind::Multi(Box::new(left), Box::new(right)),
        }
    }

    pub fn let_stmt(name: String, expr: AST) -> AST {
        AST {
            kind: ASTKind::Let {
                name,
                expr: Box::new(expr),
            },
        }
    }

    pub fn return_stmt(expr: AST) -> AST {
        AST {
            kind: ASTKind::Return(Box::new(expr)),
        }
    }

    pub fn compound_statement(stmts: Vec<AST>) -> AST {
        AST {
            kind: ASTKind::Compound(stmts),
        }
    }

    pub fn if_stmt(cond: AST, stmt: AST, else_stmt: Option<AST>) -> AST {
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

    pub fn fn_call(name: String, args: Vec<AST>) -> AST {
        AST {
            kind: ASTKind::FnCall { name, args },
        }
    }

    pub fn fn_def(args: Vec<String>, stmts: Vec<AST>) -> AST {
        AST {
            kind: ASTKind::FnDef { args, stmts },
        }
    }
}

impl<'a> Parser<'a> {
    fn return_stmt(&mut self) -> AST {
        assert_eq!(self.get(), Some(Token::Return));
        AST::return_stmt(self.expression_statement())
    }

    fn let_stmt(&mut self) -> AST {
        assert_eq!(self.get(), Some(Token::Let));
        let name = match self.peek() {
            Some(Token::Ident(s)) => {
                self.get();
                s
            }
            _ => panic!("parse error: expect ident but got {:?}", self.peek()),
        };
        assert_eq!(self.get(), Some(Token::Assign));
        let expr = self.expression();
        assert_eq!(self.get(), Some(Token::Semicolon));
        AST::let_stmt(name, expr)
    }

    fn compound_statement(&mut self) -> AST {
        self.get();
        let mut stmts = vec![];
        while self.peek() != Some(Token::RBrace) {
            stmts.push(self.statement());
        }
        self.get();
        AST::compound_statement(stmts)
    }

    fn if_stmt(&mut self) -> AST {
        self.get();
        let cond = self.expression();
        let stmt = self.statement();
        let else_stmt = match self.peek() {
            Some(Token::Else) => {
                self.get();
                Some(self.statement())
            }
            _ => None,
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
            Some(Token::Int(i)) => AST::int(i),
            Some(Token::Ident(s)) => {
                if let Some(Token::LParen) = self.peek() {
                    self.get();
                    let mut args = vec![];
                    loop {
                        args.push(self.expression());
                        match self.peek() {
                            Some(Token::RParen) => break,
                            Some(Token::Comma) => self.get(),
                            _ => panic!("parse error: unexpected token {:?}"),
                        };
                    }
                    self.get();
                    AST::fn_call(s, args)
                } else {
                    AST::ident(s)
                }
            }
            Some(Token::True) => AST::bool(true),
            Some(Token::False) => AST::bool(false),
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
            left = AST::add(left, right);
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
            left = AST::multi(left, right);
        }
        left
    }

    fn fn_def(&mut self) -> AST {
        self.get();
        let mut args = vec![];
        assert_eq!(self.get(), Some(Token::LParen));
        loop {
            let t = self.get();
            if let Some(s) = t {
                if let Token::Ident(s) = s {
                    args.push(s);
                } else {
                    panic!(
                        "parse error: illegal token for args of function defition {:?}",
                        s
                    )
                }
            } else {
                panic!(
                    "parse error: illegal token for args of function defition {:?}",
                    t
                )
            }
            match self.peek() {
                Some(Token::RParen) => break,
                Some(Token::Comma) => self.get(),
                _ => panic!("parse error: unexpected token {:?}"),
            };
        }
        self.get();

        if let ASTKind::Compound(stmts) = self.compound_statement().kind {
            AST::fn_def(args, stmts)
        } else {
            panic!("")
        }
    }

    fn expression(&mut self) -> AST {
        match self.peek() {
            Some(Token::Function) => self.fn_def(),
            Some(_) => self.additive(),
            _ => unimplemented!(),
        }
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
    use super::{Parser, Token, AST};
    #[test]
    fn parse_one_plus_two() {
        let tokens: [Token; 4] = [Token::Int(1), Token::Plus, Token::Int(2), Token::EOF];
        let mut p = Parser::new(&tokens);
        assert_eq!(p.additive(), AST::add(AST::int(1), AST::int(2)))
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
            AST::add(AST::add(AST::int(1), AST::int(2)), AST::int(3))
        )
    }

    #[test]
    fn parse_one_times_two() {
        let t = vec![Token::Int(1), Token::Star, Token::Int(2), Token::EOF];
        let mut p = Parser::new(&t);
        assert_eq!(p.multiplicative(), AST::multi(AST::int(1), AST::int(2)))
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
            AST::add(AST::int(1), AST::multi(AST::int(2), AST::int(3)))
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
        ];
        let mut p = Parser::new(&t);
        assert_eq!(
            p.additive(),
            AST::add(
                AST::add(AST::int(1), AST::multi(AST::int(2), AST::int(3))),
                AST::int(4)
            )
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
    fn test_let_stmt() {
        let t = vec![
            Token::Let,
            Token::Ident("x".to_string()),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
        ];
        let mut p = Parser::new(&t);
        assert_eq!(p.let_stmt(), AST::let_stmt("x".to_string(), AST::int(10)));

        let t = vec![
            Token::Let,
            Token::Ident("x".to_string()),
            Token::Assign,
            Token::Int(10),
            Token::Plus,
            Token::Int(20),
            Token::Semicolon,
        ];
        let mut p = Parser::new(&t);
        assert_eq!(
            p.let_stmt(),
            AST::let_stmt("x".to_string(), AST::add(AST::int(10), AST::int(20)))
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
            AST::return_stmt(AST::add(AST::ident("x".to_string()), AST::int(1)))
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
                AST::add(AST::int(1), AST::int(2)),
                AST::multi(AST::int(3), AST::int(4))
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
            AST::compound_statement(vec![
                AST::add(AST::int(1), AST::int(2)),
                AST::multi(AST::int(3), AST::int(4))
            ])
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

    #[test]
    fn parse_fncall() {
        let t = vec![
            Token::Ident("x".to_string()),
            Token::LParen,
            Token::Int(1),
            Token::RParen,
        ];
        let mut p = Parser::new(&t);
        assert_eq!(
            p.primary(),
            AST::fn_call("x".to_string(), vec![AST::int(1)])
        );

        let t = vec![
            Token::Ident("x".to_string()),
            Token::LParen,
            Token::Int(1),
            Token::Plus,
            Token::Int(2),
            Token::Comma,
            Token::Int(3),
            Token::RParen,
        ];
        let mut p = Parser::new(&t);
        assert_eq!(
            p.primary(),
            AST::fn_call(
                "x".to_string(),
                vec![AST::add(AST::int(1), AST::int(2)), AST::int(3)]
            )
        );
    }

    #[test]
    fn parse_fndef() {
        let t = vec![
            Token::Function,
            Token::LParen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Ident("y".to_string()),
            Token::Semicolon,
            Token::RBrace,
        ];
        let mut p = Parser::new(&t);
        assert_eq!(
            p.fn_def(),
            AST::fn_def(
                vec!["x".to_string(), "y".to_string()],
                vec![AST::return_stmt(AST::add(
                    AST::ident("x".to_string()),
                    AST::ident("y".to_string())
                ))]
            )
        )
    }
}
