use crate::{lexer::Lexer, token::TokenType};
use std::io::Stdin;

static PROMPT: &str = ">> ";

pub fn start(input: Stdin) {
    input.lines().for_each(|l| match l {
        Ok(line) => {
            let mut lexer = Lexer::new(line);
            loop {
                let token = lexer.next_token();
                println!("{:#?}", token);

                if token.token_type == TokenType::Eof {
                    break;
                }
            }
        }
        Err(error) => println!("error: {error}"),
    });
}
