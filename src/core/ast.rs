use crate::core::Value;

// Type system
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number,
    String,
    Boolean,
    Array(Box<Type>),            // Array of specific type: [number]
    Object(Vec<(String, Type)>), // Object with typed properties: {name: string, age: number}
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
    Union(Vec<Type>), // Union types: number | string
    Any,              // For untyped variables
    Never,            // Bottom type (for functions that never return)
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Value),
    Identifier(String),
    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
    },
    Unary {
        operator: UnaryOp,
        operand: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Array(Vec<Expr>),
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },
    Object(Vec<(String, Expr)>),
    Property {
        object: Box<Expr>,
        property: String,
    },
    ModuleAccess {
        module: String,
        function: String,
    },
    Await {
        expression: Box<Expr>,
    },
    This,
    Super {
        method: String,
    },
    New {
        class: Box<Expr>,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
}

impl BinaryOp {
    pub fn precedence(&self) -> u8 {
        match self {
            BinaryOp::Or => 1,
            BinaryOp::And => 2,
            BinaryOp::Equal | BinaryOp::NotEqual => 3,
            BinaryOp::Less | BinaryOp::LessEqual | BinaryOp::Greater | BinaryOp::GreaterEqual => 4,
            BinaryOp::Add | BinaryOp::Subtract => 5,
            BinaryOp::Multiply | BinaryOp::Divide | BinaryOp::Modulo => 6,
        }
    }

    pub fn is_left_associative(&self) -> bool {
        true // All our operators are left associative for now
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Not,
    Minus,
}

#[derive(Debug, Clone)]
pub struct MethodDecl {
    pub name: String,
    pub params: Vec<String>,
    pub param_types: Vec<Option<Type>>, // Optional parameter types
    pub return_type: Option<Type>,      // Optional return type
    pub body: Box<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Let {
        name: String,
        type_annotation: Option<Type>, // Optional type: let x: number = 42
        value: Expr,
    },
    If {
        condition: Expr,
        then_stmt: Box<Stmt>,
        else_stmt: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    For {
        var: String,
        start: Expr,
        end: Expr,
        body: Box<Stmt>,
    },
    Block(Vec<Stmt>),
    Print(Expr),
    Return(Option<Expr>),
    Function {
        name: String,
        params: Vec<String>,
        param_types: Vec<Option<Type>>, // Optional parameter types
        return_type: Option<Type>,      // Optional return type
        body: Box<Stmt>,
    },
    AsyncFunction {
        name: String,
        params: Vec<String>,
        param_types: Vec<Option<Type>>, // Optional parameter types
        return_type: Option<Type>,      // Optional return type
        body: Box<Stmt>,
    },
    Class {
        name: String,
        superclass: Option<String>,
        methods: Vec<MethodDecl>,
    },
    Try {
        try_block: Box<Stmt>,
        catch_var: String,
        catch_block: Box<Stmt>,
    },
    Assignment {
        target: AssignmentTarget,
        value: Expr,
    },
    Import {
        module_path: String,
        items: ImportItems,
        alias: Option<String>,
    },
    Export {
        item: ExportItem,
    },
}

#[derive(Debug, Clone)]
pub enum AssignmentTarget {
    Identifier(String),
    Property { object: Box<Expr>, property: String },
    Index { object: Box<Expr>, index: Box<Expr> },
}

#[derive(Debug, Clone)]
pub enum ImportItems {
    All,                    // import * from "module"
    Named(Vec<ImportItem>), // import {a, b} from "module"
    Default(String),        // import module from "module"
}

#[derive(Debug, Clone)]
pub struct ImportItem {
    pub name: String,
    pub alias: Option<String>, // import {a as b} from "module"
}

#[derive(Debug, Clone)]
pub enum ExportItem {
    Function {
        name: String,
        params: Vec<String>,
        param_types: Vec<Option<Type>>, // Optional parameter types
        return_type: Option<Type>,      // Optional return type
        body: Box<Stmt>,
    },
    Variable {
        name: String,
        type_annotation: Option<Type>, // Optional variable type
        value: Expr,
    },
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    pub fn add_statement(&mut self, stmt: Stmt) {
        self.statements.push(stmt);
    }

    pub fn is_empty(&self) -> bool {
        self.statements.is_empty()
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}
