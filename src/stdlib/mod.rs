pub mod array;
pub mod async_mod;
pub mod io;
pub mod math;
pub mod string;

use crate::core::{Result, Value};
use std::collections::HashMap;

/// Standard library module that provides built-in functions
pub struct StandardLibrary {
    modules: HashMap<String, HashMap<String, NativeFunction>>,
}

/// Native function type
pub type NativeFunction = fn(&[Value]) -> Result<Value>;

impl StandardLibrary {
    pub fn new() -> Self {
        let mut stdlib = Self {
            modules: HashMap::new(),
        };

        stdlib.register_all_modules();
        stdlib
    }

    /// Register all standard library modules
    fn register_all_modules(&mut self) {
        self.register_math_module();
        self.register_string_module();
        self.register_array_module();
        self.register_io_module();
        self.register_async_module();
    }

    /// Get a function from a module
    pub fn get_function(&self, module: &str, function: &str) -> Option<&NativeFunction> {
        self.modules.get(module)?.get(function)
    }

    /// Check if a module exists
    pub fn has_module(&self, module: &str) -> bool {
        self.modules.contains_key(module)
    }

    /// Get all available modules
    pub fn get_modules(&self) -> Vec<&str> {
        self.modules.keys().map(|s| s.as_str()).collect()
    }

    /// Get all functions in a module
    pub fn get_module_functions(&self, module: &str) -> Option<Vec<&str>> {
        self.modules
            .get(module)
            .map(|funcs| funcs.keys().map(|s| s.as_str()).collect())
    }

    // Module registration methods
    fn register_math_module(&mut self) {
        let mut math_funcs = HashMap::new();
        math_funcs.insert("sqrt".to_string(), math::sqrt as NativeFunction);
        math_funcs.insert("abs".to_string(), math::abs as NativeFunction);
        math_funcs.insert("max".to_string(), math::max as NativeFunction);
        math_funcs.insert("min".to_string(), math::min as NativeFunction);
        math_funcs.insert("pow".to_string(), math::pow as NativeFunction);
        math_funcs.insert("floor".to_string(), math::floor as NativeFunction);
        math_funcs.insert("ceil".to_string(), math::ceil as NativeFunction);
        math_funcs.insert("round".to_string(), math::round as NativeFunction);

        self.modules.insert("math".to_string(), math_funcs);
    }

    fn register_string_module(&mut self) {
        let mut string_funcs = HashMap::new();
        string_funcs.insert("length".to_string(), string::length as NativeFunction);
        string_funcs.insert("split".to_string(), string::split as NativeFunction);
        string_funcs.insert("join".to_string(), string::join as NativeFunction);
        string_funcs.insert("upper".to_string(), string::upper as NativeFunction);
        string_funcs.insert("lower".to_string(), string::lower as NativeFunction);
        string_funcs.insert("trim".to_string(), string::trim as NativeFunction);
        string_funcs.insert("contains".to_string(), string::contains as NativeFunction);
        string_funcs.insert("substring".to_string(), string::substring as NativeFunction);
        // New enhanced string functions
        string_funcs.insert("replace".to_string(), string::replace as NativeFunction);
        string_funcs.insert(
            "starts_with".to_string(),
            string::starts_with as NativeFunction,
        );
        string_funcs.insert("ends_with".to_string(), string::ends_with as NativeFunction);
        string_funcs.insert("repeat".to_string(), string::repeat as NativeFunction);
        string_funcs.insert("pad_left".to_string(), string::pad_left as NativeFunction);
        string_funcs.insert("pad_right".to_string(), string::pad_right as NativeFunction);

        self.modules.insert("string".to_string(), string_funcs);
    }

    fn register_array_module(&mut self) {
        let mut array_funcs = HashMap::new();
        array_funcs.insert("length".to_string(), array::length as NativeFunction);
        array_funcs.insert("push".to_string(), array::push as NativeFunction);
        array_funcs.insert("pop".to_string(), array::pop as NativeFunction);
        array_funcs.insert("sort".to_string(), array::sort as NativeFunction);
        array_funcs.insert("reverse".to_string(), array::reverse as NativeFunction);
        array_funcs.insert("join".to_string(), array::join as NativeFunction);
        // New functional programming methods
        array_funcs.insert("map".to_string(), array::map as NativeFunction);
        array_funcs.insert("filter".to_string(), array::filter as NativeFunction);
        array_funcs.insert("reduce".to_string(), array::reduce as NativeFunction);
        array_funcs.insert("find".to_string(), array::find as NativeFunction);
        array_funcs.insert("contains".to_string(), array::contains as NativeFunction);
        array_funcs.insert("first".to_string(), array::first as NativeFunction);
        array_funcs.insert("last".to_string(), array::last as NativeFunction);

        self.modules.insert("array".to_string(), array_funcs);
    }

    fn register_io_module(&mut self) {
        let mut io_funcs = HashMap::new();
        io_funcs.insert("read_file".to_string(), io::read_file as NativeFunction);
        io_funcs.insert("write_file".to_string(), io::write_file as NativeFunction);
        io_funcs.insert("exists".to_string(), io::exists as NativeFunction);
        io_funcs.insert("throw".to_string(), io::throw_exception as NativeFunction);

        self.modules.insert("io".to_string(), io_funcs);
    }

    fn register_async_module(&mut self) {
        let mut async_funcs = HashMap::new();
        async_funcs.insert(
            "create_promise".to_string(),
            async_mod::create_promise as NativeFunction,
        );
        async_funcs.insert(
            "create_rejected_promise".to_string(),
            async_mod::create_rejected_promise as NativeFunction,
        );
        async_funcs.insert("sleep".to_string(), async_mod::sleep as NativeFunction);
        async_funcs.insert(
            "read_file".to_string(),
            async_mod::read_file_async as NativeFunction,
        );
        async_funcs.insert(
            "write_file".to_string(),
            async_mod::write_file_async as NativeFunction,
        );
        async_funcs.insert(
            "http_get".to_string(),
            async_mod::http_get_async as NativeFunction,
        );
        async_funcs.insert("race".to_string(), async_mod::race as NativeFunction);
        async_funcs.insert("all".to_string(), async_mod::all as NativeFunction);
        async_funcs.insert("timeout".to_string(), async_mod::timeout as NativeFunction);
        async_funcs.insert("then".to_string(), async_mod::then as NativeFunction);
        self.modules.insert("async".to_string(), async_funcs);
    }
}

impl Default for StandardLibrary {
    fn default() -> Self {
        Self::new()
    }
}
