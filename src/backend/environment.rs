use crate::core::{InfraError, Result, Type, Value};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    variables: HashMap<String, Value>,
    types: HashMap<String, Option<Type>>, // Store type annotations and inferred types
    pub parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            types: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: Environment) -> Self {
        Self {
            variables: HashMap::new(),
            types: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.variables.insert(name.clone(), value);
        // Infer and store type for untyped variables
        self.types.entry(name).or_insert(None); // None means no explicit type annotation
    }

    pub fn define_with_type(&mut self, name: String, value: Value, type_annotation: Option<Type>) {
        self.variables.insert(name.clone(), value);
        self.types.insert(name, type_annotation);
    }

    pub fn get_type(&self, name: &str) -> Result<Option<Type>> {
        if let Some(t) = self.types.get(name) {
            Ok(t.clone())
        } else if let Some(parent) = &self.parent {
            parent.get_type(name)
        } else {
            Ok(None)
        }
    }

    pub fn get(&self, name: &str) -> Result<Value> {
        if let Some(value) = self.variables.get(name) {
            Ok(value.clone())
        } else if let Some(parent) = &self.parent {
            parent.get(name)
        } else {
            Err(InfraError::UndefinedVariable {
                name: name.to_string(),
                line: None,
                column: None,
                suggestion: None,
            })
        }
    }

    #[allow(dead_code)]
    pub fn assign(&mut self, name: &str, value: Value) -> Result<()> {
        if self.variables.contains_key(name) {
            self.variables.insert(name.to_string(), value);
            Ok(())
        } else if let Some(parent) = &mut self.parent {
            parent.assign(name, value)
        } else {
            Err(InfraError::UndefinedVariable {
                name: name.to_string(),
                line: None,
                column: None,
                suggestion: None,
            })
        }
    }

    #[allow(dead_code)]
    pub fn assign_with_type_check(
        &mut self,
        name: &str,
        value: Value,
        check_fn: &dyn Fn(&Value, Option<&Type>) -> Result<()>,
    ) -> Result<()> {
        if self.variables.contains_key(name) {
            // Get stored type for type checking
            let stored_type = self.types.get(name).cloned();

            // Perform type checking if needed
            check_fn(&value, stored_type.as_ref().and_then(|t| t.as_ref()))?;

            self.variables.insert(name.to_string(), value);
            Ok(())
        } else if let Some(parent) = &mut self.parent {
            parent.assign_with_type_check(name, value, check_fn)
        } else {
            Err(InfraError::UndefinedVariable {
                name: name.to_string(),
                line: None,
                column: None,
                suggestion: None,
            })
        }
    }

    #[allow(dead_code)]
    pub fn contains(&self, name: &str) -> bool {
        self.variables.contains_key(name) || self.parent.as_ref().is_some_and(|p| p.contains(name))
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.variables.clear();
    }

    pub fn size(&self) -> usize {
        let parent_size = self.parent.as_ref().map_or(0, |p| p.size());
        self.variables.len() + parent_size
    }

    #[allow(dead_code)]
    pub fn debug_vars(&self) -> Vec<String> {
        self.variables.keys().cloned().collect()
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
