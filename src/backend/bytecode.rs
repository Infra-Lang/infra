use crate::core::{
    ast::{BinaryOp, Expr, Program, Stmt, UnaryOp},
    Value,
};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpCode {
    // Stack operations
    LoadConst(usize), // Load constant from constant pool
    LoadVar(usize),   // Load variable from local variable table
    StoreVar(usize),  // Store to local variable table
    Pop,              // Pop top value from stack

    // Arithmetic operations
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Negate,

    // Comparison operations
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Logical operations
    And,
    Or,
    Not,

    // Control flow
    Jump(usize),        // Unconditional jump
    JumpIfFalse(usize), // Jump if top of stack is false
    Call(usize),        // Call function with n arguments
    Return,             // Return from function

    // Built-in functions
    Print,

    // Array operations
    MakeArray(usize), // Create array with n elements from stack
    ArrayGet,         // Get array element (array, index on stack)
    ArraySet,         // Set array element (array, index, value on stack)

    // Object operations
    MakeObject(usize), // Create object with n key-value pairs from stack
    ObjectGet,         // Get object property (object, key on stack)
    ObjectSet,         // Set object property (object, key, value on stack)

    // Async operations
    CreatePromise,  // Create a new promise
    ResolvePromise, // Resolve a promise with a value
    RejectPromise,  // Reject a promise with an error
    Await,          // Await a promise (suspends execution)
    AsyncCall,      // Call an async function

    // Halt execution
    Halt,
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>, // Line number for each bytecode instruction
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        // Check if constant already exists to avoid duplicates
        for (i, constant) in self.constants.iter().enumerate() {
            if *constant == value {
                return i;
            }
        }

        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn emit(&mut self, op: OpCode, line: usize) {
        self.code.push(op);
        self.lines.push(line);
    }

    pub fn emit_jump(&mut self, op: OpCode, line: usize) -> usize {
        self.emit(op, line);
        self.code.len() - 1
    }

    pub fn patch_jump(&mut self, offset: usize) {
        let jump_target = self.code.len();
        if let Some(OpCode::Jump(_)) | Some(OpCode::JumpIfFalse(_)) = self.code.get_mut(offset) {
            self.code[offset] = match self.code[offset] {
                OpCode::Jump(_) => OpCode::Jump(jump_target),
                OpCode::JumpIfFalse(_) => OpCode::JumpIfFalse(jump_target),
                _ => unreachable!(),
            };
        }
    }
}

#[derive(Debug)]
pub struct Compiler {
    chunk: Chunk,
    locals: HashMap<String, usize>,
    local_count: usize,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            locals: HashMap::new(),
            local_count: 0,
        }
    }

    pub fn compile(mut self, program: &Program) -> Result<Chunk, crate::core::error::InfraError> {
        for stmt in &program.statements {
            self.compile_stmt(stmt)?;
        }

        // Emit halt instruction at the end
        self.chunk.emit(OpCode::Halt, 0);

        Ok(self.chunk)
    }

    fn compile_stmt(&mut self, stmt: &Stmt) -> Result<(), crate::core::error::InfraError> {
        match stmt {
            Stmt::Expression(expr) => {
                self.compile_expr(expr)?;
                self.chunk.emit(OpCode::Pop, 0); // Pop unused expression result
            }

            Stmt::Print(expr) => {
                self.compile_expr(expr)?;
                self.chunk.emit(OpCode::Print, 0);
            }

            Stmt::Let { name, value, .. } => {
                self.compile_expr(value)?;
                let local_index = self.local_count;
                self.locals.insert(name.clone(), local_index);
                self.local_count += 1;
                self.chunk.emit(OpCode::StoreVar(local_index), 0);
            }

            Stmt::Assignment { target, value } => {
                self.compile_expr(value)?;
                match target {
                    crate::core::ast::AssignmentTarget::Identifier(name) => {
                        if let Some(&local_index) = self.locals.get(name) {
                            self.chunk.emit(OpCode::StoreVar(local_index), 0);
                        } else {
                            return Err(crate::core::error::InfraError::UndefinedVariable {
                                name: name.clone(),
                            });
                        }
                    }
                    _ => {
                        return Err(crate::core::error::InfraError::Runtime(
                            "Complex assignment targets not yet supported in bytecode".to_string(),
                        ));
                    }
                }
            }

            Stmt::Block(statements) => {
                for stmt in statements {
                    self.compile_stmt(stmt)?;
                }
            }

            Stmt::Return(expr) => {
                if let Some(expr) = expr {
                    self.compile_expr(expr)?;
                } else {
                    let null_const = self.chunk.add_constant(Value::Null);
                    self.chunk.emit(OpCode::LoadConst(null_const), 0);
                }
                self.chunk.emit(OpCode::Return, 0);
            }
            Stmt::Function {
                name, params, body, ..
            } => {
                // For now, compile function as a placeholder
                // In a full implementation, we'd compile the function body separately
                let func_name_const = self.chunk.add_constant(Value::String(name.clone()));
                self.chunk.emit(OpCode::LoadConst(func_name_const), 0);
                // Placeholder: push function as a value
                // TODO: Implement proper function compilation
            }
            Stmt::AsyncFunction {
                name, params, body, ..
            } => {
                // Compile async function similarly to regular function
                let func_name_const = self.chunk.add_constant(Value::String(name.clone()));
                self.chunk.emit(OpCode::LoadConst(func_name_const), 0);
                // Placeholder: push async function as a value
                // TODO: Implement proper async function compilation
            }

            _ => {
                return Err(crate::core::error::InfraError::Runtime(format!(
                    "Statement type not yet supported in bytecode: {:?}",
                    stmt
                )));
            }
        }

        Ok(())
    }

    fn compile_expr(&mut self, expr: &Expr) -> Result<(), crate::core::error::InfraError> {
        match expr {
            Expr::Literal(value) => {
                let const_index = self.chunk.add_constant(value.clone());
                self.chunk.emit(OpCode::LoadConst(const_index), 0);
            }

            Expr::Identifier(name) => {
                if let Some(&local_index) = self.locals.get(name) {
                    self.chunk.emit(OpCode::LoadVar(local_index), 0);
                } else {
                    return Err(crate::core::error::InfraError::UndefinedVariable {
                        name: name.clone(),
                    });
                }
            }

            Expr::Binary {
                left,
                operator,
                right,
            } => {
                self.compile_expr(left)?;
                self.compile_expr(right)?;

                match operator {
                    BinaryOp::Add => self.chunk.emit(OpCode::Add, 0),
                    BinaryOp::Subtract => self.chunk.emit(OpCode::Sub, 0),
                    BinaryOp::Multiply => self.chunk.emit(OpCode::Mul, 0),
                    BinaryOp::Divide => self.chunk.emit(OpCode::Div, 0),
                    BinaryOp::Modulo => self.chunk.emit(OpCode::Mod, 0),
                    BinaryOp::Equal => self.chunk.emit(OpCode::Equal, 0),
                    BinaryOp::NotEqual => self.chunk.emit(OpCode::NotEqual, 0),
                    BinaryOp::Less => self.chunk.emit(OpCode::Less, 0),
                    BinaryOp::Greater => self.chunk.emit(OpCode::Greater, 0),
                    BinaryOp::LessEqual => self.chunk.emit(OpCode::LessEqual, 0),
                    BinaryOp::GreaterEqual => self.chunk.emit(OpCode::GreaterEqual, 0),
                    BinaryOp::And => self.chunk.emit(OpCode::And, 0),
                    BinaryOp::Or => self.chunk.emit(OpCode::Or, 0),
                }
            }

            Expr::Unary { operator, operand } => {
                self.compile_expr(operand)?;

                match operator {
                    UnaryOp::Minus => self.chunk.emit(OpCode::Negate, 0),
                    UnaryOp::Not => self.chunk.emit(OpCode::Not, 0),
                }
            }

            Expr::Await { expression } => {
                self.compile_expr(expression)?;
                self.chunk.emit(OpCode::Await, 0);
            }

            Expr::Array(elements) => {
                for element in elements {
                    self.compile_expr(element)?;
                }
                self.chunk.emit(OpCode::MakeArray(elements.len()), 0);
            }

            Expr::Object(fields) => {
                for (key, value) in fields {
                    let key_const = self.chunk.add_constant(Value::String(key.clone()));
                    self.chunk.emit(OpCode::LoadConst(key_const), 0);
                    self.compile_expr(value)?;
                }
                self.chunk.emit(OpCode::MakeObject(fields.len()), 0);
            }

            _ => {
                return Err(crate::core::error::InfraError::Runtime(format!(
                    "Expression type not yet supported in bytecode: {:?}",
                    expr
                )));
            }
        }

        Ok(())
    }
}
