pub mod interpreter;
pub mod lexer;
pub mod parser;

use std::fs;
use std::io::Read;
use std::path::Path;

pub struct Script {
    source: String,
}

impl Script {
    pub fn new(file: &str) -> Result<Self, std::io::Error> {
        let mut file = fs::File::open(Path::new(file))?;
        let mut source = String::new();
        file.read_to_string(&mut source)?;
        Ok(Script { source })
    }

    pub fn from(source: &str) -> Self {
        Script {
            source: source.to_string(),
        }
    }

    pub fn run(&self) -> Result<(), interpreter::InterpreterError> {
        let tokens = lexer::tokenize(&self.source)?;
        let ast = parser::parse(tokens)?;
        interpreter::Interpreter::new().execute(&ast)
    }
}
