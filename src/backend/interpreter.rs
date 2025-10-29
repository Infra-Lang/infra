use crate::backend::{Environment, Evaluator, ModuleSystem};
use crate::core::{ast::*, InfraError, Result, Value};
use crate::stdlib::StandardLibrary;
use std::collections::HashMap;
use std::path::Path;

pub struct Interpreter {
    evaluator: Evaluator,
    module_system: ModuleSystem,
    current_file_path: Option<std::path::PathBuf>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            evaluator: Evaluator::new(),
            module_system: ModuleSystem::new(),
            current_file_path: None,
        }
    }

    pub fn with_environment(environment: Environment) -> Self {
        Self {
            evaluator: Evaluator::with_environment(environment),
            module_system: ModuleSystem::new(),
            current_file_path: None,
        }
    }

    pub fn execute(&mut self, program: &Program) -> Result<()> {
        for stmt in &program.statements {
            self.execute_statement(stmt)?;
        }
        Ok(())
    }

    pub fn execute_statement(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::Expression(expr) => {
                self.evaluator.evaluate_expression(expr)?;
                Ok(())
            }
            Stmt::Let {
                name,
                type_annotation,
                value,
            } => {
                let val = self.evaluator.evaluate_expression(value)?;

                // Perform type checking if type annotation is provided
                if let Some(expected_type) = type_annotation {
                    self.check_type_compatibility_detailed(
                        &val,
                        expected_type,
                        &format!("variable '{}'", name),
                    )?;
                    self.evaluator.define_variable_with_type(
                        name.clone(),
                        val,
                        Some(expected_type),
                    );
                } else {
                    // No type annotation - infer type from value
                    let inferred_type = self.evaluator.infer_value_type(&val);
                    self.evaluator.define_variable_with_type(
                        name.clone(),
                        val,
                        Some(inferred_type),
                    );
                }

                Ok(())
            }
            Stmt::Print(expr) => {
                let value = self.evaluator.evaluate_expression(expr)?;
                println!("{}", value);
                Ok(())
            }
            Stmt::Block(statements) => {
                // Create new scope
                let old_env = self.evaluator.get_environment().clone();
                let new_env = Environment::with_parent(old_env);
                self.evaluator = Evaluator::with_environment(new_env);

                let mut result = Ok(());
                for statement in statements {
                    if let Err(e) = self.execute_statement(statement) {
                        result = Err(e);
                        break;
                    }
                }

                // Restore parent scope
                if let Some(parent) = self.evaluator.get_environment().parent.as_ref() {
                    self.evaluator = Evaluator::with_environment((**parent).clone());
                }

                result
            }
            Stmt::If {
                condition,
                then_stmt,
                else_stmt,
            } => {
                let condition_value = self.evaluator.evaluate_expression(condition)?;

                if condition_value.is_truthy() {
                    self.execute_statement(then_stmt)?;
                } else if let Some(else_stmt) = else_stmt {
                    self.execute_statement(else_stmt)?;
                }
                Ok(())
            }
            Stmt::While { condition, body } => {
                loop {
                    let condition_value = self.evaluator.evaluate_expression(condition)?;
                    if !condition_value.is_truthy() {
                        break;
                    }
                    self.execute_statement(body)?;
                }
                Ok(())
            }
            Stmt::For {
                var,
                start,
                end,
                body,
            } => {
                let start_val = self.evaluator.evaluate_expression(start)?;
                let end_val = self.evaluator.evaluate_expression(end)?;

                let (start_num, end_num) = match (start_val, end_val) {
                    (crate::core::Value::Number(s), crate::core::Value::Number(e)) => {
                        (s as i64, e as i64)
                    }
                    _ => {
                        return Err(crate::core::InfraError::TypeError {
                            expected: "number".to_string(),
                            found: "non-number in range".to_string(),
                        })
                    }
                };

                // Save old variable value if it exists
                let old_var_value = self.evaluator.get_environment().get(var).ok();

                for i in start_num..end_num {
                    self.evaluator
                        .define_variable(var.clone(), crate::core::Value::Number(i as f64));
                    self.execute_statement(body)?;
                }

                // Restore old variable value or remove it
                if let Some(old_value) = old_var_value {
                    self.evaluator.define_variable(var.clone(), old_value);
                } else {
                    // Variable didn't exist before, so we could remove it
                    // But our current Environment doesn't support removal
                    // This is a limitation we could address in the future
                }

                Ok(())
            }
            Stmt::Return(value) => {
                let return_value = if let Some(expr) = value {
                    Some(self.evaluator.evaluate_expression(expr)?)
                } else {
                    None
                };
                Err(crate::core::InfraError::Return(return_value))
            }
            Stmt::Function {
                name,
                params,
                param_types,
                return_type,
                body,
                ..
            } => {
                let function_value = crate::core::Value::Function {
                    name: name.clone(),
                    params: params.clone(),
                    param_types: param_types.clone(),
                    return_type: return_type.clone(),
                    body: body.clone(),
                };
                self.evaluator.define_variable(name.clone(), function_value);
                Ok(())
            }
            Stmt::Assignment { target, value } => {
                // Delegate to the evaluator's assignment handling
                let temp_stmt = Stmt::Assignment {
                    target: target.clone(),
                    value: value.clone(),
                };
                self.evaluator.execute_function_body(&temp_stmt)
            }
            Stmt::Try {
                try_block,
                catch_var,
                catch_block,
            } => {
                // Execute the try block
                match self.execute_statement(try_block) {
                    Ok(_) => Ok(()), // Success, no error caught
                    Err(error) => {
                        // Check if this is an exception that can be caught
                        let can_catch = matches!(
                            error,
                            crate::core::InfraError::Exception { .. }
                                | crate::core::InfraError::RuntimeError { .. }
                                | crate::core::InfraError::TypeError { .. }
                                | crate::core::InfraError::DivisionByZero
                                | crate::core::InfraError::IndexOutOfBounds { .. }
                                | crate::core::InfraError::PropertyNotFound { .. }
                                | crate::core::InfraError::UndefinedVariable { .. }
                                | crate::core::InfraError::ArgumentCountMismatch { .. }
                        );

                        if can_catch {
                            // Store the error message in the catch variable
                            let error_message = error.to_string();
                            self.evaluator.define_variable(
                                catch_var.clone(),
                                crate::core::Value::String(error_message),
                            );
                            // Execute the catch block
                            self.execute_statement(catch_block)
                        } else {
                            // Some errors cannot be caught (like Return)
                            Err(error)
                        }
                    }
                }
            }
            Stmt::Import {
                module_path,
                items,
                alias,
            } => {
                // Handle module imports
                let current_dir = self
                    .current_file_path
                    .as_ref()
                    .and_then(|p| p.parent())
                    .unwrap_or_else(|| Path::new("."));

                let module = self.module_system.load_module(module_path, current_dir)?;

                match items {
                    ImportItems::All => {
                        // Import all exports directly into current scope
                        for (name, value) in &module.exports {
                            self.evaluator.define_variable(name.clone(), value.clone());
                        }
                    }
                    ImportItems::Named(import_items) => {
                        // Import specific named exports
                        for import_item in import_items {
                            if let Some(value) = module.exports.get(&import_item.name) {
                                let import_name =
                                    import_item.alias.as_ref().unwrap_or(&import_item.name);
                                self.evaluator
                                    .define_variable(import_name.clone(), value.clone());
                            } else {
                                return Err(crate::core::InfraError::RuntimeError {
                                    message: format!(
                                        "Export '{}' not found in module '{}'",
                                        import_item.name, module_path
                                    ),
                                });
                            }
                        }
                    }
                    ImportItems::Default(name) => {
                        // Import default export (usually the module object itself)
                        let import_name = alias.as_ref().unwrap_or(name);
                        if let Some(default_export) = module.exports.get("default") {
                            self.evaluator
                                .define_variable(import_name.clone(), default_export.clone());
                        } else {
                            // If no default export, create an object with all exports
                            let mut exports_obj = std::collections::HashMap::new();
                            for (export_name, export_value) in &module.exports {
                                exports_obj.insert(export_name.clone(), export_value.clone());
                            }
                            self.evaluator.define_variable(
                                import_name.clone(),
                                crate::core::Value::Object(exports_obj),
                            );
                        }
                    }
                }
                Ok(())
            }
            Stmt::Export { item: _ } => {
                // Export statements are handled during module loading
                // When this statement is executed in a regular context, it's a no-op
                Ok(())
            }
        }
    }

    // Type checking helper function
    fn check_type_compatibility(&self, value: &Value, expected_type: &Type) -> bool {
        match (value, expected_type) {
            (_, Type::Any) => true, // Any type accepts any value
            (Value::Number(_), Type::Number) => true,
            (Value::String(_), Type::String) => true,
            (Value::Boolean(_), Type::Boolean) => true,
            (Value::Array(arr), Type::Array(element_type)) => {
                // Check if all array elements match the expected element type
                arr.iter()
                    .all(|val| self.check_type_compatibility(val, element_type))
            }
            (Value::Object(obj), Type::Object(expected_props)) => {
                // Check if object has all required properties with correct types
                expected_props.iter().all(|(prop_name, prop_type)| {
                    obj.get(prop_name)
                        .map(|val| self.check_type_compatibility(val, prop_type))
                        .unwrap_or(false)
                })
            }
            (value, Type::Union(types)) => {
                // Value must match at least one type in the union
                types
                    .iter()
                    .any(|t| self.check_type_compatibility(value, t))
            }
            (_, Type::Never) => false, // Never type accepts no values
            // TODO: Add function type checking when we have function values
            _ => false,
        }
    }

    fn type_to_string(&self, type_annotation: &Type) -> String {
        match type_annotation {
            Type::Number => "number".to_string(),
            Type::String => "string".to_string(),
            Type::Boolean => "boolean".to_string(),
            Type::Array(element_type) => format!("[{}]", self.type_to_string(element_type)),
            Type::Object(props) => {
                let prop_strings: Vec<String> = props
                    .iter()
                    .map(|(name, t)| format!("{}: {}", name, self.type_to_string(t)))
                    .collect();
                format!("{{{}}}", prop_strings.join(", "))
            }
            Type::Function {
                params,
                return_type,
            } => {
                let param_strings: Vec<String> =
                    params.iter().map(|t| self.type_to_string(t)).collect();
                format!(
                    "({}) -> {}",
                    param_strings.join(", "),
                    self.type_to_string(return_type)
                )
            }
            Type::Union(types) => {
                let type_strings: Vec<String> =
                    types.iter().map(|t| self.type_to_string(t)).collect();
                type_strings.join(" | ")
            }
            Type::Any => "any".to_string(),
            Type::Never => "never".to_string(),
        }
    }

    pub fn get_environment(&self) -> &Environment {
        self.evaluator.get_environment()
    }

    pub fn reset(&mut self) {
        self.evaluator = Evaluator::new();
        self.module_system = ModuleSystem::new();
        self.current_file_path = None;
    }

    pub fn set_current_file(&mut self, file_path: std::path::PathBuf) {
        self.current_file_path = Some(file_path);
    }

    pub fn get_current_file(&self) -> Option<&std::path::PathBuf> {
        self.current_file_path.as_ref()
    }

    // Enhanced type checking with detailed error messages
    fn check_type_compatibility_detailed(
        &self,
        value: &Value,
        expected_type: &Type,
        context: &str,
    ) -> Result<()> {
        match (value, expected_type) {
            (_, Type::Any) => Ok(()), // Any type accepts any value
            (Value::Number(_), Type::Number) => Ok(()),
            (Value::String(_), Type::String) => Ok(()),
            (Value::Boolean(_), Type::Boolean) => Ok(()),
            (Value::Array(arr), Type::Array(element_type)) => {
                // Check each array element with detailed position information
                for (index, val) in arr.iter().enumerate() {
                    if !self.check_type_compatibility(val, element_type) {
                        return Err(crate::core::InfraError::TypeError {
                            expected: format!(
                                "{} to have array element at index {} of type {}",
                                context,
                                index,
                                self.type_to_string(element_type)
                            ),
                            found: format!("{} ({})", val.type_name(), val),
                        });
                    }
                }
                Ok(())
            }
            (Value::Object(obj), Type::Object(expected_props)) => {
                // Check each required property
                for (prop_name, prop_type) in expected_props {
                    match obj.get(prop_name) {
                        Some(val) => {
                            if !self.check_type_compatibility(val, prop_type) {
                                return Err(crate::core::InfraError::TypeError {
                                    expected: format!(
                                        "{} to have property '{}' of type {}",
                                        context,
                                        prop_name,
                                        self.type_to_string(prop_type)
                                    ),
                                    found: format!("{} ({})", val.type_name(), val),
                                });
                            }
                        }
                        None => {
                            return Err(crate::core::InfraError::TypeError {
                                expected: format!(
                                    "{} to have required property '{}'",
                                    context, prop_name
                                ),
                                found: "missing property".to_string(),
                            });
                        }
                    }
                }
                Ok(())
            }
            (value, Type::Union(types)) => {
                // Value must match at least one type in the union
                for union_type in types {
                    if self.check_type_compatibility(value, union_type) {
                        return Ok(());
                    }
                }
                let type_strings: Vec<String> =
                    types.iter().map(|t| self.type_to_string(t)).collect();
                Err(crate::core::InfraError::TypeError {
                    expected: format!("{} to be of type {}", context, type_strings.join(" | ")),
                    found: format!("{} ({})", value.type_name(), value),
                })
            }
            (_, Type::Never) => Err(crate::core::InfraError::TypeError {
                expected: format!("{} to be of type never (impossible)", context),
                found: format!("{} ({})", value.type_name(), value),
            }),
            _ => Err(crate::core::InfraError::TypeError {
                expected: format!(
                    "{} to be of type {}",
                    context,
                    self.type_to_string(expected_type)
                ),
                found: format!("{} ({})", value.type_name(), value),
            }),
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}
