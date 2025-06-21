#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
        }
    }
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
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Not,
    Minus,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Let {
        name: String,
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
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Stmt>,
}
