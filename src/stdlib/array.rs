use crate::core::{InfraError, Result, Value};

/// Get array length
pub fn length(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 1,
            found: args.len(),
            function_name: Some("array.length".to_string()),
            line: None,
        });
    }

    match &args[0] {
        Value::Array(arr) => Ok(Value::Number(arr.len() as f64)),
        _ => Err(InfraError::TypeError {
            expected: "array".to_string(),
            found: args[0].type_name().to_string(),
            context: Some("array.length function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Push element to array (returns new array)
pub fn push(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 2,
            found: args.len(),
            function_name: Some("array.push".to_string()),
            line: None,
        });
    }

    match &args[0] {
        Value::Array(arr) => {
            let mut new_arr = arr.clone();
            new_arr.push(args[1].clone());
            Ok(Value::Array(new_arr))
        }
        _ => Err(InfraError::TypeError {
            expected: "array".to_string(),
            found: args[0].type_name().to_string(),
            context: Some("array.push function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Pop last element from array (returns new array)
pub fn pop(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 1,
            found: args.len(),
            function_name: Some("array.pop".to_string()),
            line: None,
        });
    }

    match &args[0] {
        Value::Array(arr) => {
            if arr.is_empty() {
                Err(InfraError::RuntimeError {
                    message: "Cannot pop from empty array".to_string(),
                    line: None,
                    column: None,
                    stack_trace: vec![],
                    source_code: None,
                })
            } else {
                let mut new_arr = arr.clone();
                new_arr.pop();
                Ok(Value::Array(new_arr))
            }
        }
        _ => Err(InfraError::TypeError {
            expected: "array".to_string(),
            found: args[0].type_name().to_string(),
            context: Some("array.pop function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Sort array (only works with arrays of numbers or strings)
pub fn sort(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 1,
            found: args.len(),
            function_name: Some("array.sort".to_string()),
            line: None,
        });
    }

    match &args[0] {
        Value::Array(arr) => {
            if arr.is_empty() {
                return Ok(Value::Array(vec![]));
            }

            // Check if all elements are the same type
            let first_type = arr[0].type_name();
            if !arr.iter().all(|v| v.type_name() == first_type) {
                return Err(InfraError::RuntimeError {
                    message: "Cannot sort array with mixed types".to_string(),
                    line: None,
                    column: None,
                    stack_trace: vec![],
                    source_code: None,
                });
            }

            let mut sorted_arr = arr.clone();

            match &arr[0] {
                Value::Number(_) => {
                    sorted_arr.sort_by(|a, b| {
                        if let (Value::Number(x), Value::Number(y)) = (a, b) {
                            x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
                        } else {
                            std::cmp::Ordering::Equal
                        }
                    });
                }
                Value::String(_) => {
                    sorted_arr.sort_by(|a, b| {
                        if let (Value::String(x), Value::String(y)) = (a, b) {
                            x.cmp(y)
                        } else {
                            std::cmp::Ordering::Equal
                        }
                    });
                }
                _ => {
                    return Err(InfraError::RuntimeError {
                        message: format!("Cannot sort array of {}", first_type),
                        line: None,
                        column: None,
                        stack_trace: vec![],
                        source_code: None,
                    });
                }
            }

            Ok(Value::Array(sorted_arr))
        }
        _ => Err(InfraError::TypeError {
            expected: "array".to_string(),
            found: args[0].type_name().to_string(),
            context: Some("array.sort function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Reverse array
pub fn reverse(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 1,
            found: args.len(),
            function_name: Some("array.reverse".to_string()),
            line: None,
        });
    }

    match &args[0] {
        Value::Array(arr) => {
            let mut reversed_arr = arr.clone();
            reversed_arr.reverse();
            Ok(Value::Array(reversed_arr))
        }
        _ => Err(InfraError::TypeError {
            expected: "array".to_string(),
            found: args[0].type_name().to_string(),
            context: Some("array.reverse function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Join array elements into a string
pub fn join(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 2,
            found: args.len(),
            function_name: Some("array.join".to_string()),
            line: None,
        });
    }

    match (&args[0], &args[1]) {
        (Value::Array(arr), Value::String(delimiter)) => {
            let string_parts: Result<Vec<String>> = arr
                .iter()
                .map(|v| match v {
                    Value::String(s) => Ok(s.clone()),
                    Value::Number(n) => Ok(n.to_string()),
                    Value::Boolean(b) => Ok(b.to_string()),
                    Value::Null => Ok("null".to_string()),
                    _ => Err(InfraError::RuntimeError {
                        message: format!("Cannot convert {} to string for joining", v.type_name()),
                        line: None,
                        column: None,
                        stack_trace: vec![],
                        source_code: None,
                    }),
                })
                .collect();

            match string_parts {
                Ok(parts) => Ok(Value::String(parts.join(delimiter))),
                Err(e) => Err(e),
            }
        }
        _ => Err(InfraError::TypeError {
            expected: "array and string".to_string(),
            found: format!("{} and {}", args[0].type_name(), args[1].type_name()),
            context: Some("array.join function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Map function over array elements (functional programming)
/// Syntax: array.map(arr, function)
/// Note: Since we can't pass functions as values yet, this is a placeholder
/// for when function values are implemented
pub fn map(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 2,
            found: args.len(),
            function_name: Some("array.map".to_string()),
            line: None,
        });
    }

    // For now, we'll implement a basic transformation that doubles numbers
    // This is a placeholder until function values are implemented
    match &args[0] {
        Value::Array(arr) => {
            let mapped: Result<Vec<Value>> = arr
                .iter()
                .map(|v| match v {
                    Value::Number(n) => Ok(Value::Number(n * 2.0)), // Example: double numbers
                    _ => Ok(v.clone()),                             // Keep other types unchanged
                })
                .collect();

            match mapped {
                Ok(result_arr) => Ok(Value::Array(result_arr)),
                Err(e) => Err(e),
            }
        }
        _ => Err(InfraError::TypeError {
            expected: "array".to_string(),
            found: args[0].type_name().to_string(),
            context: Some("array.map function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Filter array elements based on a condition
/// For now, filters numbers greater than the second argument
pub fn filter(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 2,
            found: args.len(),
            function_name: Some("array.filter".to_string()),
            line: None,
        });
    }

    match (&args[0], &args[1]) {
        (Value::Array(arr), Value::Number(threshold)) => {
            let filtered: Vec<Value> = arr
                .iter()
                .filter(|v| match v {
                    Value::Number(n) => n > threshold,
                    _ => false, // Non-numbers are filtered out
                })
                .cloned()
                .collect();

            Ok(Value::Array(filtered))
        }
        _ => Err(InfraError::TypeError {
            expected: "array and number".to_string(),
            found: format!("{} and {}", args[0].type_name(), args[1].type_name()),
            context: Some("array.filter function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Reduce array to a single value (sum for numbers)
pub fn reduce(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 1,
            found: args.len(),
            function_name: Some("array.reduce".to_string()),
            line: None,
        });
    }

    match &args[0] {
        Value::Array(arr) => {
            if arr.is_empty() {
                return Ok(Value::Number(0.0));
            }

            // For now, sum all numbers in the array
            let mut sum = 0.0;
            for item in arr {
                match item {
                    Value::Number(n) => sum += n,
                    _ => {
                        return Err(InfraError::RuntimeError {
                            message: "reduce currently only supports arrays of numbers".to_string(),
                            line: None,
                            column: None,
                            stack_trace: vec![],
                            source_code: None,
                        })
                    }
                }
            }

            Ok(Value::Number(sum))
        }
        _ => Err(InfraError::TypeError {
            expected: "array".to_string(),
            found: args[0].type_name().to_string(),
            context: Some("array.reduce function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Find first element matching a condition
/// For now, finds first number greater than the second argument
pub fn find(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 2,
            found: args.len(),
            function_name: Some("array.find".to_string()),
            line: None,
        });
    }

    match (&args[0], &args[1]) {
        (Value::Array(arr), Value::Number(target)) => {
            for item in arr {
                if let Value::Number(n) = item {
                    if n > target {
                        return Ok(item.clone());
                    }
                }
            }

            Ok(Value::Null) // Return null if not found
        }
        _ => Err(InfraError::TypeError {
            expected: "array and number".to_string(),
            found: format!("{} and {}", args[0].type_name(), args[1].type_name()),
            context: Some("array.find function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Check if array contains a specific element
pub fn contains(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 2,
            found: args.len(),
            function_name: Some("array.contains".to_string()),
            line: None,
        });
    }

    match &args[0] {
        Value::Array(arr) => {
            let target = &args[1];
            let found = arr.iter().any(|item| match (item, target) {
                (Value::Number(a), Value::Number(b)) => (a - b).abs() < f64::EPSILON,
                (Value::String(a), Value::String(b)) => a == b,
                (Value::Boolean(a), Value::Boolean(b)) => a == b,
                (Value::Null, Value::Null) => true,
                _ => false,
            });

            Ok(Value::Boolean(found))
        }
        _ => Err(InfraError::TypeError {
            expected: "array".to_string(),
            found: args[0].type_name().to_string(),
            context: Some("array.contains function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Get the first element of an array
pub fn first(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 1,
            found: args.len(),
            function_name: Some("array.first".to_string()),
            line: None,
        });
    }

    match &args[0] {
        Value::Array(arr) => {
            if arr.is_empty() {
                Ok(Value::Null)
            } else {
                Ok(arr[0].clone())
            }
        }
        _ => Err(InfraError::TypeError {
            expected: "array".to_string(),
            found: args[0].type_name().to_string(),
            context: Some("array.first function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Get the last element of an array
pub fn last(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 1,
            found: args.len(),
            function_name: Some("array.last".to_string()),
            line: None,
        });
    }

    match &args[0] {
        Value::Array(arr) => {
            if arr.is_empty() {
                Ok(Value::Null)
            } else {
                Ok(arr[arr.len() - 1].clone())
            }
        }
        _ => Err(InfraError::TypeError {
            expected: "array".to_string(),
            found: args[0].type_name().to_string(),
            context: Some("array.last function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}
