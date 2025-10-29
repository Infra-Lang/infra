use crate::core::{InfraError, Result, Value};

/// Get string length
pub fn length(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 1,
            found: args.len(),
        });
    }

    match &args[0] {
        Value::String(s) => Ok(Value::Number(s.len() as f64)),
        _ => Err(InfraError::TypeError {
            expected: "string".to_string(),
            found: args[0].type_name().to_string(),
            context: None,
        ,
            context: None,
        }),
    }
}

/// Split string by delimiter
pub fn split(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 2,
            found: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::String(text), Value::String(delimiter)) => {
            let parts: Vec<Value> = text
                .split(delimiter)
                .map(|s| Value::String(s.to_string()))
                .collect();
            Ok(Value::Array(parts))
        }
        _ => Err(InfraError::TypeError {
            expected: "two strings".to_string(),
            found: format!("{} and {}", args[0].type_name(), args[1].type_name()),
            context: Some("string.split() function".to_string()),
        ,
            context: None,
        }),
    }
}

/// Join array of strings with delimiter
pub fn join(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 2,
            found: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::Array(arr), Value::String(delimiter)) => {
            let mut string_parts = Vec::new();

            for item in arr {
                match item {
                    Value::String(s) => string_parts.push(s.clone()),
                    _ => {
                        return Err(InfraError::TypeError {
                            expected: "array of strings".to_string(),
                            found: format!("array containing {}", item.type_name()),
                            context: Some("string.join() function".to_string()),
                        })
                    }
                }
            }

            Ok(Value::String(string_parts.join(delimiter)))
        }
        _ => Err(InfraError::TypeError {
            expected: "array and string".to_string(),
            found: format!("{} and {}", args[0].type_name(), args[1].type_name()),
            context: None,
        }),
    }
}

/// Convert string to uppercase
pub fn upper(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 1,
            found: args.len(),
        });
    }

    match &args[0] {
        Value::String(s) => Ok(Value::String(s.to_uppercase())),
        _ => Err(InfraError::TypeError {
            expected: "string".to_string(),
            found: args[0].type_name().to_string(),
            context: None,
        ,
            context: None,
        }),
    }
}

/// Convert string to lowercase
pub fn lower(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 1,
            found: args.len(),
        });
    }

    match &args[0] {
        Value::String(s) => Ok(Value::String(s.to_lowercase())),
        _ => Err(InfraError::TypeError {
            expected: "string".to_string(),
            found: args[0].type_name().to_string(),
            context: None,
        ,
            context: None,
        }),
    }
}

/// Trim whitespace from string
pub fn trim(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 1,
            found: args.len(),
        });
    }

    match &args[0] {
        Value::String(s) => Ok(Value::String(s.trim().to_string())),
        _ => Err(InfraError::TypeError {
            expected: "string".to_string(),
            found: args[0].type_name().to_string(),
            context: None,
        ,
            context: None,
        }),
    }
}

/// Check if string contains substring
pub fn contains(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 2,
            found: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::String(text), Value::String(substring)) => {
            Ok(Value::Boolean(text.contains(substring)))
        }
        _ => Err(InfraError::TypeError {
            expected: "two strings".to_string(),
            found: format!("{} and {}", args[0].type_name(), args[1].type_name()),
        ,
            context: None,
        }),
    }
}

/// Get substring from start index to end index
pub fn substring(args: &[Value]) -> Result<Value> {
    if args.len() != 3 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 3,
            found: args.len(),
        });
    }

    match (&args[0], &args[1], &args[2]) {
        (Value::String(s), Value::Number(start), Value::Number(end)) => {
            let start_idx = *start as usize;
            let end_idx = *end as usize;

            if start_idx > s.len() || end_idx > s.len() || start_idx > end_idx {
                return Err(InfraError::RuntimeError {
                    message: "Substring indices out of bounds".to_string(),
                });
            }

            Ok(Value::String(s[start_idx..end_idx].to_string()))
        }
        _ => Err(InfraError::TypeError {
            expected: "string and two numbers".to_string(),
            found: format!(
                "{}, {}, and {}",
                args[0].type_name(),
                args[1].type_name(),
                args[2].type_name()
            ),
        ,
            context: None,
        }),
    }
}

/// Replace occurrences of a substring with another string
pub fn replace(args: &[Value]) -> Result<Value> {
    if args.len() != 3 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 3,
            found: args.len(),
        });
    }

    match (&args[0], &args[1], &args[2]) {
        (Value::String(text), Value::String(from), Value::String(to)) => {
            Ok(Value::String(text.replace(from, to)))
        }
        _ => Err(InfraError::TypeError {
            expected: "three strings".to_string(),
            found: format!(
                "{}, {}, and {}",
                args[0].type_name(),
                args[1].type_name(),
                args[2].type_name()
            ),
        ,
            context: None,
        }),
    }
}

/// Check if string starts with a prefix
pub fn starts_with(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 2,
            found: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::String(text), Value::String(prefix)) => {
            Ok(Value::Boolean(text.starts_with(prefix)))
        }
        _ => Err(InfraError::TypeError {
            expected: "two strings".to_string(),
            found: format!("{} and {}", args[0].type_name(), args[1].type_name()),
        ,
            context: None,
        }),
    }
}

/// Check if string ends with a suffix
pub fn ends_with(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 2,
            found: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::String(text), Value::String(suffix)) => Ok(Value::Boolean(text.ends_with(suffix))),
        _ => Err(InfraError::TypeError {
            expected: "two strings".to_string(),
            found: format!("{} and {}", args[0].type_name(), args[1].type_name()),
        ,
            context: None,
        }),
    }
}

/// Repeat a string n times
pub fn repeat(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 2,
            found: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::String(text), Value::Number(count)) => {
            if *count < 0.0 {
                return Err(InfraError::RuntimeError {
                    message: "Repeat count cannot be negative".to_string(),
                });
            }

            let repeat_count = *count as usize;
            Ok(Value::String(text.repeat(repeat_count)))
        }
        _ => Err(InfraError::TypeError {
            expected: "string and number".to_string(),
            found: format!("{} and {}", args[0].type_name(), args[1].type_name()),
        ,
            context: None,
        }),
    }
}

/// Pad string to a certain length with spaces on the left
pub fn pad_left(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 2,
            found: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::String(text), Value::Number(width)) => {
            if *width < 0.0 {
                return Err(InfraError::RuntimeError {
                    message: "Pad width cannot be negative".to_string(),
                });
            }

            let target_width = *width as usize;
            if text.len() >= target_width {
                Ok(Value::String(text.clone()))
            } else {
                let padding = " ".repeat(target_width - text.len());
                Ok(Value::String(format!("{}{}", padding, text)))
            }
        }
        _ => Err(InfraError::TypeError {
            expected: "string and number".to_string(),
            found: format!("{} and {}", args[0].type_name(), args[1].type_name()),
        ,
            context: None,
        }),
    }
}

/// Pad string to a certain length with spaces on the right
pub fn pad_right(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 2,
            found: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::String(text), Value::Number(width)) => {
            if *width < 0.0 {
                return Err(InfraError::RuntimeError {
                    message: "Pad width cannot be negative".to_string(),
                });
            }

            let target_width = *width as usize;
            if text.len() >= target_width {
                Ok(Value::String(text.clone()))
            } else {
                let padding = " ".repeat(target_width - text.len());
                Ok(Value::String(format!("{}{}", text, padding)))
            }
        }
        _ => Err(InfraError::TypeError {
            expected: "string and number".to_string(),
            found: format!("{} and {}", args[0].type_name(), args[1].type_name()),
        ,
            context: None,
        }),
    }
}
