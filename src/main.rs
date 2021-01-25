mod backend;
mod lexer;
mod parser;
use backend::*;
use lexer::*;
use parser::*;
use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = env::args().last().unwrap();
    let source = fs::read_to_string(file)?;
    let lex = Lexer::new(source);
    let mut interp = Interpreter::new(lex);
    interp.execute();

    Ok(())
}
