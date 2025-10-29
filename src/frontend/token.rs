use crate::core::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Number(f64),
    String(String),
    Identifier(String),

    // Keywords
    Let,
    If,
    Else,
    While,
    For,
    In,
    Range,
    True,
    False,
    Null,
    Print,
    Return,
    Function,
    Def,     // Alternative function keyword
    Try,     // New: try statement
    Catch,   // New: catch statement
    Import,  // New: import statement
    Export,  // New: export statement
    From,    // New: from keyword for imports
    As,      // New: as keyword for aliasing
    Async,   // New: async keyword
    Await,   // New: await keyword
    Class,   // New: class keyword
    Extends, // New: extends keyword for inheritance
    This,    // New: this keyword
    Super,   // New: super keyword
    Init,    // New: constructor keyword

    // Type annotations (NEW)
    Arrow,       // -> for function return types
    NumberType,  // number type keyword
    StringType,  // string type keyword
    BooleanType, // boolean type keyword

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    Pipe, // | for union types

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    Semicolon,
    Dot,
    Newline,

    // End of file
    Eof,
}

impl TokenType {
    pub fn is_literal(&self) -> bool {
        matches!(
            self,
            TokenType::Number(_)
                | TokenType::String(_)
                | TokenType::True
                | TokenType::False
                | TokenType::Null
        )
    }

    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            TokenType::Let
                | TokenType::If
                | TokenType::Else
                | TokenType::While
                | TokenType::For
                | TokenType::In
                | TokenType::Range
                | TokenType::True
                | TokenType::False
                | TokenType::Null
                | TokenType::Print
                | TokenType::Return
                | TokenType::Function
                | TokenType::Def
                | TokenType::Try
                | TokenType::Catch
                | TokenType::Import
                | TokenType::Export
                | TokenType::From
                | TokenType::As
                | TokenType::Async
                | TokenType::Await
                | TokenType::Class
                | TokenType::Extends
                | TokenType::This
                | TokenType::Super
                | TokenType::Init
        )
    }

    pub fn is_operator(&self) -> bool {
        matches!(
            self,
            TokenType::Plus
                | TokenType::Minus
                | TokenType::Star
                | TokenType::Slash
                | TokenType::Percent
                | TokenType::Equal
                | TokenType::EqualEqual
                | TokenType::Bang
                | TokenType::BangEqual
                | TokenType::Less
                | TokenType::LessEqual
                | TokenType::Greater
                | TokenType::GreaterEqual
                | TokenType::And
                | TokenType::Or
                | TokenType::Pipe
        )
    }

    pub fn to_literal_value(&self) -> Option<Value> {
        match self {
            TokenType::Number(n) => Some(Value::Number(*n)),
            TokenType::String(s) => Some(Value::String(s.clone())),
            TokenType::True => Some(Value::Boolean(true)),
            TokenType::False => Some(Value::Boolean(false)),
            TokenType::Null => Some(Value::Null),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
    pub lexeme: String,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize, lexeme: String) -> Self {
        Self {
            token_type,
            line,
            column,
            lexeme,
        }
    }

    pub fn eof(line: usize, column: usize) -> Self {
        Self::new(TokenType::Eof, line, column, String::new())
    }

    pub fn is_eof(&self) -> bool {
        matches!(self.token_type, TokenType::Eof)
    }
}
