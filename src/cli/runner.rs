use crate::backend::Interpreter;
use crate::core::{InfraError, Result};
use crate::frontend::{Lexer, Parser};
use std::fs;

pub struct Runner {
    interpreter: Interpreter,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
        }
    }

    pub fn run_file(&mut self, filename: &str) -> Result<()> {
        let contents = fs::read_to_string(filename).map_err(|err| InfraError::IoError {
            message: format!("Error reading file '{}': {}", filename, err),
            operation: Some("read file".to_string()),
            path: Some(filename.to_string()),
        })?;

        // Set the current file path for module resolution
        let file_path = std::path::Path::new(filename)
            .canonicalize()
            .unwrap_or_else(|_| std::path::PathBuf::from(filename));
        self.interpreter.set_current_file(file_path);

        self.execute_code(&contents)
    }

    pub fn execute_code(&mut self, code: &str) -> Result<()> {
        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize()?;

        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;

        self.interpreter.execute(&ast)?;
        Ok(())
    }

    pub fn reset_interpreter(&mut self) {
        self.interpreter.reset();
    }

    pub fn get_interpreter(&self) -> &Interpreter {
        &self.interpreter
    }

    pub fn get_interpreter_mut(&mut self) -> &mut Interpreter {
        &mut self.interpreter
    }
}

impl Default for Runner {
    fn default() -> Self {
        Self::new()
    }
}
