use std::io::stdin;

use crate::repl::start;

mod lexer;
mod repl;
mod token;

fn main() {
    println!("Hello, this is Monkey programming language");
    println!("Feel free to type in commands");
    start(stdin());
}
