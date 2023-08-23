use std::str::from_utf8;

use crate::token::{lookup_ident, Token, TokenType};

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        l.read_char();
        l
    }
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
    pub fn next_token(&mut self) -> Token {
        let mut token = Token {
            token_type: TokenType::Illegal,
            literal: "".to_string(),
        };

        self.skip_whitespace();

        if is_letter(self.ch) {
            token.literal = self.read_identifier();
            token.token_type = lookup_ident(&token.literal);
            return token;
        } else if is_digit(self.ch) {
            token.token_type = TokenType::Int;
            token.literal = self.read_number();
            return token;
        } else {
            token = generate_new_token(TokenType::Illegal, self.ch);
        }

        match self.ch {
            b'+' => token.token_type = TokenType::Plus,
            b'-' => token.token_type = TokenType::Minus,
            b'!' => {
                if self.peek_char() == b'=' {
                    let ch = self.ch;
                    self.read_char();
                    token.literal = format!("{}{}", ch_to_utf8(ch), ch_to_utf8(self.ch));
                    token.token_type = TokenType::NotEq
                } else {
                    token.token_type = TokenType::Bang
                }
            }
            b'*' => token.token_type = TokenType::Asterisk,
            b'/' => token.token_type = TokenType::Slash,

            b'=' => {
                if self.peek_char() == b'=' {
                    let ch = self.ch;
                    self.read_char();
                    token.literal = format!("{}{}", ch_to_utf8(ch), ch_to_utf8(self.ch));
                    token.token_type = TokenType::Eq
                } else {
                    token.token_type = TokenType::Assign
                }
            }
            b';' => token.token_type = TokenType::Semicolon,
            b',' => token.token_type = TokenType::Comma,

            b'<' => token.token_type = TokenType::Lt,
            b'>' => token.token_type = TokenType::Gt,

            b'(' => token.token_type = TokenType::Lparen,
            b')' => token.token_type = TokenType::Rparen,
            b'{' => token.token_type = TokenType::Lbrace,
            b'}' => token.token_type = TokenType::Rbrace,

            0 => token.token_type = TokenType::Eof,

            _ => token.token_type = TokenType::Illegal,
        }
        self.read_char();

        token
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;

        while is_letter(self.ch) {
            self.read_char();
        }

        self.input[position..self.position].to_string()
    }
    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn peek_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        } else {
            return self.input.as_bytes()[self.read_position];
        }
    }
}

pub fn is_letter(ch: u8) -> bool {
    ch >= b'a' && ch <= b'z' || ch >= b'A' && ch <= b'Z' || ch == b'_'
}

pub fn is_digit(ch: u8) -> bool {
    ch >= b'0' && ch <= b'9'
}

pub fn generate_new_token(token_type: TokenType, ch: u8) -> Token {
    Token {
        token_type,
        literal: ch_to_utf8(ch),
    }
}

pub fn ch_to_utf8(ch: u8) -> String {
    from_utf8(&[ch]).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = r#"
        let five = 5;
        let ten = 10;

        let add = fn(x,y) {
            x+y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5; 

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;
        "#;

        let mut lexer = Lexer::new(String::from(input));
        let expected_tokens = vec![
            (TokenType::Let, "let"),
            (TokenType::Ident, "five"),
            (TokenType::Assign, "="),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "ten"),
            (TokenType::Assign, "="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "add"),
            (TokenType::Assign, "="),
            (TokenType::Function, "fn"),
            (TokenType::Lparen, "("),
            (TokenType::Ident, "x"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "y"),
            (TokenType::Rparen, ")"),
            (TokenType::Lbrace, "{"),
            (TokenType::Ident, "x"),
            (TokenType::Plus, "+"),
            (TokenType::Ident, "y"),
            (TokenType::Semicolon, ";"),
            (TokenType::Rbrace, "}"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "result"),
            (TokenType::Assign, "="),
            (TokenType::Ident, "add"),
            (TokenType::Lparen, "("),
            (TokenType::Ident, "five"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "ten"),
            (TokenType::Rparen, ")"),
            (TokenType::Semicolon, ";"),
            // !-/*5;
            (TokenType::Bang, "!"),
            (TokenType::Minus, "-"),
            (TokenType::Slash, "/"),
            (TokenType::Asterisk, "*"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            // 5 < 10 > 5;
            (TokenType::Int, "5"),
            (TokenType::Lt, "<"),
            (TokenType::Int, "10"),
            (TokenType::Gt, ">"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            // New tokens for if/else statement:
            (TokenType::If, "if"),
            (TokenType::Lparen, "("),
            (TokenType::Int, "5"),
            (TokenType::Lt, "<"),
            (TokenType::Int, "10"),
            (TokenType::Rparen, ")"),
            (TokenType::Lbrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::True, "true"),
            (TokenType::Semicolon, ";"),
            (TokenType::Rbrace, "}"),
            (TokenType::Else, "else"),
            (TokenType::Lbrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::False, "false"),
            (TokenType::Semicolon, ";"),
            (TokenType::Rbrace, "}"),
            // ...
            // New tokens for equality operators:
            (TokenType::Int, "10"),
            (TokenType::Eq, "=="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int, "10"),
            (TokenType::NotEq, "!="),
            (TokenType::Int, "9"),
            (TokenType::Semicolon, ";"),
            (TokenType::Eof, "\0"),
        ];

        // Then in the test loop:

        for (expected_type, expected_literal) in &expected_tokens {
            let actual_token = lexer.next_token();
            println!("{:#?}", actual_token);
            assert_eq!(actual_token.token_type, *expected_type);
            assert_eq!(actual_token.literal, *expected_literal);
        }
    }
}
