use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
}

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub value: String, // later Expression
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: String, // later Expression
}

#[derive(Debug)]
pub enum Expression {}

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}
