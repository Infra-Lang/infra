use crate::backend::bytecode::Compiler;
use crate::backend::vm::VM;
use crate::core::ast::{Program, Stmt, Expr};
use crate::core::Value;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_arithmetic() {
        // Create a simple program: 2 + 3
        let mut program = Program::new();
        program.add_statement(Stmt::Expression(Expr::Binary {
            left: Box::new(Expr::Literal(Value::Number(2.0))),
            operator: crate::core::ast::BinaryOp::Add,
            right: Box::new(Expr::Literal(Value::Number(3.0))),
        }));

        // Compile to bytecode
        let compiler = Compiler::new();
        let chunk = compiler.compile(&program).expect("Compilation should succeed");

        // Execute in VM
        let mut vm = VM::new();
        vm.interpret(chunk).expect("Execution should succeed");
    }

    #[test]
    fn test_variable_assignment() {
        // Create program: let x = 42; print x
        let mut program = Program::new();
        program.add_statement(Stmt::Let {
            name: "x".to_string(),
            type_annotation: None,
            value: Expr::Literal(Value::Number(42.0)),
        });
        program.add_statement(Stmt::Print(Expr::Identifier("x".to_string())));

        // Compile and execute
        let compiler = Compiler::new();
        let chunk = compiler.compile(&program).expect("Compilation should succeed");

        let mut vm = VM::new();
        vm.interpret(chunk).expect("Execution should succeed");
    }
}
