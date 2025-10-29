use crate::core::{InfraError, Result, Value};

/// Get string length
#[allow(dead_code)]
pub fn length(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 1,
            found: args.len(),
            function_name: Some("string_length".to_string()),
            line: None,
        });
    }

    match &args[0] {
        Value::String(s) => Ok(Value::Number(s.len() as f64)),
        _ => Err(InfraError::TypeError {
            expected: "string".to_string(),
            found: args[0].type_name().to_string(),
            context: Some("string_length() function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Split string by delimiter
#[allow(dead_code)]
pub fn split(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 2,
            found: args.len(),
            function_name: Some("string_split".to_string()),
            line: None,
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
            context: Some("string_split() function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Join array of strings with delimiter
#[allow(dead_code)]
pub fn join(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 2,
            found: args.len(),
            function_name: Some("string_join".to_string()),
            line: None,
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
                            context: Some("string_join() function".to_string()),
                            line: None,
                            column: None,
                            hint: None,
                        })
                    }
                }
            }

            Ok(Value::String(string_parts.join(delimiter)))
        }
        _ => Err(InfraError::TypeError {
            expected: "array and string".to_string(),
            found: format!("{} and {}", args[0].type_name(), args[1].type_name()),
            context: Some("string_join() function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Convert string to uppercase
#[allow(dead_code)]
pub fn upper(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 1,
            found: args.len(),
            function_name: Some("string_upper".to_string()),
            line: None,
        });
    }

    match &args[0] {
        Value::String(s) => Ok(Value::String(s.to_uppercase())),
        _ => Err(InfraError::TypeError {
            expected: "string".to_string(),
            found: args[0].type_name().to_string(),
            context: Some("string_upper() function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Convert string to lowercase
#[allow(dead_code)]
pub fn lower(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 1,
            found: args.len(),
            function_name: Some("string_lower".to_string()),
            line: None,
        });
    }

    match &args[0] {
        Value::String(s) => Ok(Value::String(s.to_lowercase())),
        _ => Err(InfraError::TypeError {
            expected: "string".to_string(),
            found: args[0].type_name().to_string(),
            context: Some("string_lower() function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Trim whitespace from string
#[allow(dead_code)]
pub fn trim(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 1,
            found: args.len(),
            function_name: Some("string_trim".to_string()),
            line: None,
        });
    }

    match &args[0] {
        Value::String(s) => Ok(Value::String(s.trim().to_string())),
        _ => Err(InfraError::TypeError {
            expected: "string".to_string(),
            found: args[0].type_name().to_string(),
            context: Some("string_trim() function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Check if string contains substring
#[allow(dead_code)]
pub fn contains(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 2,
            found: args.len(),
            function_name: Some("string_contains".to_string()),
            line: None,
        });
    }

    match (&args[0], &args[1]) {
        (Value::String(text), Value::String(substring)) => {
            Ok(Value::Boolean(text.contains(substring)))
        }
        _ => Err(InfraError::TypeError {
            expected: "two strings".to_string(),
            found: format!("{} and {}", args[0].type_name(), args[1].type_name()),
            context: Some("string_contains() function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Get substring from start index to end index
#[allow(dead_code)]
pub fn substring(args: &[Value]) -> Result<Value> {
    if args.len() != 3 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 3,
            found: args.len(),
            function_name: Some("string_substring".to_string()),
            line: None,
        });
    }

    match (&args[0], &args[1], &args[2]) {
        (Value::String(s), Value::Number(start), Value::Number(end)) => {
            let start_idx = *start as usize;
            let end_idx = *end as usize;

            if start_idx > s.len() || end_idx > s.len() || start_idx > end_idx {
                return Err(InfraError::RuntimeError {
                    message: "Substring indices out of bounds".to_string(),
                    line: None,
                    column: None,
                    stack_trace: vec![],
                    source_code: None,
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
            context: Some("string_substring() function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Replace occurrences of a substring with another string
#[allow(dead_code)]
pub fn replace(args: &[Value]) -> Result<Value> {
    if args.len() != 3 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 3,
            found: args.len(),
            function_name: Some("string_replace".to_string()),
            line: None,
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
            context: Some("string_replace() function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Check if string starts with a prefix
#[allow(dead_code)]
pub fn starts_with(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 2,
            found: args.len(),
            function_name: Some("string_starts_with".to_string()),
            line: None,
        });
    }

    match (&args[0], &args[1]) {
        (Value::String(text), Value::String(prefix)) => {
            Ok(Value::Boolean(text.starts_with(prefix)))
        }
        _ => Err(InfraError::TypeError {
            expected: "two strings".to_string(),
            found: format!("{} and {}", args[0].type_name(), args[1].type_name()),
            context: Some("string_starts_with() function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Check if string ends with a suffix
#[allow(dead_code)]
pub fn ends_with(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 2,
            found: args.len(),
            function_name: Some("string_ends_with".to_string()),
            line: None,
        });
    }

    match (&args[0], &args[1]) {
        (Value::String(text), Value::String(suffix)) => Ok(Value::Boolean(text.ends_with(suffix))),
        _ => Err(InfraError::TypeError {
            expected: "two strings".to_string(),
            found: format!("{} and {}", args[0].type_name(), args[1].type_name()),
            context: Some("string_ends_with() function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Repeat a string n times
#[allow(dead_code)]
pub fn repeat(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 2,
            found: args.len(),
            function_name: Some("string_repeat".to_string()),
            line: None,
        });
    }

    match (&args[0], &args[1]) {
        (Value::String(text), Value::Number(count)) => {
            if *count < 0.0 {
                return Err(InfraError::RuntimeError {
                    message: "Repeat count cannot be negative".to_string(),
                    line: None,
                    column: None,
                    stack_trace: vec![],
                    source_code: None,
                });
            }

            let repeat_count = *count as usize;
            Ok(Value::String(text.repeat(repeat_count)))
        }
        _ => Err(InfraError::TypeError {
            expected: "string and number".to_string(),
            found: format!("{} and {}", args[0].type_name(), args[1].type_name()),
            context: Some("string_repeat() function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Pad string to a certain length with spaces on the left
#[allow(dead_code)]
pub fn pad_left(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 2,
            found: args.len(),
            function_name: Some("string_pad_left".to_string()),
            line: None,
        });
    }

    match (&args[0], &args[1]) {
        (Value::String(text), Value::Number(width)) => {
            if *width < 0.0 {
                return Err(InfraError::RuntimeError {
                    message: "Pad width cannot be negative".to_string(),
                    line: None,
                    column: None,
                    stack_trace: vec![],
                    source_code: None,
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
            context: Some("string_pad_left() function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Pad string to a certain length with spaces on the right
#[allow(dead_code)]
pub fn pad_right(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 2,
            found: args.len(),
            function_name: Some("string_pad_right".to_string()),
            line: None,
        });
    }

    match (&args[0], &args[1]) {
        (Value::String(text), Value::Number(width)) => {
            if *width < 0.0 {
                return Err(InfraError::RuntimeError {
                    message: "Pad width cannot be negative".to_string(),
                    line: None,
                    column: None,
                    stack_trace: vec![],
                    source_code: None,
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
            context: Some("string_pad_right() function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}
