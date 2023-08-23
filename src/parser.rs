use crate::{
    ast::{Identifier, LetStatement, ReturnStatement, Statement},
    lexer::Lexer,
    token::{Token, TokenType},
};

#[derive(Debug)]
struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
            peek_token,
            errors: vec![],
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(&mut self) -> Vec<Statement> {
        let mut program = vec![];

        while self.current_token.token_type != TokenType::Eof {
            let statement = self.parse_statement();
            if statement.is_some() {
                program.push(statement.unwrap())
            }
            self.next_token();
        }

        return program;
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let current_token = self.current_token.clone();

        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let statement_name = Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        while !self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        let stmt = LetStatement {
            token: current_token,
            name: statement_name,
            value: self.current_token.literal.clone(),
        };

        Some(Statement::Let(stmt))
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let current_token = self.current_token.clone();

        self.next_token();

        while !self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        let stmt = ReturnStatement {
            token: current_token,
            value: self.current_token.literal.clone(),
        };

        return Some(Statement::Return(stmt));
    }

    fn cur_token_is(&self, token: TokenType) -> bool {
        self.current_token.token_type == token
    }

    fn peek_token_is(&self, token: TokenType) -> bool {
        self.peek_token.token_type == token
    }

    fn expect_peek(&mut self, token: TokenType) -> bool {
        if self.peek_token_is(token) {
            self.next_token();
            true
        } else {
            self.peak_error(token);
            false
        }
    }

    fn errors(&mut self) -> Vec<String> {
        return self.errors.clone();
    }

    fn peak_error(&mut self, token: TokenType) {
        self.errors.push(format!(
            "expected next token to be {}, got {} instead",
            token, self.peek_token.token_type,
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_let_statements() {
        let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "#;

        let lexer = Lexer::new(input.to_owned());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&mut parser);
        if program.len() != 3 {
            panic!("Parsed too few statements");
        }

        let tests = vec![("x", "5"), ("y", "10"), ("foobar", "838383")];

        let mut i = 0;
        for (expected_ident, expected_value) in &tests {
            let stmt = &program[i]; // get next statement

            if !test_let_statement(stmt, expected_ident, expected_value) {
                panic!("test_let_statement failed");
            }
            i += 1;
        }
    }

    fn test_let_statement(stmt: &Statement, expected_ident: &str, expected_value: &str) -> bool {
        if let Statement::Let(let_stmt) = stmt {
            return let_stmt.name.value == expected_ident && let_stmt.value == expected_value;
        }
        false
    }

    fn check_parser_errors(parser: &mut Parser) {
        let errors = parser.errors();

        if errors.len() == 0 {
            return;
        }

        for msg in errors {
            panic!("Parser error: {}", msg);
        }
    }

    #[test]
    fn test_return_statements() {
        let input = r#"
       return 5;
       return 10;
       return add(15);
        "#;

        let lexer = Lexer::new(input.to_owned());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_errors(&mut parser);

        if program.len() != 3 {
            panic!("Parsed too few statements");
        }

        for statement in &program {
            match statement {
                Statement::Let(_) => {}
                Statement::Return(stmt) => {
                    println!("{:#?}", stmt);
                }
            }
        }
    }
}
