use crate::parser::Statement;
use std::collections::HashMap;
use std::fs;

pub struct Interpreter {
    variables: HashMap<String, String>,
}

#[derive(Debug)]
pub enum InterpreterError {
    IoError(std::io::Error),
    RuntimeError(String),
}

impl From<std::io::Error> for InterpreterError {
    fn from(error: std::io::Error) -> Self {
        InterpreterError::IoError(error)
    }
}

impl From<String> for InterpreterError {
    fn from(error: String) -> Self {
        InterpreterError::RuntimeError(error)
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    pub fn execute(&mut self, ast: &[Statement]) -> Result<(), InterpreterError> {
        for stmt in ast {
            match stmt {
                Statement::Read(src, dest) => {
                    let content = fs::read_to_string(src)?;
                    self.variables.insert(dest.clone(), content);
                }
                Statement::Write(path, content) => {
                    let file_content = self.get_var_or_literal(content);
                    fs::write(path, file_content)?;
                }
                Statement::Print(content) => {
                    let output = self.get_var_or_literal(content);
                    println!("{}", output);
                }
                Statement::Append(src1, src2, dest) => {
                    let str1 = self.get_var_or_literal(src1);
                    let str2 = self.get_var_or_literal(src2);
                    let result = str1 + &str2;
                    self.variables.insert(dest.clone(), result);
                }
            }
        }
        Ok(())
    }

    fn get_var_or_literal(&self, key: &str) -> String {
        if key.starts_with('"') && key.ends_with('"') {
            key[1..key.len() - 1].to_string()
        } else {
            self.variables
                .get(key)
                .cloned()
                .unwrap_or_else(|| key.to_string())
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_interpreter_basic() {
        let script = r#"READ input.txt -> content
PRINT content
APPEND content " (appended)" -> appended_content
PRINT appended_content
WRITE output.txt appended_content
"#;

        // Ensure input.txt exists and has content
        fs::write("input.txt", "Hello, World!").unwrap();

        let tokens = crate::lexer::tokenize(script).unwrap();
        let ast = crate::parser::parse(tokens).unwrap();
        let mut interpreter = Interpreter::new();
        interpreter.execute(&ast).unwrap();

        let output = fs::read_to_string("output.txt").unwrap();
        assert_eq!(output, "Hello, World! (appended)");
    }
}
