use lexer::*;
use token::*;

#[allow(unused_imports)]
use std::io::{self, stdin, Read, Write};

pub fn start() {
    println!("Yo this is a Monkey programming language REPL!");
    println!("Feel free to type some statement!");
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
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
