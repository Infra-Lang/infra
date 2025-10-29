use crate::backend::Environment;
use crate::core::{ast::*, InfraError, Result, Value};
use crate::stdlib::StandardLibrary;

pub struct Evaluator {
    environment: Environment,
    stdlib: StandardLibrary,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
            stdlib: StandardLibrary::new(),
        }
    }

    pub fn with_environment(environment: Environment) -> Self {
        Self {
            environment,
            stdlib: StandardLibrary::new(),
        }
    }

    pub fn evaluate_expression(&mut self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::Literal(value) => Ok(value.clone()),
            Expr::Identifier(name) => self.environment.get(name),
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;
                self.apply_binary_operator(operator, &left_val, &right_val)
            }
            Expr::Unary { operator, operand } => {
                let operand_val = self.evaluate_expression(operand)?;
                self.apply_unary_operator(operator, &operand_val)
            }
            Expr::Call { callee, args } => {
                // Check if this is a module function call
                if let Expr::ModuleAccess { module, function } = callee.as_ref() {
                    // Handle module function call
                    return self.call_module_function(module, function, args);
                }

                let function = self.evaluate_expression(callee)?;

                match function {
                    Value::Function {
                        name,
                        params,
                        param_types,
                        return_type,
                        body,
                        ..
                    } => {
                        // Evaluate arguments
                        let mut arg_values = Vec::new();
                        for arg in args {
                            arg_values.push(self.evaluate_expression(arg)?);
                        }

                        // Check argument count
                        if arg_values.len() != params.len() {
                            return Err(InfraError::ArgumentCountMismatch {
                                expected: params.len(),
                                found: arg_values.len(),
                            });
                        }

                        // Check parameter types with enhanced error messages
                        for (i, (param_type, arg_value)) in
                            param_types.iter().zip(arg_values.iter()).enumerate()
                        {
                            if let Some(expected_type) = param_type {
                                if !self.check_type_compatibility(arg_value, expected_type) {
                                    return Err(InfraError::TypeError {
                                        expected: format!(
                                            "parameter '{}' to be of type {}",
                                            params[i],
                                            self.type_to_string(expected_type)
                                        ),
                                        found: format!("{} ({})", arg_value.type_name(), arg_value),
                                        context: Some(format!("function call to '{}'", name)),
                                    });
                                }
                            }
                        }

                        // Create new environment for function
                        let old_env = self.environment.clone();
                        let mut function_env = Environment::with_parent(old_env.clone());

                        // Bind parameters
                        for (param, arg_value) in params.iter().zip(arg_values.iter()) {
                            function_env.define(param.clone(), arg_value.clone());
                        }

                        // Bind the function itself for recursion
                        let recursive_func = Value::Function {
                            name: name.clone(),
                            params: params.clone(),
                            param_types: param_types.clone(),
                            return_type: return_type.clone(),
                            body: body.clone(),
                        };
                        function_env.define(name.clone(), recursive_func);

                        // Execute function body with new environment
                        let old_evaluator_env =
                            std::mem::replace(&mut self.environment, function_env);

                        let result = match self.execute_function_body(&body) {
                            Ok(()) => Ok(Value::Null), // Function completed without return
                            Err(InfraError::Return(Some(value))) => {
                                // Check return type with enhanced error message
                                if let Some(expected_return_type) = return_type {
                                    if !self.check_type_compatibility(&value, &expected_return_type)
                                    {
                                        return Err(InfraError::TypeError {
                                            expected: format!(
                                                "function '{}' to return type {}",
                                                name,
                                                self.type_to_string(&expected_return_type)
                                            ),
                                            found: format!("{} ({})", value.type_name(), value),
                                            context: Some(format!(
                                                "function '{}' return statement",
                                                name
                                            )),
                                        });
                                    }
                                }
                                Ok(value)
                            }
                            Err(InfraError::Return(None)) => Ok(Value::Null),
                            Err(e) => Err(e),
                        };

                        // Restore environment
                        self.environment = old_evaluator_env;

                        result
                    }
                    _ => Err(InfraError::TypeError {
                        expected: "function".to_string(),
                        found: function.type_name().to_string(),
                    }),
                }
            }
            Expr::Array(elements) => {
                let mut array_values = Vec::new();
                for element in elements {
                    array_values.push(self.evaluate_expression(element)?);
                }
                Ok(Value::Array(array_values))
            }
            Expr::Index { object, index } => {
                let obj_value = self.evaluate_expression(object)?;
                let index_value = self.evaluate_expression(index)?;

                match (&obj_value, &index_value) {
                    (Value::Array(arr), Value::Number(idx)) => {
                        let index = *idx as usize;
                        if index >= arr.len() {
                            Err(InfraError::IndexOutOfBounds {
                                index,
                                length: arr.len(),
                            })
                        } else {
                            Ok(arr[index].clone())
                        }
                    }
                    (Value::Array(_), _) => Err(InfraError::TypeError {
                        expected: "number".to_string(),
                        found: index_value.type_name().to_string(),
                    }),
                    _ => Err(InfraError::TypeError {
                        expected: "array".to_string(),
                        found: obj_value.type_name().to_string(),
                    }),
                }
            }
            Expr::Object(properties) => {
                let mut object = std::collections::HashMap::new();
                for (key, value_expr) in properties {
                    let value = self.evaluate_expression(value_expr)?;
                    object.insert(key.clone(), value);
                }
                Ok(Value::Object(object))
            }
            Expr::Property { object, property } => {
                let obj_value = self.evaluate_expression(object)?;

                match obj_value {
                    Value::Object(obj) => match obj.get(property) {
                        Some(value) => Ok(value.clone()),
                        None => Err(InfraError::PropertyNotFound {
                            property: property.clone(),
                        }),
                    },
                    _ => Err(InfraError::TypeError {
                        expected: "object".to_string(),
                        found: obj_value.type_name().to_string(),
                    }),
                }
            }
            Expr::ModuleAccess { module, function } => {
                // Module access should not be evaluated directly - it should only be used in function calls
                Err(InfraError::RuntimeError {
                    message: format!(
                        "Cannot access {}.{} directly - use as function call",
                        module, function
                    ),
                })
            }
        }
    }

    /// Call a function from a standard library module
    fn call_module_function(
        &mut self,
        module: &str,
        function: &str,
        args: &[Expr],
    ) -> Result<Value> {
        // Evaluate arguments
        let mut arg_values = Vec::new();
        for arg in args {
            arg_values.push(self.evaluate_expression(arg)?);
        }

        // Get the native function from stdlib
        if let Some(native_func) = self.stdlib.get_function(module, function) {
            native_func(&arg_values)
        } else {
            Err(InfraError::RuntimeError {
                message: format!("Unknown function {}.{}", module, function),
            })
        }
    }

    pub fn define_variable(&mut self, name: String, value: Value) {
        self.environment.define(name, value);
    }

    pub fn define_variable_with_type(
        &mut self,
        name: String,
        value: Value,
        type_annotation: Option<Type>,
    ) {
        self.environment
            .define_with_type(name, value, type_annotation);
    }

    pub fn get_variable_type(&self, name: &str) -> Result<Option<Type>> {
        self.environment.get_type(name)
    }

    pub fn get_environment(&self) -> &Environment {
        &self.environment
    }

    pub fn get_environment_mut(&mut self) -> &mut Environment {
        &mut self.environment
    }

    fn apply_binary_operator(&self, op: &BinaryOp, left: &Value, right: &Value) -> Result<Value> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => self.apply_numeric_binary_operator(op, *l, *r),
            (Value::String(l), Value::String(r)) => self.apply_string_binary_operator(op, l, r),
            (Value::Boolean(l), Value::Boolean(r)) => {
                self.apply_boolean_binary_operator(op, *l, *r)
            }
            (Value::String(s), Value::Number(n)) if matches!(op, BinaryOp::Add) => {
                Ok(Value::String(format!("{}{}", s, n)))
            }
            (Value::Number(n), Value::String(s)) if matches!(op, BinaryOp::Add) => {
                Ok(Value::String(format!("{}{}", n, s)))
            }
            _ => {
                // Handle logical operators for mixed types
                match op {
                    BinaryOp::And => Ok(Value::Boolean(left.is_truthy() && right.is_truthy())),
                    BinaryOp::Or => Ok(Value::Boolean(left.is_truthy() || right.is_truthy())),
                    BinaryOp::Equal => Ok(Value::Boolean(self.values_equal(left, right))),
                    BinaryOp::NotEqual => Ok(Value::Boolean(!self.values_equal(left, right))),
                    _ => Err(InfraError::TypeError {
                        expected: "compatible types".to_string(),
                        found: format!("{} and {}", left.type_name(), right.type_name()),
                    }),
                }
            }
        }
    }

    fn apply_numeric_binary_operator(&self, op: &BinaryOp, left: f64, right: f64) -> Result<Value> {
        match op {
            BinaryOp::Add => Ok(Value::Number(left + right)),
            BinaryOp::Subtract => Ok(Value::Number(left - right)),
            BinaryOp::Multiply => Ok(Value::Number(left * right)),
            BinaryOp::Divide => {
                if right == 0.0 {
                    Err(InfraError::DivisionByZero)
                } else {
                    Ok(Value::Number(left / right))
                }
            }
            BinaryOp::Modulo => Ok(Value::Number(left % right)),
            BinaryOp::Equal => Ok(Value::Boolean((left - right).abs() < f64::EPSILON)),
            BinaryOp::NotEqual => Ok(Value::Boolean((left - right).abs() >= f64::EPSILON)),
            BinaryOp::Less => Ok(Value::Boolean(left < right)),
            BinaryOp::LessEqual => Ok(Value::Boolean(left <= right)),
            BinaryOp::Greater => Ok(Value::Boolean(left > right)),
            BinaryOp::GreaterEqual => Ok(Value::Boolean(left >= right)),
            BinaryOp::And => Ok(Value::Boolean(left != 0.0 && right != 0.0)),
            BinaryOp::Or => Ok(Value::Boolean(left != 0.0 || right != 0.0)),
        }
    }

    fn apply_string_binary_operator(
        &self,
        op: &BinaryOp,
        left: &str,
        right: &str,
    ) -> Result<Value> {
        match op {
            BinaryOp::Add => Ok(Value::String(format!("{}{}", left, right))),
            BinaryOp::Equal => Ok(Value::Boolean(left == right)),
            BinaryOp::NotEqual => Ok(Value::Boolean(left != right)),
            BinaryOp::Less => Ok(Value::Boolean(left < right)),
            BinaryOp::LessEqual => Ok(Value::Boolean(left <= right)),
            BinaryOp::Greater => Ok(Value::Boolean(left > right)),
            BinaryOp::GreaterEqual => Ok(Value::Boolean(left >= right)),
            _ => Err(InfraError::TypeError {
                expected: "numeric operation".to_string(),
                found: "string".to_string(),
            }),
        }
    }

    fn apply_boolean_binary_operator(
        &self,
        op: &BinaryOp,
        left: bool,
        right: bool,
    ) -> Result<Value> {
        match op {
            BinaryOp::Equal => Ok(Value::Boolean(left == right)),
            BinaryOp::NotEqual => Ok(Value::Boolean(left != right)),
            BinaryOp::And => Ok(Value::Boolean(left && right)),
            BinaryOp::Or => Ok(Value::Boolean(left || right)),
            _ => Err(InfraError::TypeError {
                expected: "logical operation".to_string(),
                found: "boolean".to_string(),
            }),
        }
    }

    fn apply_unary_operator(&self, op: &UnaryOp, operand: &Value) -> Result<Value> {
        match (op, operand) {
            (UnaryOp::Minus, Value::Number(n)) => Ok(Value::Number(-n)),
            (UnaryOp::Not, value) => Ok(Value::Boolean(!value.is_truthy())),
            (UnaryOp::Minus, _) => Err(InfraError::TypeError {
                expected: "number".to_string(),
                found: operand.type_name().to_string(),
            }),
        }
    }

    fn values_equal(&self, left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => (l - r).abs() < f64::EPSILON,
            (Value::String(l), Value::String(r)) => l == r,
            (Value::Boolean(l), Value::Boolean(r)) => l == r,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }

    pub fn execute_function_body(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::Expression(expr) => {
                self.evaluate_expression(expr)?;
                Ok(())
            }
            Stmt::Let { name, value, .. } => {
                let val = self.evaluate_expression(value)?;
                self.environment.define(name.clone(), val);
                Ok(())
            }
            Stmt::Print(expr) => {
                let value = self.evaluate_expression(expr)?;
                println!("{}", value);
                Ok(())
            }
            Stmt::Block(statements) => {
                // Create new scope
                let old_env = self.environment.clone();
                let new_env = Environment::with_parent(old_env.clone());
                self.environment = new_env;

                let mut result = Ok(());
                for statement in statements {
                    if let Err(e) = self.execute_function_body(statement) {
                        result = Err(e);
                        break;
                    }
                }

                // Restore original scope
                self.environment = old_env;

                result
            }
            Stmt::If {
                condition,
                then_stmt,
                else_stmt,
            } => {
                let condition_value = self.evaluate_expression(condition)?;

                if condition_value.is_truthy() {
                    self.execute_function_body(then_stmt)?;
                } else if let Some(else_stmt) = else_stmt {
                    self.execute_function_body(else_stmt)?;
                }
                Ok(())
            }
            Stmt::While { condition, body } => {
                loop {
                    let condition_value = self.evaluate_expression(condition)?;
                    if !condition_value.is_truthy() {
                        break;
                    }
                    self.execute_function_body(body)?;
                }
                Ok(())
            }
            Stmt::For {
                var,
                start,
                end,
                body,
            } => {
                let start_val = self.evaluate_expression(start)?;
                let end_val = self.evaluate_expression(end)?;

                let (start_num, end_num) = match (start_val, end_val) {
                    (Value::Number(s), Value::Number(e)) => (s as i64, e as i64),
                    _ => {
                        return Err(InfraError::TypeError {
                            expected: "number".to_string(),
                            found: "non-number in range".to_string(),
                        })
                    }
                };

                // Save old variable value if it exists
                let old_var_value = self.environment.get(var).ok();

                for i in start_num..end_num {
                    self.environment
                        .define(var.clone(), Value::Number(i as f64));
                    self.execute_function_body(body)?;
                }

                // Restore old variable value or remove it
                if let Some(old_value) = old_var_value {
                    self.environment.define(var.clone(), old_value);
                }

                Ok(())
            }
            Stmt::Return(value) => {
                let return_value = if let Some(expr) = value {
                    Some(self.evaluate_expression(expr)?)
                } else {
                    None
                };
                Err(InfraError::Return(return_value))
            }
            Stmt::Function {
                name,
                params,
                param_types,
                return_type,
                body,
                ..
            } => {
                let function_value = Value::Function {
                    name: name.clone(),
                    params: params.clone(),
                    param_types: param_types.clone(),
                    return_type: return_type.clone(),
                    body: body.clone(),
                };
                self.environment.define(name.clone(), function_value);
                Ok(())
            }
            Stmt::Assignment { target, value } => {
                let new_value = self.evaluate_expression(value)?;

                match target {
                    AssignmentTarget::Identifier(name) => {
                        if self.environment.get(name).is_err() {
                            return Err(InfraError::UndefinedVariable { name: name.clone() });
                        }

                        // Check type compatibility for assignment
                        if let Ok(stored_type) = self.environment.get_type(name) {
                            if let Some(expected_type) = stored_type {
                                if !self.check_type_compatibility(&new_value, &expected_type) {
                                    return Err(InfraError::TypeError {
                                        expected: format!(
                                            "variable '{}' to be of type {}",
                                            name,
                                            self.type_to_string(&expected_type)
                                        ),
                                        found: format!("{} ({})", new_value.type_name(), new_value),
                                        context: Some(format!("assignment to variable '{}'", name)),
                                    });
                                }
                            }
                        }

                        self.environment.define(name.clone(), new_value);
                        Ok(())
                    }
                    AssignmentTarget::Property { object, property } => {
                        let obj_val = self.evaluate_expression(object)?;
                        match obj_val {
                            Value::Object(mut map) => {
                                map.insert(property.clone(), new_value);
                                let updated_obj = Value::Object(map);

                                // We need to update the object in the environment
                                // This is tricky because we need to find where the object is stored
                                if let Expr::Identifier(obj_name) = object.as_ref() {
                                    self.environment.define(obj_name.clone(), updated_obj);
                                    Ok(())
                                } else {
                                    Err(InfraError::RuntimeError {
                                        message: "Cannot assign to property of complex expression"
                                            .to_string(),
                                    })
                                }
                            }
                            _ => Err(InfraError::TypeError {
                                expected: "object".to_string(),
                                found: obj_val.type_name().to_string(),
                            }),
                        }
                    }
                    AssignmentTarget::Index { object, index } => {
                        let obj_val = self.evaluate_expression(object)?;
                        let index_val = self.evaluate_expression(index)?;

                        match (obj_val, index_val) {
                            (Value::Array(mut arr), Value::Number(idx)) => {
                                let index = idx as usize;
                                if index >= arr.len() {
                                    return Err(InfraError::IndexOutOfBounds {
                                        index,
                                        length: arr.len(),
                                    });
                                }
                                arr[index] = new_value;
                                let updated_arr = Value::Array(arr);

                                // Update array in environment
                                if let Expr::Identifier(arr_name) = object.as_ref() {
                                    self.environment.define(arr_name.clone(), updated_arr);
                                    Ok(())
                                } else {
                                    Err(InfraError::RuntimeError {
                                        message: "Cannot assign to index of complex expression"
                                            .to_string(),
                                    })
                                }
                            }
                            (Value::Array(_), _) => Err(InfraError::TypeError {
                                expected: "number".to_string(),
                                found: "non-number index".to_string(),
                            }),
                            (_, _) => Err(InfraError::TypeError {
                                expected: "array".to_string(),
                                found: "non-array for indexing".to_string(),
                            }),
                        }
                    }
                }
            }
            Stmt::Try { .. } => {
                // Try statements should be handled by the interpreter, not the evaluator
                Err(InfraError::RuntimeError {
                    message: "Try/catch statements should be handled by interpreter".to_string(),
                })
            }
            Stmt::Import { .. } => {
                // Import statements should be handled by the interpreter, not the evaluator
                Err(InfraError::RuntimeError {
                    message: "Import statements should be handled by interpreter".to_string(),
                })
            }
            Stmt::Export { .. } => {
                // Export statements should be handled by the interpreter, not the evaluator
                Err(InfraError::RuntimeError {
                    message: "Export statements should be handled by interpreter".to_string(),
                })
            }
        }
    }

    // Enhanced type checking for function parameters and returns
    fn check_function_parameter_types(
        &self,
        param_types: &[Option<Type>],
        arg_values: &[Value],
        param_names: &[String],
    ) -> Result<()> {
        for (i, (param_type, arg_value)) in param_types.iter().zip(arg_values.iter()).enumerate() {
            if let Some(expected_type) = param_type {
                if !self.check_type_compatibility(arg_value, expected_type) {
                    return Err(InfraError::TypeError {
                        expected: format!(
                            "parameter '{}' of type {}",
                            param_names[i],
                            self.type_to_string(expected_type)
                        ),
                        found: format!(
                            "{} (actual value: {})",
                            arg_value.type_name(),
                            self.format_value_for_error(arg_value)
                        ),
                    });
                }
            }
        }
        Ok(())
    }

    fn check_function_return_type(
        &self,
        return_value: &Value,
        expected_return_type: &Option<Type>,
        function_name: &str,
    ) -> Result<()> {
        if let Some(expected_type) = expected_return_type {
            if !self.check_type_compatibility(return_value, expected_type) {
                return Err(InfraError::TypeError {
                    expected: format!(
                        "function '{}' to return {}",
                        function_name,
                        self.type_to_string(expected_type)
                    ),
                    found: format!(
                        "{} (actual value: {})",
                        return_value.type_name(),
                        self.format_value_for_error(return_value)
                    ),
                });
            }
        }
        Ok(())
    }

    fn format_value_for_error(&self, value: &Value) -> String {
        match value {
            Value::String(s) => format!("\"{}\"", s),
            Value::Number(n) => n.to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Array(arr) => {
                if arr.len() <= 3 {
                    format!(
                        "[{}]",
                        arr.iter()
                            .map(|v| self.format_value_for_error(v))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                } else {
                    format!(
                        "[{}, ... {} more items]",
                        arr.iter()
                            .take(2)
                            .map(|v| self.format_value_for_error(v))
                            .collect::<Vec<_>>()
                            .join(", "),
                        arr.len() - 2
                    )
                }
            }
            Value::Object(obj) => {
                if obj.len() <= 2 {
                    format!(
                        "{{{}}}",
                        obj.iter()
                            .map(|(k, v)| format!("{}: {}", k, self.format_value_for_error(v)))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                } else {
                    format!("{{... {} properties}}", obj.len())
                }
            }
            _ => format!("{}", value),
        }
    }

    fn type_to_string(&self, type_annotation: &Type) -> String {
        match type_annotation {
            Type::Number => "number".to_string(),
            Type::String => "string".to_string(),
            Type::Boolean => "boolean".to_string(),
            Type::Any => "any".to_string(),
            Type::Array(element_type) => format!("[{}]", self.type_to_string(element_type)),
            Type::Object(fields) => {
                let field_strings: Vec<String> = fields
                    .iter()
                    .map(|(name, field_type)| {
                        format!("{}: {}", name, self.type_to_string(field_type))
                    })
                    .collect();
                format!("{{{}}}", field_strings.join(", "))
            }
            Type::Union(types) => types
                .iter()
                .map(|t| self.type_to_string(t))
                .collect::<Vec<_>>()
                .join(" | "),
            Type::Function {
                params,
                return_type,
            } => {
                let param_strings: Vec<String> =
                    params.iter().map(|p| self.type_to_string(p)).collect();
                format!(
                    "({}) -> {}",
                    param_strings.join(", "),
                    self.type_to_string(return_type)
                )
            }
            Type::Never => "never".to_string(),
        }
    }

    // Enhanced type inference
    fn infer_value_type(&self, value: &Value) -> Type {
        match value {
            Value::Number(_) => Type::Number,
            Value::String(_) => Type::String,
            Value::Boolean(_) => Type::Boolean,
            Value::Null => Type::Any, // Use Any for null values
            Value::Array(arr) => {
                if arr.is_empty() {
                    // For empty arrays, we can't infer the element type
                    Type::Array(Box::new(Type::Union(vec![
                        Type::Number,
                        Type::String,
                        Type::Boolean,
                    ])))
                } else {
                    // Infer from first element (could be enhanced to check all elements)
                    let element_type = self.infer_value_type(&arr[0]);
                    Type::Array(Box::new(element_type))
                }
            }
            Value::Object(obj) => {
                let mut fields = Vec::new();
                for (key, value) in obj {
                    fields.push((key.clone(), self.infer_value_type(value)));
                }
                Type::Object(fields)
            }
            Value::Function {
                param_types,
                return_type,
                ..
            } => {
                let params: Vec<Type> = param_types
                    .iter()
                    .map(|opt_type| {
                        opt_type.clone().unwrap_or(Type::Union(vec![
                            Type::Number,
                            Type::String,
                            Type::Boolean,
                        ]))
                    })
                    .collect();
                let ret_type = return_type.clone().unwrap_or(Type::Any);
                Type::Function {
                    params,
                    return_type: Box::new(ret_type),
                }
            }
        }
    }

    // Enhanced type compatibility checking with better union type support
    fn check_type_compatibility(&self, value: &Value, expected_type: &Type) -> bool {
        let value_type = self.infer_value_type(value);
        self.types_compatible(&value_type, expected_type)
    }

    fn types_compatible(&self, actual: &Type, expected: &Type) -> bool {
        match (actual, expected) {
            // Exact matches
            (Type::Number, Type::Number) => true,
            (Type::String, Type::String) => true,
            (Type::Boolean, Type::Boolean) => true,
            (Type::Any, _) | (_, Type::Any) => true, // Any is compatible with everything

            // Array compatibility
            (Type::Array(actual_elem), Type::Array(expected_elem)) => {
                self.types_compatible(actual_elem, expected_elem)
            }

            // Object compatibility (structural typing)
            (Type::Object(actual_fields), Type::Object(expected_fields)) => {
                // Check that all expected fields are present and compatible
                for (expected_key, expected_field_type) in expected_fields {
                    if let Some((_, actual_field_type)) =
                        actual_fields.iter().find(|(key, _)| key == expected_key)
                    {
                        if !self.types_compatible(actual_field_type, expected_field_type) {
                            return false;
                        }
                    } else {
                        return false; // Missing required field
                    }
                }
                true
            }

            // Union type compatibility
            (actual_type, Type::Union(union_types)) => {
                // Check if the actual type is compatible with any type in the union
                union_types
                    .iter()
                    .any(|union_type| self.types_compatible(actual_type, union_type))
            }

            (Type::Union(actual_types), expected_type) => {
                // All types in the union must be compatible with expected
                actual_types
                    .iter()
                    .all(|actual_type| self.types_compatible(actual_type, expected_type))
            }

            // Function compatibility
            (
                Type::Function {
                    params: actual_params,
                    return_type: actual_return,
                },
                Type::Function {
                    params: expected_params,
                    return_type: expected_return,
                },
            ) => {
                // Check parameter compatibility (contravariant)
                if actual_params.len() != expected_params.len() {
                    return false;
                }
                for (actual_param, expected_param) in
                    actual_params.iter().zip(expected_params.iter())
                {
                    if !self.types_compatible(expected_param, actual_param) {
                        // Note: reversed for contravariance
                        return false;
                    }
                }
                // Check return type compatibility (covariant)
                self.types_compatible(actual_return, expected_return)
            }

            _ => false,
        }
    }

    /// Infer the type of an expression
    pub fn infer_expression_type(&self, expr: &Expr) -> Type {
        match expr {
            Expr::Literal(value) => self.value_to_type(value),
            Expr::Identifier(name) => {
                // Look up variable type in environment
                if let Ok(value) = self.environment.get(name) {
                    self.value_to_type(&value)
                } else {
                    Type::Any
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_type = self.infer_expression_type(left);
                let right_type = self.infer_expression_type(right);
                self.infer_binary_operation_type(operator, &left_type, &right_type)
            }
            Expr::Unary { operator, operand } => {
                let operand_type = self.infer_expression_type(operand);
                self.infer_unary_operation_type(operator, &operand_type)
            }
            Expr::Call { callee, args } => {
                // Try to infer return type from function signature
                if let Expr::Identifier(func_name) = callee.as_ref() {
                    if let Ok(Value::Function { return_type, .. }) = self.environment.get(func_name)
                    {
                        return return_type.unwrap_or(Type::Any);
                    }
                }
                Type::Any // Default to Any if we can't determine
            }
            Expr::Array(elements) => {
                if elements.is_empty() {
                    Type::Array(Box::new(Type::Any))
                } else {
                    // Infer common type of all elements
                    let element_types: Vec<Type> = elements
                        .iter()
                        .map(|e| self.infer_expression_type(e))
                        .collect();
                    let common_type = self.find_common_type(&element_types);
                    Type::Array(Box::new(common_type))
                }
            }
            Expr::Object(fields) => {
                let typed_fields: Vec<(String, Type)> = fields
                    .iter()
                    .map(|(key, value_expr)| (key.clone(), self.infer_expression_type(value_expr)))
                    .collect();
                Type::Object(typed_fields)
            }
            Expr::Property { object, .. } => {
                // Try to infer property type from object type
                Type::Any // Simplified for now
            }
            Expr::Index { object, .. } => {
                let object_type = self.infer_expression_type(object);
                match object_type {
                    Type::Array(element_type) => *element_type,
                    Type::Object(_) => Type::Any, // Could be any property type
                    _ => Type::Any,
                }
            }
            Expr::ModuleAccess { .. } => Type::Any, // Module functions return Any for now
        }
    }

    /// Find a common type for a collection of types (for type inference)
    fn find_common_type(&self, types: &[Type]) -> Type {
        if types.is_empty() {
            return Type::Any;
        }

        let first_type = &types[0];

        // If all types are the same, return that type
        if types.iter().all(|t| t == first_type) {
            return first_type.clone();
        }

        // If types are different, create a union type
        let unique_types: Vec<Type> = types.iter().cloned().collect();
        if unique_types.len() == 1 {
            unique_types[0].clone()
        } else {
            Type::Union(unique_types)
        }
    }

    /// Infer the result type of a binary operation
    fn infer_binary_operation_type(&self, operator: &BinaryOp, left: &Type, right: &Type) -> Type {
        use BinaryOp::*;

        match operator {
            Add | Subtract | Multiply | Divide | Modulo => {
                // Arithmetic operations
                match (left, right) {
                    (Type::Number, Type::Number) => Type::Number,
                    (Type::String, Type::String) if matches!(operator, Add) => Type::String,
                    _ => Type::Any, // Could be invalid, but we'll let runtime handle it
                }
            }
            Equal | NotEqual | Less | Greater | LessEqual | GreaterEqual => {
                // Comparison operations always return boolean
                Type::Boolean
            }
            And | Or => {
                // Logical operations return boolean
                Type::Boolean
            }
        }
    }

    /// Infer the result type of a unary operation
    fn infer_unary_operation_type(&self, operator: &UnaryOp, operand: &Type) -> Type {
        use UnaryOp::*;

        match operator {
            Minus => match operand {
                Type::Number => Type::Number,
                _ => Type::Any,
            },
            Not => Type::Boolean, // Logical not always returns boolean
        }
    }

    /// Convert a runtime value to its corresponding type
    fn value_to_type(&self, value: &Value) -> Type {
        match value {
            Value::Number(_) => Type::Number,
            Value::String(_) => Type::String,
            Value::Boolean(_) => Type::Boolean,
            Value::Array(elements) => {
                if elements.is_empty() {
                    Type::Array(Box::new(Type::Any))
                } else {
                    let element_types: Vec<Type> =
                        elements.iter().map(|v| self.value_to_type(v)).collect();
                    let common_type = self.find_common_type(&element_types);
                    Type::Array(Box::new(common_type))
                }
            }
            Value::Object(map) => {
                let typed_fields: Vec<(String, Type)> = map
                    .iter()
                    .map(|(key, value)| (key.clone(), self.value_to_type(value)))
                    .collect();
                Type::Object(typed_fields)
            }
            Value::Function {
                param_types,
                return_type,
                ..
            } => Type::Function {
                params: param_types
                    .iter()
                    .map(|opt_type| opt_type.clone().unwrap_or(Type::Any))
                    .collect(),
                return_type: Box::new(return_type.clone().unwrap_or(Type::Any)),
            },
            Value::Null => Type::Any, // Null can be any type
        }
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}
