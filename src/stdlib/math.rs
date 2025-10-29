use crate::core::{InfraError, Result, Value};

/// Square root function
pub fn sqrt(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 1,
            found: args.len(),
        });
    }

    match &args[0] {
        Value::Number(n) => {
            if *n < 0.0 {
                Err(InfraError::RuntimeError {
                    message: "Cannot take square root of negative number".to_string(),
                })
            } else {
                Ok(Value::Number(n.sqrt()))
            }
        }
        _ => Err(InfraError::TypeError {
            expected: "number".to_string(),
            found: args[0].type_name().to_string(),
            context: None,
        ,
            context: None,
        }),
    }
}

/// Absolute value function
pub fn abs(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 1,
            found: args.len(),
        });
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.abs())),
        _ => Err(InfraError::TypeError {
            expected: "number".to_string(),
            found: args[0].type_name().to_string(),
            context: None,
        ,
            context: None,
        }),
    }
}

/// Maximum of two numbers
pub fn max(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 2,
            found: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.max(*b))),
        _ => Err(InfraError::TypeError {
            expected: "two numbers".to_string(),
            found: format!("{} and {}", args[0].type_name(), args[1].type_name()),
        ,
            context: None,
        }),
    }
}

/// Minimum of two numbers
pub fn min(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 2,
            found: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.min(*b))),
        _ => Err(InfraError::TypeError {
            expected: "two numbers".to_string(),
            found: format!("{} and {}", args[0].type_name(), args[1].type_name()),
        ,
            context: None,
        }),
    }
}

/// Power function (base^exponent)
pub fn pow(args: &[Value]) -> Result<Value> {
    if args.len() != 2 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 2,
            found: args.len(),
        });
    }

    match (&args[0], &args[1]) {
        (Value::Number(base), Value::Number(exp)) => Ok(Value::Number(base.powf(*exp))),
        _ => Err(InfraError::TypeError {
            expected: "two numbers".to_string(),
            found: format!("{} and {}", args[0].type_name(), args[1].type_name()),
        ,
            context: None,
        }),
    }
}

/// Floor function
pub fn floor(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 1,
            found: args.len(),
        });
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.floor())),
        _ => Err(InfraError::TypeError {
            expected: "number".to_string(),
            found: args[0].type_name().to_string(),
            context: None,
        ,
            context: None,
        }),
    }
}

/// Ceiling function
pub fn ceil(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 1,
            found: args.len(),
        });
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.ceil())),
        _ => Err(InfraError::TypeError {
            expected: "number".to_string(),
            found: args[0].type_name().to_string(),
            context: None,
        ,
            context: None,
        }),
    }
}

/// Round function
pub fn round(args: &[Value]) -> Result<Value> {
    if args.len() != 1 {
        return Err(InfraError::ArgumentCountMismatch {
            expected: 1,
            found: args.len(),
        });
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(n.round())),
        _ => Err(InfraError::TypeError {
            expected: "number".to_string(),
            found: args[0].type_name().to_string(),
            context: None,
        ,
            context: None,
        }),
    }
}
