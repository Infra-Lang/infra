use crate::core::{Value, InfraError, Result};
use std::fs;

/// Read file contents as string
pub fn read_file(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch { 
            expected: 1, 
            found: args.len() 
        });
    }
    
    match &args[0] {
        Value::String(filename) => {
            match fs::read_to_string(filename) {
                Ok(content) => Ok(Value::String(content)),
                Err(e) => Err(InfraError::IoError {
                    message: format!("Failed to read file '{}': {}", filename, e),
                }),
            }
        }
        _ => Err(InfraError::TypeError {
            expected: "string".to_string(),
            found: args[0].type_name().to_string(),
        }),
    }
}

/// Write string content to file
pub fn write_file(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(InfraError::ArgumentCountMismatch { 
            expected: 2, 
            found: args.len() 
        });
    }
    
    match (&args[0], &args[1]) {
        (Value::String(filename), Value::String(content)) => {
            match fs::write(filename, content) {
                Ok(()) => Ok(Value::Null),
                Err(e) => Err(InfraError::IoError {
                    message: format!("Failed to write file '{}': {}", filename, e),
                }),
            }
        }
        _ => Err(InfraError::TypeError {
            expected: "two strings".to_string(),
            found: format!("{} and {}", args[0].type_name(), args[1].type_name()),
        }),
    }
}

/// Check if file exists
pub fn exists(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch { 
            expected: 1, 
            found: args.len() 
        });
    }
    
    match &args[0] {
        Value::String(filename) => {
            Ok(Value::Boolean(std::path::Path::new(filename).exists()))
        }
        _ => Err(InfraError::TypeError {
            expected: "string".to_string(),
            found: args[0].type_name().to_string(),
        }),
    }
}

/// Throw an exception that can be caught by try/catch
pub fn throw_exception(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch { 
            expected: 1, 
            found: args.len() 
        });
    }
    
    match &args[0] {
        Value::String(message) => {
            Err(InfraError::Exception { message: message.clone() })
        }
        _ => Err(InfraError::TypeError {
            expected: "string".to_string(),
            found: args[0].type_name().to_string(),
        }),
    }
}
