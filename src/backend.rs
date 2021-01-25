use crate::*;
use std::collections::HashMap;

pub trait Statement {
    fn resolve(&self, _interp: &mut Interpreter) {}
}

impl Statement for Assignment {
    fn resolve(&self, interp: &mut Interpreter) {
        let k = self.var.name.clone();
        let v = self.string.clone();
        interp.variables.insert(k, v);
    }
}

impl Statement for Print {
    fn resolve(&self, interp: &mut Interpreter) {
        let name = self.var.name.clone();
        println!("{}", interp.variables.get(&name).unwrap());
    }
}

pub struct Interpreter {
    lexer: Lexer,
    variables: HashMap<String, String>,
}

impl std::fmt::Display for Interpreter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Interpreter: {:?})", self.variables)
    }
}

impl Interpreter {
    pub fn new(lexer: Lexer) -> Self {
        let variables = HashMap::new();
        Interpreter { lexer, variables }
    }

    pub fn execute(&mut self) {
        let ast = parse(&mut self.lexer);
        for statement in ast {
            statement.resolve(self);
        }
    }
}
