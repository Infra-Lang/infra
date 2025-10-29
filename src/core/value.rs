use crate::core::ast::Stmt;
use std::collections::HashMap;
use std::fmt;
use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
    Function {
        name: String,
        params: Vec<String>,
        param_types: Vec<Option<crate::core::ast::Type>>, // Parameter types
        return_type: Option<crate::core::ast::Type>,      // Return type
        body: Box<Stmt>,
    },
    Promise {
        value: Option<Box<Value>>,
        resolved: bool,
        rejected: bool,
        error: Option<String>,
    },
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Null, Value::Null) => true,
            (Value::Array(a), Value::Array(b)) => a == b,
            (Value::Object(a), Value::Object(b)) => a == b,
            (
                Value::Function {
                    name: n1,
                    params: p1,
                    ..
                },
                Value::Function {
                    name: n2,
                    params: p2,
                    ..
                },
            ) => {
                // Functions are equal if they have the same name and parameters
                // We don't compare bodies for simplicity
                n1 == n2 && p1 == p2
            }
            (Value::Promise { .. }, Value::Promise { .. }) => {
                // Promises are equal only if they are the same instance
                // For simplicity, we'll say they're never equal
                false
            }
            _ => false,
        }
    }
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Boolean(_) => "boolean",
            Value::Null => "null",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
            Value::Function { .. } => "function",
            Value::Promise { .. } => "promise",
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Null => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(arr) => !arr.is_empty(),
            Value::Object(obj) => !obj.is_empty(),
            Value::Function { .. } => true, // Functions are always truthy
            Value::Promise { resolved, .. } => *resolved, // Promises are truthy if resolved
        }
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Value::Boolean(_))
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Value::Array(_))
    }

    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Array(arr) => Some(arr),
            _ => None,
        }
    }

    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
        match self {
            Value::Array(arr) => Some(arr),
            _ => None,
        }
    }

    pub fn is_object(&self) -> bool {
        matches!(self, Value::Object(_))
    }

    pub fn as_object(&self) -> Option<&HashMap<String, Value>> {
        match self {
            Value::Object(obj) => Some(obj),
            _ => None,
        }
    }

    pub fn as_object_mut(&mut self) -> Option<&mut HashMap<String, Value>> {
        match self {
            Value::Object(obj) => Some(obj),
            _ => None,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, item) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            Value::Object(obj) => {
                write!(f, "{{")?;
                let mut first = true;
                for (key, value) in obj.iter() {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "\"{}\": {}", key, value)?;
                    first = false;
                }
                write!(f, "}}")
            }
            Value::Function { name, params, .. } => {
                write!(f, "<function {}({})>", name, params.join(", "))
            }
            Value::Promise {
                resolved,
                rejected,
                error,
                ..
            } => {
                if *rejected {
                    write!(
                        f,
                        "<Promise rejected: {}>",
                        error.as_deref().unwrap_or("unknown error")
                    )
                } else if *resolved {
                    write!(f, "<Promise resolved>")
                } else {
                    write!(f, "<Promise pending>")
                }
            }
        }
    }
}

impl Add for Value {
    type Output = Result<Value, crate::core::error::InfraError>;

    fn add(self, other: Value) -> Self::Output {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::String(a), b) => Ok(Value::String(format!("{}{}", a, b))),
            (a, Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (a, b) => Err(crate::core::error::InfraError::TypeError {
                expected: "number or string".to_string(),
                found: format!("{} + {}", a.type_name(), b.type_name()),
                context: Some("addition operation".to_string()),
                line: None,
                column: None,
                hint: None,
            }),
        }
    }
}

impl Sub for Value {
    type Output = Result<Value, crate::core::error::InfraError>;

    fn sub(self, other: Value) -> Self::Output {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
            (a, b) => Err(crate::core::error::InfraError::TypeError {
                expected: "number".to_string(),
                found: format!("{} - {}", a.type_name(), b.type_name()),
                context: Some("subtraction operation".to_string()),
                line: None,
                column: None,
                hint: None,
            }),
        }
    }
}

impl Mul for Value {
    type Output = Result<Value, crate::core::error::InfraError>;

    fn mul(self, other: Value) -> Self::Output {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
            (a, b) => Err(crate::core::error::InfraError::TypeError {
                expected: "number".to_string(),
                found: format!("{} * {}", a.type_name(), b.type_name()),
                context: Some("multiplication operation".to_string()),
                line: None,
                column: None,
                hint: None,
            }),
        }
    }
}

impl Div for Value {
    type Output = Result<Value, crate::core::error::InfraError>;

    fn div(self, other: Value) -> Self::Output {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => {
                if b == 0.0 {
                    Err(crate::core::error::InfraError::DivisionByZero {
                        line: None,
                        column: None,
                    })
                } else {
                    Ok(Value::Number(a / b))
                }
            }
            (a, b) => Err(crate::core::error::InfraError::TypeError {
                expected: "number".to_string(),
                found: format!("{} / {}", a.type_name(), b.type_name()),
                context: Some("division operation".to_string()),
                line: None,
                column: None,
                hint: None,
            }),
        }
    }
}

impl Rem for Value {
    type Output = Result<Value, crate::core::error::InfraError>;

    fn rem(self, other: Value) -> Self::Output {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => {
                if b == 0.0 {
                    Err(crate::core::error::InfraError::DivisionByZero {
                        line: None,
                        column: None,
                    })
                } else {
                    Ok(Value::Number(a % b))
                }
            }
            (a, b) => Err(crate::core::error::InfraError::TypeError {
                expected: "number".to_string(),
                found: format!("{} % {}", a.type_name(), b.type_name()),
                context: Some("modulo operation".to_string()),
                line: None,
                column: None,
                hint: None,
            }),
        }
    }
}
