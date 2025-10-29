use std::fmt;

#[derive(Debug, Clone)]
pub enum InfraError {
    LexError {
        message: String,
        line: usize,
        column: usize,
        source_code: Option<String>,
    },
    ParseError {
        message: String,
        line: usize,
        column: usize,
        source_code: Option<String>,
        hint: Option<String>,
    },
    RuntimeError {
        message: String,
        line: Option<usize>,
        column: Option<usize>,
        stack_trace: Vec<String>,
        source_code: Option<String>,
    },
    TypeError {
        expected: String,
        found: String,
        context: Option<String>,
        line: Option<usize>,
        column: Option<usize>,
        hint: Option<String>,
    },
    DivisionByZero {
        line: Option<usize>,
        column: Option<usize>,
    },
    UndefinedVariable {
        name: String,
        line: Option<usize>,
        column: Option<usize>,
        suggestion: Option<String>,
    },
    UndefinedFunction {
        name: String,
        line: Option<usize>,
        column: Option<usize>,
        suggestion: Option<String>,
    },
    ArgumentCountMismatch {
        expected: usize,
        found: usize,
        function_name: Option<String>,
        line: Option<usize>,
    },
    IndexOutOfBounds {
        index: usize,
        length: usize,
        array_name: Option<String>,
        line: Option<usize>,
    },
    PropertyNotFound {
        property: String,
        object_type: Option<String>,
        line: Option<usize>,
        available_properties: Option<Vec<String>>,
    },
    ReturnValue(Option<crate::core::Value>), // Renamed from Return
    IoError {
        message: String,
        operation: Option<String>,
        path: Option<String>,
    },
    Exception {
        message: String,
        exception_type: Option<String>,
        line: Option<usize>,
        stack_trace: Vec<String>,
    },
    ModuleError {
        module_name: String,
        reason: String,
    },
    AsyncError {
        message: String,
        operation: Option<String>,
    },
    ClassError {
        message: String,
        class_name: Option<String>,
        method_name: Option<String>,
        line: Option<usize>,
    },
    MemoryError {
        message: String,
        operation: Option<String>,
    },
    Generic(String), // General fallback error
}

impl fmt::Display for InfraError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InfraError::LexError {
                message,
                line,
                column,
                source_code: _,
            } => {
                write!(
                    f,
                    "Lexer error at line {}, column {}: {}",
                    line, column, message
                )
            }
            InfraError::ParseError {
                message,
                line,
                column,
                source_code: _,
                hint,
            } => {
                write!(
                    f,
                    "Parser error at line {}, column {}: {}",
                    line, column, message
                )?;
                if let Some(hint_str) = hint {
                    write!(f, "\n  Hint: {}", hint_str)?;
                }
                Ok(())
            }
            InfraError::RuntimeError {
                message,
                line,
                column,
                stack_trace,
                source_code: _,
            } => {
                if let (Some(l), Some(c)) = (line, column) {
                    write!(f, "Runtime error at line {}, column {}: {}", l, c, message)?;
                } else {
                    write!(f, "Runtime error: {}", message)?;
                }

                if !stack_trace.is_empty() {
                    write!(f, "\nStack trace:")?;
                    for frame in stack_trace {
                        write!(f, "\n  {}", frame)?;
                    }
                }
                Ok(())
            }
            InfraError::TypeError {
                expected,
                found,
                context,
                line,
                column: _,
                hint,
            } => {
                if let Some(ctx) = context {
                    write!(
                        f,
                        "Type error in {}: expected {}, found {}",
                        ctx, expected, found
                    )?;
                } else {
                    write!(f, "Type error: expected {}, found {}", expected, found)?;
                }

                if let Some(hint_str) = hint {
                    write!(f, "\n  Hint: {}", hint_str)?;
                }
                Ok(())
            }
            InfraError::DivisionByZero { line: _, column: _ } => {
                write!(f, "Runtime error: Division by zero")
            }
            InfraError::UndefinedVariable {
                name,
                line: _,
                column: _,
                suggestion,
            } => {
                write!(f, "Runtime error: Undefined variable '{}'", name)?;
                if let Some(sugg) = suggestion {
                    write!(f, "\n  Did you mean '{}'?", sugg)?;
                }
                Ok(())
            }
            InfraError::UndefinedFunction {
                name,
                line: _,
                column: _,
                suggestion,
            } => {
                write!(f, "Runtime error: Undefined function '{}'", name)?;
                if let Some(sugg) = suggestion {
                    write!(f, "\n  Did you mean '{}'?", sugg)?;
                }
                Ok(())
            }
            InfraError::ArgumentCountMismatch {
                expected,
                found,
                function_name,
                line: _,
            } => {
                if let Some(func_name) = function_name {
                    write!(
                        f,
                        "Runtime error: Function '{}' expected {} arguments, found {}",
                        func_name, expected, found
                    )
                } else {
                    write!(
                        f,
                        "Runtime error: Expected {} arguments, found {}",
                        expected, found
                    )
                }
            }
            InfraError::IndexOutOfBounds {
                index,
                length,
                array_name,
                line: _,
            } => {
                if let Some(name) = array_name {
                    write!(
                        f,
                        "Runtime error: Array index {} out of bounds for '{}' (length: {})",
                        index, name, length
                    )
                } else {
                    write!(
                        f,
                        "Runtime error: Array index {} out of bounds for array of length {}",
                        index, length
                    )
                }
            }
            InfraError::PropertyNotFound {
                property,
                object_type,
                line: _,
                available_properties,
            } => {
                if let Some(obj_type) = object_type {
                    write!(
                        f,
                        "Runtime error: Property '{}' not found on {}",
                        property, obj_type
                    )?;
                } else {
                    write!(
                        f,
                        "Runtime error: Property '{}' not found on object",
                        property
                    )?;
                }

                if let Some(props) = available_properties {
                    write!(f, "\n  Available properties: {}", props.join(", "))?;
                }
                Ok(())
            }
            InfraError::ReturnValue(value) => {
                write!(
                    f,
                    "Return value: {}",
                    value
                        .as_ref()
                        .map(|v| v.to_string())
                        .unwrap_or("null".to_string())
                )
            }
            InfraError::IoError {
                message,
                operation,
                path,
            } => {
                write!(f, "I/O error: {}", message)?;
                if let Some(op) = operation {
                    write!(f, " during operation '{}'", op)?;
                }
                if let Some(p) = path {
                    write!(f, " at path '{}'", p)?;
                }
                Ok(())
            }
            InfraError::Exception {
                message,
                exception_type,
                line: _,
                stack_trace,
            } => {
                if let Some(exc_type) = exception_type {
                    write!(f, "{}: {}", exc_type, message)?;
                } else {
                    write!(f, "Exception: {}", message)?;
                }

                if !stack_trace.is_empty() {
                    write!(f, "\nStack trace:")?;
                    for frame in stack_trace {
                        write!(f, "\n  {}", frame)?;
                    }
                }
                Ok(())
            }
            InfraError::ModuleError {
                module_name,
                reason,
            } => {
                write!(
                    f,
                    "Module error: Could not load '{}': {}",
                    module_name, reason
                )
            }
            InfraError::AsyncError { message, operation } => {
                if let Some(op) = operation {
                    write!(f, "Async error in {}: {}", op, message)
                } else {
                    write!(f, "Async error: {}", message)
                }
            }
            InfraError::ClassError {
                message,
                class_name,
                method_name,
                line: _,
            } => {
                if let Some(method) = method_name {
                    write!(f, "Class error in method '{}': {}", method, message)?;
                    if let Some(class) = class_name {
                        write!(f, " (class: {})", class)?;
                    }
                } else if let Some(class) = class_name {
                    write!(f, "Class error in '{}': {}", class, message)?;
                } else {
                    write!(f, "Class error: {}", message)?;
                }
                Ok(())
            }
            InfraError::MemoryError { message, operation } => {
                if let Some(op) = operation {
                    write!(f, "Memory error during {}: {}", op, message)
                } else {
                    write!(f, "Memory error: {}", message)
                }
            }
            InfraError::Generic(message) => {
                write!(f, "Error: {}", message)
            }
        }
    }
}

impl std::error::Error for InfraError {}

pub type Result<T> = std::result::Result<T, InfraError>;
