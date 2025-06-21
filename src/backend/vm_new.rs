use crate::backend::bytecode::{Chunk, OpCode};
use crate::core::{Value, error::InfraError};
use std::collections::HashMap;

const STACK_MAX: usize = 256;

#[derive(Debug)]
pub struct VM {
    chunk: Option<Chunk>,
    ip: usize,              // Instruction pointer
    stack: Vec<Value>,
    locals: Vec<Value>,     // Local variables storage
    globals: HashMap<String, Value>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            chunk: None,
            ip: 0,
            stack: Vec::with_capacity(STACK_MAX),
            locals: Vec::new(),
            globals: HashMap::new(),
        }
    }
    
    pub fn interpret(&mut self, chunk: Chunk) -> Result<(), InfraError> {
        self.chunk = Some(chunk);
        self.ip = 0;
        self.run()
    }
    
    fn run(&mut self) -> Result<(), InfraError> {
        loop {
            if self.ip >= self.chunk.as_ref().unwrap().code.len() {
                break;
            }
            
            let instruction = self.chunk.as_ref().unwrap().code[self.ip];
            self.ip += 1;
            
            match instruction {
                OpCode::LoadConst(index) => {
                    let value = self.chunk.as_ref().unwrap().constants[index].clone();
                    self.push(value)?;
                }
                
                OpCode::LoadVar(slot) => {
                    if slot >= self.locals.len() {
                        self.locals.resize(slot + 1, Value::Null);
                    }
                    let value = self.locals[slot].clone();
                    self.push(value)?;
                }
                
                OpCode::StoreVar(slot) => {
                    let value = self.pop()?;
                    if slot >= self.locals.len() {
                        self.locals.resize(slot + 1, Value::Null);
                    }
                    self.locals[slot] = value;
                }
                
                OpCode::Pop => {
                    self.pop()?;
                }
                
                OpCode::Add => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let result = (a + b)?;
                    self.push(result)?;
                }
                
                OpCode::Sub => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let result = (a - b)?;
                    self.push(result)?;
                }
                
                OpCode::Mul => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let result = (a * b)?;
                    self.push(result)?;
                }
                
                OpCode::Div => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let result = (a / b)?;
                    self.push(result)?;
                }
                
                OpCode::Mod => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let result = (a % b)?;
                    self.push(result)?;
                }
                
                OpCode::Negate => {
                    let value = self.pop()?;
                    match value {
                        Value::Number(n) => self.push(Value::Number(-n))?,
                        _ => return Err(InfraError::Runtime(
                            "Can only negate numbers".to_string()
                        )),
                    }
                }
                
                OpCode::Equal => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(Value::Boolean(a == b))?;
                }
                
                OpCode::NotEqual => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(Value::Boolean(a != b))?;
                }
                
                OpCode::Less => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.push(Value::Boolean(a < b))?;
                        }
                        _ => return Err(InfraError::Runtime(
                            "Can only compare numbers".to_string()
                        )),
                    }
                }
                
                OpCode::Greater => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.push(Value::Boolean(a > b))?;
                        }
                        _ => return Err(InfraError::Runtime(
                            "Can only compare numbers".to_string()
                        )),
                    }
                }
                
                OpCode::LessEqual => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.push(Value::Boolean(a <= b))?;
                        }
                        _ => return Err(InfraError::Runtime(
                            "Can only compare numbers".to_string()
                        )),
                    }
                }
                
                OpCode::GreaterEqual => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.push(Value::Boolean(a >= b))?;
                        }
                        _ => return Err(InfraError::Runtime(
                            "Can only compare numbers".to_string()
                        )),
                    }
                }
                
                OpCode::And => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let result = Value::Boolean(a.is_truthy() && b.is_truthy());
                    self.push(result)?;
                }
                
                OpCode::Or => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    let result = Value::Boolean(a.is_truthy() || b.is_truthy());
                    self.push(result)?;
                }
                
                OpCode::Not => {
                    let value = self.pop()?;
                    self.push(Value::Boolean(!value.is_truthy()))?;
                }
                
                OpCode::Print => {
                    let value = self.pop()?;
                    println!("{}", value);
                }
                
                OpCode::MakeArray(count) => {
                    let mut elements = Vec::with_capacity(count);
                    for _ in 0..count {
                        elements.push(self.pop()?);
                    }
                    elements.reverse(); // Since we popped in reverse order
                    self.push(Value::Array(elements))?;
                }
                
                OpCode::MakeObject(count) => {
                    let mut object = HashMap::new();
                    for _ in 0..count {
                        let value = self.pop()?;
                        let key = self.pop()?;
                        if let Value::String(key_str) = key {
                            object.insert(key_str, value);
                        } else {
                            return Err(InfraError::Runtime(
                                "Object keys must be strings".to_string()
                            ));
                        }
                    }
                    self.push(Value::Object(object))?;
                }
                
                OpCode::Jump(target) => {
                    self.ip = target;
                }
                
                OpCode::JumpIfFalse(target) => {
                    let value = self.pop()?;
                    if !value.is_truthy() {
                        self.ip = target;
                    }
                }
                
                OpCode::Return => {
                    // For now, just break out of the loop
                    // In a full implementation, this would handle function returns
                    break;
                }
                
                OpCode::Halt => {
                    break;
                }
                
                _ => {
                    return Err(InfraError::Runtime(
                        format!("Unimplemented opcode: {:?}", instruction)
                    ));
                }
            }
        }
        
        Ok(())
    }
    
    fn push(&mut self, value: Value) -> Result<(), InfraError> {
        if self.stack.len() >= STACK_MAX {
            return Err(InfraError::Runtime("Stack overflow".to_string()));
        }
        self.stack.push(value);
        Ok(())
    }
    
    fn pop(&mut self) -> Result<Value, InfraError> {
        self.stack.pop().ok_or_else(|| InfraError::Runtime("Stack underflow".to_string()))
    }
    
    fn peek(&self, distance: usize) -> &Value {
        &self.stack[self.stack.len() - 1 - distance]
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}
