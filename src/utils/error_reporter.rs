use crate::core::InfraError;
use std::io::{self, Write};

pub struct ErrorReporter {
    had_error: bool,
}

impl ErrorReporter {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn report_error(&mut self, error: &InfraError) {
        self.had_error = true;
        
        match error {
            InfraError::LexError { message, line, column } => {
                self.report_at(*line, *column, "Lexical Error", message);
            }
            InfraError::ParseError { message, line, column } => {
                self.report_at(*line, *column, "Parse Error", message);
            }
            InfraError::RuntimeError { message } => {
                eprintln!("Runtime Error: {}", message);
            }
            InfraError::TypeError { expected, found } => {
                eprintln!("Type Error: expected {}, found {}", expected, found);
            }
            InfraError::DivisionByZero => {
                eprintln!("Runtime Error: Division by zero");
            }
            InfraError::UndefinedVariable { name } => {
                eprintln!("Runtime Error: Undefined variable '{}'", name);
            }
            InfraError::UndefinedFunction { name } => {
                eprintln!("Runtime Error: Undefined function '{}'", name);
            }
            InfraError::ArgumentCountMismatch { expected, found } => {
                eprintln!("Runtime Error: Expected {} arguments, found {}", expected, found);
            }
            InfraError::IndexOutOfBounds { index, length } => {
                eprintln!("Runtime Error: Array index {} out of bounds for array of length {}", index, length);
            }
            InfraError::PropertyNotFound { property } => {
                eprintln!("Runtime Error: Property '{}' not found on object", property);
            }
            InfraError::Return(value) => {
                // This should not be reported as an error in normal operation
                if let Some(val) = value {
                    eprintln!("Unexpected return: {}", val);
                } else {
                    eprintln!("Unexpected return");
                }
            }
            InfraError::IoError { message } => {
                eprintln!("I/O Error: {}", message);
            }
            InfraError::Exception { message } => {
                eprintln!("Exception: {}", message);
            }
            InfraError::Runtime(message) => {
                eprintln!("Runtime Error: {}", message);
            }
        }
        
        io::stderr().flush().unwrap();
    }

    pub fn had_error(&self) -> bool {
        self.had_error
    }

    pub fn reset(&mut self) {
        self.had_error = false;
    }

    fn report_at(&self, line: usize, column: usize, error_type: &str, message: &str) {
        eprintln!("[line {}, column {}] {}: {}", line, column, error_type, message);
    }
}

impl Default for ErrorReporter {
    fn default() -> Self {
        Self::new()
    }
}
