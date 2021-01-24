use crate::backend::Statement;
use crate::lexer::*;

#[derive(Debug)]
pub struct Variable {
    pub line: usize,
    pub name: String,
}

#[derive(Debug)]
pub struct Assignment {
    pub line: usize,
    pub var: Variable,
    pub string: String,
}

#[derive(Debug)]
pub struct Print {
    pub line: usize,
    pub var: Variable,
}

pub fn parse_name(lexer: &mut Lexer) -> String {
    lexer.next_token_is(TokenType::Name).2.unwrap()
}

pub fn parse_string(lexer: &mut Lexer) -> String {
    lexer.next_token_is(TokenType::String).2.unwrap()
}

pub fn parse_ignore(lexer: &mut Lexer) {
    // let mut token = lexer.next_token_is(TokenType::Ignored);
    while lexer.lookahead().0 == TokenType::Ignored {
        lexer.next_token_is(TokenType::Ignored);
    }
}

pub fn parse_variable(lexer: &mut Lexer) -> Variable {
    let line = lexer.next_token_is(TokenType::VarPrefix).1;
    let name = parse_name(lexer);
    parse_ignore(lexer);
    Variable { line, name }
}

pub fn parse_assignment(lexer: &mut Lexer) -> Assignment {
    let var = parse_variable(lexer);
    parse_ignore(lexer);
    lexer.next_token_is(TokenType::Equal);
    parse_ignore(lexer);
    let string = parse_string(lexer);
    parse_ignore(lexer);
    let line = var.line;
    Assignment { line, var, string }
}

pub fn parse_print(lexer: &mut Lexer) -> Print {
    let line = lexer.next_token_is(TokenType::Print).1;
    parse_ignore(lexer);
    lexer.next_token_is(TokenType::LeftBracket);
    parse_ignore(lexer);
    let var = parse_variable(lexer);
    parse_ignore(lexer);
    lexer.next_token_is(TokenType::RightBracket);
    parse_ignore(lexer);
    Print { line, var }
}

pub fn parse_statement(lexer: &mut Lexer) -> Box<dyn Statement> {
    if lexer.lookahead().0 == TokenType::Print {
        return Box::new(parse_print(lexer));
    }
    if lexer.lookahead().0 == TokenType::VarPrefix {
        return Box::new(parse_assignment(lexer));
    }
    panic!("parse_statement: unexpected token");
}

pub fn parse(lexer: &mut Lexer) -> Vec<Box<dyn Statement>> {
    let mut statements = Vec::new();
    while lexer.lookahead().0 != TokenType::Eof {
        statements.push(parse_statement(lexer));
    }
    statements
}
