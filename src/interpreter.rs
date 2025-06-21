use crate::ast::*;
use std::collections::HashMap;

pub struct Interpreter {
    variables: HashMap<String, Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
    
    pub fn execute(&mut self, program: &Program) -> Result<(), String> {
        for stmt in &program.statements {
            self.execute_statement(stmt)?;
        }
        Ok(())
    }
    
    fn execute_statement(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Expression(expr) => {
                self.evaluate_expression(expr)?;
                Ok(())
            }
            Stmt::Let { name, value } => {
                let val = self.evaluate_expression(value)?;
                self.variables.insert(name.clone(), val);
                Ok(())
            }
            Stmt::Print(expr) => {
                let value = self.evaluate_expression(expr)?;
                println!("{}", value);
                Ok(())
            }
            Stmt::Block(statements) => {
                // Create new scope
                let old_vars = self.variables.clone();
                
                for statement in statements {
                    self.execute_statement(statement)?;
                }
                
                // Restore old scope (simple scoping for now)
                self.variables = old_vars;
                Ok(())
            }
            Stmt::If { condition, then_stmt, else_stmt } => {
                let condition_value = self.evaluate_expression(condition)?;
                
                if self.is_truthy(&condition_value) {
                    self.execute_statement(then_stmt)?;
                } else if let Some(else_stmt) = else_stmt {
                    self.execute_statement(else_stmt)?;
                }
                Ok(())
            }
            Stmt::While { condition, body } => {
                loop {
                    let condition_value = self.evaluate_expression(condition)?;
                    if !self.is_truthy(&condition_value) {
                        break;
                    }
                    self.execute_statement(body)?;
                }
                Ok(())
            }
            Stmt::For { var, start, end, body } => {
                let start_val = self.evaluate_expression(start)?;
                let end_val = self.evaluate_expression(end)?;
                
                let (start_num, end_num) = match (start_val, end_val) {
                    (Value::Number(s), Value::Number(e)) => (s as i64, e as i64),
                    _ => return Err("For loop range must be numbers".to_string()),
                };
                
                let old_var_value = self.variables.get(var).cloned();
                
                for i in start_num..end_num {
                    self.variables.insert(var.clone(), Value::Number(i as f64));
                    self.execute_statement(body)?;
                }
                
                // Restore old variable value
                if let Some(old_value) = old_var_value {
                    self.variables.insert(var.clone(), old_value);
                } else {
                    self.variables.remove(var);
                }
                
                Ok(())
            }
        }
    }
    
    fn evaluate_expression(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Literal(value) => Ok(value.clone()),
            Expr::Identifier(name) => {
                self.variables
                    .get(name)
                    .cloned()
                    .ok_or_else(|| format!("Undefined variable '{}'", name))
            }
            Expr::Binary { left, operator, right } => {
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;
                self.apply_binary_operator(operator, &left_val, &right_val)
            }
            Expr::Unary { operator, operand } => {
                let operand_val = self.evaluate_expression(operand)?;
                self.apply_unary_operator(operator, &operand_val)
            }
            Expr::Call { callee: _, args: _ } => {
                // Function calls not implemented yet
                Err("Function calls not implemented yet".to_string())
            }
        }
    }
    
    fn apply_binary_operator(&self, op: &BinaryOp, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => {
                match op {
                    BinaryOp::Add => Ok(Value::Number(l + r)),
                    BinaryOp::Subtract => Ok(Value::Number(l - r)),
                    BinaryOp::Multiply => Ok(Value::Number(l * r)),
                    BinaryOp::Divide => {
                        if *r == 0.0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(Value::Number(l / r))
                        }
                    }
                    BinaryOp::Modulo => Ok(Value::Number(l % r)),
                    BinaryOp::Equal => Ok(Value::Boolean((l - r).abs() < f64::EPSILON)),
                    BinaryOp::NotEqual => Ok(Value::Boolean((l - r).abs() >= f64::EPSILON)),
                    BinaryOp::Less => Ok(Value::Boolean(l < r)),
                    BinaryOp::LessEqual => Ok(Value::Boolean(l <= r)),
                    BinaryOp::Greater => Ok(Value::Boolean(l > r)),
                    BinaryOp::GreaterEqual => Ok(Value::Boolean(l >= r)),
                    BinaryOp::And => Ok(Value::Boolean(self.is_truthy(left) && self.is_truthy(right))),
                    BinaryOp::Or => Ok(Value::Boolean(self.is_truthy(left) || self.is_truthy(right))),
                }
            }
            (Value::String(l), Value::String(r)) => {
                match op {
                    BinaryOp::Add => Ok(Value::String(format!("{}{}", l, r))),
                    BinaryOp::Equal => Ok(Value::Boolean(l == r)),
                    BinaryOp::NotEqual => Ok(Value::Boolean(l != r)),
                    _ => Err(format!("Operator {:?} not supported for strings", op)),
                }
            }
            (Value::Boolean(l), Value::Boolean(r)) => {
                match op {
                    BinaryOp::Equal => Ok(Value::Boolean(l == r)),
                    BinaryOp::NotEqual => Ok(Value::Boolean(l != r)),
                    BinaryOp::And => Ok(Value::Boolean(*l && *r)),
                    BinaryOp::Or => Ok(Value::Boolean(*l || *r)),
                    _ => Err(format!("Operator {:?} not supported for booleans", op)),
                }
            }
            _ => Err(format!("Type mismatch for operator {:?}", op)),
        }
    }
    
    fn apply_unary_operator(&self, op: &UnaryOp, operand: &Value) -> Result<Value, String> {
        match (op, operand) {
            (UnaryOp::Minus, Value::Number(n)) => Ok(Value::Number(-n)),
            (UnaryOp::Not, value) => Ok(Value::Boolean(!self.is_truthy(value))),
            _ => Err(format!("Unary operator {:?} not supported for {:?}", op, operand)),
        }
    }
    
    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Boolean(b) => *b,
            Value::Null => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
        }
    }
}
