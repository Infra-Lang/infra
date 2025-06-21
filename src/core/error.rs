use std::fmt;

#[derive(Debug, Clone)]
pub enum InfraError {
    LexError { message: String, line: usize, column: usize },
    ParseError { message: String, line: usize, column: usize },
    RuntimeError { message: String },
    TypeError { expected: String, found: String },
    DivisionByZero,
    UndefinedVariable { name: String },
    UndefinedFunction { name: String },
    ArgumentCountMismatch { expected: usize, found: usize },
    IndexOutOfBounds { index: usize, length: usize },
    PropertyNotFound { property: String },
    Return(Option<crate::core::Value>), // Special error for early returns
    IoError { message: String },
    Exception { message: String }, // For try/catch blocks
    Runtime(String), // General runtime error
}

impl fmt::Display for InfraError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InfraError::LexError { message, line, column } => {
                write!(f, "Lexer error at line {}, column {}: {}", line, column, message)
            }
            InfraError::ParseError { message, line, column } => {
                write!(f, "Parser error at line {}, column {}: {}", line, column, message)
            }
            InfraError::RuntimeError { message } => {
                write!(f, "Runtime error: {}", message)
            }
            InfraError::TypeError { expected, found } => {
                write!(f, "Type error: expected {}, found {}", expected, found)
            }
            InfraError::DivisionByZero => {
                write!(f, "Runtime error: Division by zero")
            }
            InfraError::UndefinedVariable { name } => {
                write!(f, "Runtime error: Undefined variable '{}'", name)
            }
            InfraError::UndefinedFunction { name } => {
                write!(f, "Runtime error: Undefined function '{}'", name)
            }
            InfraError::ArgumentCountMismatch { expected, found } => {
                write!(f, "Runtime error: Expected {} arguments, found {}", expected, found)
            }
            InfraError::IndexOutOfBounds { index, length } => {
                write!(f, "Runtime error: Array index {} out of bounds for array of length {}", index, length)
            }
            InfraError::PropertyNotFound { property } => {
                write!(f, "Runtime error: Property '{}' not found on object", property)
            }
            InfraError::Return(value) => {
                write!(f, "Return value: {}", 
                    value.as_ref().map(|v| v.to_string()).unwrap_or("null".to_string()))
            }
            InfraError::IoError { message } => {
                write!(f, "I/O error: {}", message)
            }
            InfraError::Exception { message } => {
                write!(f, "Exception: {}", message)
            }
            InfraError::Runtime(message) => {
                write!(f, "Runtime error: {}", message)
            }
        }
    }
}

impl std::error::Error for InfraError {}

pub type Result<T> = std::result::Result<T, InfraError>;
