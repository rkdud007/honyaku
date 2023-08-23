use std::io::stdin;

use crate::repl::start;

mod ast;
mod lexer;
mod parser;
mod repl;
mod token;

fn main() {
    println!("Hello, this is Monkey programming language");
    println!("Feel free to type in commands");
    start(stdin());
}
