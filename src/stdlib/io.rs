use crate::core::{InfraError, Result, Value};
use std::fs;

/// Read file contents as string
#[allow(dead_code)]
pub fn read_file(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 1,
            found: args.len(),
            function_name: Some("file_read".to_string()),
            line: None,
        });
    }

    match &args[0] {
        Value::String(filename) => match fs::read_to_string(filename) {
            Ok(content) => Ok(Value::String(content)),
            Err(e) => Err(InfraError::IoError {
                message: format!("Failed to read file '{}': {}", filename, e),
                operation: Some("file_read".to_string()),
                path: Some(filename.clone()),
            }),
        },
        _ => Err(InfraError::TypeError {
            expected: "string".to_string(),
            found: args[0].type_name().to_string(),
            context: Some("file_read() function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Write string content to file
#[allow(dead_code)]
pub fn write_file(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 2,
            found: args.len(),
            function_name: Some("file_write".to_string()),
            line: None,
        });
    }

    match (&args[0], &args[1]) {
        (Value::String(filename), Value::String(content)) => match fs::write(filename, content) {
            Ok(()) => Ok(Value::Null),
            Err(e) => Err(InfraError::IoError {
                message: format!("Failed to write file '{}': {}", filename, e),
                operation: Some("file_write".to_string()),
                path: Some(filename.clone()),
            }),
        },
        _ => Err(InfraError::TypeError {
            expected: "two strings".to_string(),
            found: format!("{} and {}", args[0].type_name(), args[1].type_name()),
            context: Some("file_write() function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Check if file exists
#[allow(dead_code)]
pub fn exists(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 1,
            found: args.len(),
            function_name: Some("file_exists".to_string()),
            line: None,
        });
    }

    match &args[0] {
        Value::String(filename) => Ok(Value::Boolean(std::path::Path::new(filename).exists())),
        _ => Err(InfraError::TypeError {
            expected: "string".to_string(),
            found: args[0].type_name().to_string(),
            context: Some("file_exists() function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Throw an exception that can be caught by try/catch
#[allow(dead_code)]
pub fn throw_exception(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 1,
            found: args.len(),
            function_name: Some("throw_exception".to_string()),
            line: None,
        });
    }

    match &args[0] {
        Value::String(message) => Err(InfraError::Exception {
            message: message.clone(),
            exception_type: None,
            line: None,
            stack_trace: vec![],
        }),
        _ => Err(InfraError::TypeError {
            expected: "string".to_string(),
            found: args[0].type_name().to_string(),
            context: Some("throw_exception() function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}
