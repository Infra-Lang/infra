use crate::core::{InfraError, Result, Value};

/// Square root function
#[allow(dead_code)]
pub fn sqrt(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 1,
            found: args.len(),
            function_name: Some("math_sqrt".to_string()),
            line: None,
        });
    }

    match &args[0] {
        Value::Number(n) => {
            if *n < 0.0 {
                Err(InfraError::RuntimeError {
                    message: "Cannot take square root of negative number".to_string(),
                    line: None,
                    column: None,
                    stack_trace: vec![],
                    source_code: None,
                })
            } else {
                Ok(Value::Number(n.sqrt()))
            }
        }
        _ => Err(InfraError::TypeError {
            expected: "number".to_string(),
            found: args[0].type_name().to_string(),
            context: Some("math_sqrt() function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Absolute value function
#[allow(dead_code)]
pub fn abs(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 1,
            found: args.len(),
            function_name: Some("math_abs".to_string()),
            line: None,
        });
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.abs())),
        _ => Err(InfraError::TypeError {
            expected: "number".to_string(),
            found: args[0].type_name().to_string(),
            context: Some("math_abs() function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Maximum of two numbers
#[allow(dead_code)]
pub fn max(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 2,
            found: args.len(),
            function_name: Some("math_max".to_string()),
            line: None,
        });
    }

    match (&args[0], &args[1]) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.max(*b))),
        _ => Err(InfraError::TypeError {
            expected: "two numbers".to_string(),
            found: format!("{} and {}", args[0].type_name(), args[1].type_name()),
            context: Some("math_max() function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Minimum of two numbers
#[allow(dead_code)]
pub fn min(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 2,
            found: args.len(),
            function_name: Some("math_min".to_string()),
            line: None,
        });
    }

    match (&args[0], &args[1]) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.min(*b))),
        _ => Err(InfraError::TypeError {
            expected: "two numbers".to_string(),
            found: format!("{} and {}", args[0].type_name(), args[1].type_name()),
            context: Some("math_min() function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Power function (base^exponent)
#[allow(dead_code)]
pub fn pow(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 2,
            found: args.len(),
            function_name: Some("math_pow".to_string()),
            line: None,
        });
    }

    match (&args[0], &args[1]) {
        (Value::Number(base), Value::Number(exp)) => Ok(Value::Number(base.powf(*exp))),
        _ => Err(InfraError::TypeError {
            expected: "two numbers".to_string(),
            found: format!("{} and {}", args[0].type_name(), args[1].type_name()),
            context: Some("math_pow() function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Floor function
#[allow(dead_code)]
pub fn floor(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 1,
            found: args.len(),
            function_name: Some("math_floor".to_string()),
            line: None,
        });
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.floor())),
        _ => Err(InfraError::TypeError {
            expected: "number".to_string(),
            found: args[0].type_name().to_string(),
            context: Some("math_floor() function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Ceiling function
#[allow(dead_code)]
pub fn ceil(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 1,
            found: args.len(),
            function_name: Some("math_ceil".to_string()),
            line: None,
        });
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.ceil())),
        _ => Err(InfraError::TypeError {
            expected: "number".to_string(),
            found: args[0].type_name().to_string(),
            context: Some("math_ceil() function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}

/// Round function
#[allow(dead_code)]
pub fn round(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 1,
            found: args.len(),
            function_name: Some("math_round".to_string()),
            line: None,
        });
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.round())),
        _ => Err(InfraError::TypeError {
            expected: "number".to_string(),
            found: args[0].type_name().to_string(),
            context: Some("math_round() function".to_string()),
            line: None,
            column: None,
            hint: None,
        }),
    }
}
