use std::collections::HashMap;
use crate::core::{Value, InfraError, Result};

#[derive(Debug, Clone)]
pub struct Environment {
    variables: HashMap<String, Value>,
    pub parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: Environment) -> Self {
        Self {
            variables: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<Value> {
        if let Some(value) = self.variables.get(name) {
            Ok(value.clone())
        } else if let Some(parent) = &self.parent {
            parent.get(name)
        } else {
            Err(InfraError::UndefinedVariable {
                name: name.to_string(),
            })
        }
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<()> {
        if self.variables.contains_key(name) {
            self.variables.insert(name.to_string(), value);
            Ok(())
        } else if let Some(parent) = &mut self.parent {
            parent.assign(name, value)
        } else {
            Err(InfraError::UndefinedVariable {
                name: name.to_string(),
            })
        }
    }

    pub fn contains(&self, name: &str) -> bool {
        self.variables.contains_key(name) || 
        self.parent.as_ref().map_or(false, |p| p.contains(name))
    }

    pub fn clear(&mut self) {
        self.variables.clear();
    }

    pub fn size(&self) -> usize {
        let parent_size = self.parent.as_ref().map_or(0, |p| p.size());
        self.variables.len() + parent_size
    }

    pub fn debug_vars(&self) -> Vec<String> {
        self.variables.keys().cloned().collect()
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
