use crate::backend::bytecode::{Chunk, OpCode};
use crate::core::{error::InfraError, Value};
use std::collections::HashMap;

const STACK_MAX: usize = 256;

#[derive(Debug)]
pub struct VM {
    chunk: Option<Chunk>,
    ip: usize, // Instruction pointer
    stack: Vec<Value>,
    locals: Vec<Value>, // Local variables storage
    globals: HashMap<String, Value>,

    // Async execution state
    async_state: AsyncState,
    event_loop: EventLoop,
}

#[derive(Debug, Clone)]
pub struct AsyncState {
    pub is_async: bool,
    pub suspended_ip: Option<usize>,
    pub suspended_locals: Vec<Value>,
    pub current_promise: Option<Value>,
    pub awaiting: bool,
}

#[derive(Debug)]
pub struct EventLoop {
    promises: Vec<Promise>,
    tasks: Vec<Task>,
    microtasks: Vec<MicroTask>,
    timers: Vec<Timer>,
}

#[derive(Debug, Clone)]
pub struct Promise {
    pub id: usize,
    pub value: Option<Value>,
    pub resolved: bool,
    pub rejected: bool,
    pub error: Option<String>,
    pub callbacks: Vec<Value>,
}

#[derive(Debug)]
pub struct Task {
    pub id: usize,
    pub chunk: Chunk,
    pub ip: usize,
    pub stack: Vec<Value>,
    pub locals: Vec<Value>,
    pub awaiting_promise: Option<usize>,
}

#[derive(Debug)]
pub struct MicroTask {
    pub callback: Value,
    pub value: Value,
}

#[derive(Debug)]
pub struct Timer {
    pub id: usize,
    pub duration: u64,
    pub callback: Value,
    pub elapsed: u64,
}

impl VM {
    pub fn new() -> Self {
        Self {
            chunk: None,
            ip: 0,
            stack: Vec::with_capacity(STACK_MAX),
            locals: Vec::new(),
            globals: HashMap::new(),
            async_state: AsyncState {
                is_async: false,
                suspended_ip: None,
                suspended_locals: Vec::new(),
                current_promise: None,
                awaiting: false,
            },
            event_loop: EventLoop {
                promises: Vec::new(),
                tasks: Vec::new(),
                microtasks: Vec::new(),
                timers: Vec::new(),
            },
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
                        _ => {
                            return Err(InfraError::Runtime("Can only negate numbers".to_string()))
                        }
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
                        _ => {
                            return Err(InfraError::Runtime("Can only compare numbers".to_string()))
                        }
                    }
                }

                OpCode::Greater => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.push(Value::Boolean(a > b))?;
                        }
                        _ => {
                            return Err(InfraError::Runtime("Can only compare numbers".to_string()))
                        }
                    }
                }

                OpCode::LessEqual => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.push(Value::Boolean(a <= b))?;
                        }
                        _ => {
                            return Err(InfraError::Runtime("Can only compare numbers".to_string()))
                        }
                    }
                }

                OpCode::GreaterEqual => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.push(Value::Boolean(a >= b))?;
                        }
                        _ => {
                            return Err(InfraError::Runtime("Can only compare numbers".to_string()))
                        }
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
                                "Object keys must be strings".to_string(),
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

                OpCode::CreatePromise => {
                    let promise = self.create_promise()?;
                    self.push(promise)?;
                }

                OpCode::ResolvePromise => {
                    let value = self.pop()?;
                    self.resolve_promise(value)?;
                }

                OpCode::RejectPromise => {
                    let error = self.pop()?;
                    self.reject_promise(error)?;
                }

                OpCode::Await => {
                    let promise = self.pop()?;
                    let result = self.await_promise(promise)?;
                    if result.is_some() {
                        self.push(result.unwrap())?;
                    } else {
                        // Suspend execution
                        return Ok(());
                    }
                }

                OpCode::AsyncCall => {
                    // Handle async function calls
                    // For now, just treat as regular call
                    let arg_count = self.pop()?;
                    let args_count = if let Value::Number(n) = arg_count {
                        n as usize
                    } else {
                        0
                    };

                    let mut args = Vec::new();
                    for _ in 0..args_count {
                        args.push(self.pop()?);
                    }
                    args.reverse();

                    // Pop the function and call it
                    let function = self.pop()?;
                    // TODO: Implement proper async function calling
                    self.push(Value::Null)?;
                }

                OpCode::Halt => {
                    break;
                }

                _ => {
                    return Err(InfraError::Runtime(format!(
                        "Unimplemented opcode: {:?}",
                        instruction
                    )));
                }
            }
        }

        Ok(())
    }

    // Async helper methods
    fn create_promise(&mut self) -> Result<Value, InfraError> {
        let promise_id = self.event_loop.promises.len();
        let promise = Promise {
            id: promise_id,
            value: None,
            resolved: false,
            rejected: false,
            error: None,
            callbacks: Vec::new(),
        };

        self.event_loop.promises.push(promise.clone());

        Ok(Value::Promise {
            value: None,
            resolved: false,
            rejected: false,
            error: None,
        })
    }

    fn resolve_promise(&mut self, value: Value) -> Result<(), InfraError> {
        // Get the current promise being resolved
        if let Some(current_promise) = &self.async_state.current_promise {
            if let Value::Promise { resolved, .. } = current_promise {
                if !*resolved {
                    // TODO: Implement promise resolution logic
                    // For now, just mark as resolved
                    // In a full implementation, this would:
                    // 1. Set the promise value
                    // 2. Mark as resolved
                    // 3. Execute callbacks
                    // 4. Resume any waiting tasks
                }
            }
        }
        Ok(())
    }

    fn reject_promise(&mut self, error: Value) -> Result<(), InfraError> {
        // Get the current promise being rejected
        if let Some(current_promise) = &self.async_state.current_promise {
            if let Value::Promise { rejected, .. } = current_promise {
                if !*rejected {
                    // TODO: Implement promise rejection logic
                    // For now, just mark as rejected
                }
            }
        }
        Ok(())
    }

    fn await_promise(&mut self, promise: Value) -> Result<Option<Value>, InfraError> {
        match promise {
            Value::Promise {
                resolved, value, ..
            } => {
                if *resolved {
                    // Promise is already resolved, return the value
                    Ok(value.clone())
                } else {
                    // Promise is not resolved yet, suspend execution
                    self.suspend_execution(promise)?;
                    Ok(None)
                }
            }
            _ => Err(InfraError::Runtime(
                "await can only be used with promises".to_string(),
            )),
        }
    }

    fn suspend_execution(&mut self, promise: Value) -> Result<(), InfraError> {
        // Save current execution state
        self.async_state.suspended_ip = Some(self.ip);
        self.async_state.suspended_locals = self.locals.clone();
        self.async_state.current_promise = Some(promise.clone());
        self.async_state.awaiting = true;

        // TODO: Add this task to the event loop
        // For now, just suspend and let the event loop handle it

        Ok(())
    }

    fn resume_execution(&mut self) -> Result<(), InfraError> {
        // Restore execution state
        if let Some(suspended_ip) = self.async_state.suspended_ip {
            self.ip = suspended_ip;
        }
        self.locals = self.async_state.suspended_locals.clone();

        self.async_state.awaiting = false;
        self.async_state.current_promise = None;

        Ok(())
    }

    fn run_event_loop(&mut self) -> Result<(), InfraError> {
        // Process microtasks first
        while !self.event_loop.microtasks.is_empty() {
            let task = self.event_loop.microtasks.remove(0);
            self.execute_microtask(task)?;
        }

        // Process timers
        self.process_timers()?;

        // Process regular tasks
        if !self.event_loop.tasks.is_empty() {
            let task = self.event_loop.tasks.remove(0);
            self.execute_task(task)?;
        }

        Ok(())
    }

    fn execute_microtask(&mut self, task: MicroTask) -> Result<(), InfraError> {
        // Execute the callback with the resolved value
        // TODO: Implement microtask execution
        Ok(())
    }

    fn execute_task(&mut self, task: Task) -> Result<(), InfraError> {
        // Execute a suspended task
        self.chunk = Some(task.chunk);
        self.ip = task.ip;
        self.stack = task.stack;
        self.locals = task.locals;

        // Continue execution from where it left off
        self.run()
    }

    fn process_timers(&mut self) -> Result<(), InfraError> {
        // Check for expired timers and execute their callbacks
        let mut expired_timers = Vec::new();

        for i in (0..self.event_loop.timers.len()).rev() {
            let timer = &mut self.event_loop.timers[i];
            timer.elapsed += 1; // Simulate time passing

            if timer.elapsed >= timer.duration {
                expired_timers.push(i);
            }
        }

        // Execute expired timers
        for &index in &expired_timers {
            let timer = self.event_loop.timers.remove(index);
            self.execute_timer_callback(timer)?;
        }

        Ok(())
    }

    fn execute_timer_callback(&mut self, timer: Timer) -> Result<(), InfraError> {
        // Execute the timer callback
        // TODO: Implement timer callback execution
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
        self.stack
            .pop()
            .ok_or_else(|| InfraError::Runtime("Stack underflow".to_string()))
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
