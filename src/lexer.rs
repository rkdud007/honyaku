use crate::token::{lookup_ident, Token, TokenType};

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

trait LexerTrait {
    fn new(input: String) -> Lexer;
    fn read_char(&mut self);
    fn next_token(&mut self) -> Token;
    fn read_identifier(&mut self) -> String;
    fn skip_whitespace(&mut self);
    fn read_number(&mut self) -> String;
}

impl LexerTrait for Lexer {
    fn new(input: String) -> Lexer {
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
    fn next_token(&mut self) -> Token {
        let mut token = Token {
            token_type: TokenType::Illegal,
            literal: "".to_string(),
        };

        self.skip_whitespace();

        if is_letter(self.ch) {
            token.literal = self.read_identifier();
            println!("token literal : {}", token.literal);
            token.token_type = lookup_ident(&token.literal);
            println!("token type : {:#?}", token.token_type);
            return token;
        } else if is_digit(self.ch) {
            token.token_type = TokenType::Int;
            token.literal = self.read_number();
            return token;
        } else {
            token = generate_new_token(TokenType::Illegal, self.ch);
        }

        match self.ch {
            b';' => token.token_type = TokenType::Semicolon,
            b',' => token.token_type = TokenType::Comma,
            b'+' => token.token_type = TokenType::Plus,
            b'=' => token.token_type = TokenType::Assign,
            b'a'..=b'z' | b'A'..=b'Z' => token.token_type = TokenType::Ident,
            b'0'..=b'9' => token.token_type = TokenType::Int,
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
}

pub fn is_letter(ch: u8) -> bool {
    ch >= b'a' && ch <= b'z' || ch >= b'A' && ch <= b'Z' || ch == b'_'
}

pub fn is_digit(ch: u8) -> bool {
    ch >= b'0' && ch <= b'9'
}

pub fn generate_new_token(token_type: TokenType, ch: u8) -> Token {
    let literal = std::str::from_utf8(&[ch]).unwrap().to_string();

    Token {
        token_type,
        literal,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = " let five = 5;";

        let expected_tokens = vec![
            Token {
                token_type: TokenType::Let,
                literal: String::from("let"),
            },
            Token {
                token_type: TokenType::Ident,
                literal: String::from("five"),
            },
            Token {
                token_type: TokenType::Assign,
                literal: String::from("="),
            },
            Token {
                token_type: TokenType::Int,
                literal: String::from("5"),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: String::from(";"),
            },
        ];

        let mut lexer = Lexer::new(String::from(input));

        for expected_token in expected_tokens {
            let actual_token = lexer.next_token();

            assert_eq!(actual_token, expected_token);
        }
    }
}
