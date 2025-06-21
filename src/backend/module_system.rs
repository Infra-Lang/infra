use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::core::{Result, InfraError, Value};
use crate::frontend::{Lexer, Parser};
use crate::core::ast::{Program, Stmt, ExportItem};

/// Represents a loaded module with its exports
#[derive(Debug, Clone)]
pub struct Module {
    pub path: PathBuf,
    pub exports: HashMap<String, Value>,
}

/// Module loader and cache
pub struct ModuleSystem {
    loaded_modules: HashMap<String, Module>,
    search_paths: Vec<PathBuf>,
}

impl ModuleSystem {
    pub fn new() -> Self {
        Self {
            loaded_modules: HashMap::new(),
            search_paths: vec![
                PathBuf::from("."),
                PathBuf::from("./lib"),
                PathBuf::from("./modules"),
            ],
        }
    }

    pub fn add_search_path(&mut self, path: PathBuf) {
        self.search_paths.push(path);
    }

    /// Load a module from a file path
    pub fn load_module(&mut self, module_path: &str, current_dir: &Path) -> Result<Module> {
        // Check if module is already loaded
        if let Some(module) = self.loaded_modules.get(module_path) {
            return Ok(module.clone());
        }

        // Resolve the module path
        let resolved_path = self.resolve_module_path(module_path, current_dir)?;
        
        // Read the module file
        let source = std::fs::read_to_string(&resolved_path)
            .map_err(|_| InfraError::RuntimeError {
                message: format!("Could not read module file: {}", resolved_path.display()),
            })?;

        // Parse the module
        let mut lexer = Lexer::new(&source);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let program = parser.parse()?;

        // Extract exports from the parsed program
        let exports = self.extract_exports_from_program(&program)?;

        let module = Module {
            path: resolved_path.clone(),
            exports,
        };

        // Cache the module
        self.loaded_modules.insert(module_path.to_string(), module.clone());

        Ok(module)
    }

    /// Resolve a module path to an actual file path
    fn resolve_module_path(&self, module_path: &str, current_dir: &Path) -> Result<PathBuf> {
        // If it's a relative path starting with './', resolve relative to current file
        if module_path.starts_with("./") || module_path.starts_with("../") {
            let path = current_dir.join(module_path);
            if path.exists() {
                return Ok(path);
            }
            // Try with .infra extension
            let path_with_ext = path.with_extension("infra");
            if path_with_ext.exists() {
                return Ok(path_with_ext);
            }
        }

        // Try each search path
        for search_path in &self.search_paths {
            let path = search_path.join(module_path);
            if path.exists() {
                return Ok(path);
            }
            // Try with .infra extension
            let path_with_ext = path.with_extension("infra");
            if path_with_ext.exists() {
                return Ok(path_with_ext);
            }
        }

        Err(InfraError::RuntimeError {
            message: format!("Module not found: {}", module_path),
        })
    }

    /// Extract exports from a program without full execution
    fn extract_exports_from_program(&self, program: &Program) -> Result<HashMap<String, Value>> {
        let mut exports = HashMap::new();
        
        for stmt in &program.statements {
            match stmt {
                Stmt::Export { item } => {
                    match item {
                        ExportItem::Function { name, params, param_types, return_type, body, .. } => {
                            // Create a proper function value
                            let function_value = Value::Function {
                                name: name.clone(),
                                params: params.clone(),
                                param_types: param_types.clone(),
                                return_type: return_type.clone(),
                                body: body.clone(),
                            };
                            exports.insert(name.clone(), function_value);
                        }
                        ExportItem::Variable { name, value, .. } => {
                            // For exported variables, we need to evaluate them
                            // For now, we'll create a temporary evaluator to evaluate the expression
                            let temp_env = crate::backend::environment::Environment::new();
                            let mut temp_evaluator = crate::backend::evaluator::Evaluator::with_environment(temp_env);
                            
                            match temp_evaluator.evaluate_expression(value) {
                                Ok(val) => {
                                    exports.insert(name.clone(), val);
                                }
                                Err(_) => {
                                    // If evaluation fails, store as null for now
                                    exports.insert(name.clone(), Value::Null);
                                }
                            }
                        }
                    }
                }
                _ => {
                    // Non-export statements are ignored during module loading
                }
            }
        }

        // If no exports found, create a default export
        if exports.is_empty() {
            exports.insert("default".to_string(), Value::String("module".to_string()));
        }

        Ok(exports)
    }

    /// Get a value from a loaded module
    pub fn get_module_export(&self, module_path: &str, export_name: &str) -> Option<Value> {
        self.loaded_modules
            .get(module_path)
            .and_then(|module| module.exports.get(export_name))
            .cloned()
    }

    /// List all exports from a module
    pub fn list_module_exports(&self, module_path: &str) -> Option<Vec<String>> {
        self.loaded_modules
            .get(module_path)
            .map(|module| module.exports.keys().cloned().collect())
    }
}

impl Default for ModuleSystem {
    fn default() -> Self {
        Self::new()
    }
}
