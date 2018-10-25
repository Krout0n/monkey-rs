use ast::*;
use eval::Evaluator;
use lexer::*;
use token::*;

#[allow(unused_imports)]
use std::io::{self, stdin, Read, Write};

pub fn start() {
    println!("Yo this is a Monkey programming language REPL!");
    println!("Feel free to type some statement!");
    let ev = Evaluator::new();
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        let mut l = Lexer::new(read_input());
        let mut v = vec![];
        loop {
            let t = l.next_token();
            v.push(t);
            if v.get(v.len() - 1) == Some(&Token::EOF) {
                break;
            }
        }
        let mut p = Parser::new(&v);
        p.parse();
        for s in p.result {
            println!("{:?}", ev.eval(s));
        }
    }
}

fn read_input() -> String {
    let mut s = String::new();
    stdin().read_line(&mut s).expect("failed to read stdin");
    s
}
