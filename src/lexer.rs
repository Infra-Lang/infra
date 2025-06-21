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
    
    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Colon,
    Newline,
    
    // End of file
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }
    
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        
        while !self.is_at_end() {
            self.skip_whitespace();
            
            if self.is_at_end() {
                break;
            }
            
            let token = self.next_token()?;
            tokens.push(token);
        }
        
        tokens.push(Token {
            token_type: TokenType::Eof,
            line: self.line,
            column: self.column,
        });
        
        Ok(tokens)
    }
    
    fn next_token(&mut self) -> Result<Token, String> {
        let start_line = self.line;
        let start_column = self.column;
        
        let c = self.advance();
        
        let token_type = match c {
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '*' => TokenType::Star,
            '/' => TokenType::Slash,
            '%' => TokenType::Percent,
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            ':' => TokenType::Colon,
            '\n' => {
                self.line += 1;
                self.column = 1;
                TokenType::Newline
            },
            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                }
            },
            '!' => {
                if self.peek() == '=' {
                    self.advance();
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                }
            },
            '<' => {
                if self.peek() == '=' {
                    self.advance();
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                }
            },
            '>' => {
                if self.peek() == '=' {
                    self.advance();
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                }
            },
            '&' => {
                if self.peek() == '&' {
                    self.advance();
                    TokenType::And
                } else {
                    return Err(format!("Unexpected character '&' at line {}, column {}", start_line, start_column));
                }
            },
            '|' => {
                if self.peek() == '|' {
                    self.advance();
                    TokenType::Or
                } else {
                    return Err(format!("Unexpected character '|' at line {}, column {}", start_line, start_column));
                }
            },
            '"' => self.string()?,
            _ if c.is_ascii_digit() => self.number(c)?,
            _ if c.is_alphabetic() || c == '_' => self.identifier(c)?,
            _ => return Err(format!("Unexpected character '{}' at line {}, column {}", c, start_line, start_column)),
        };
        
        Ok(Token {
            token_type,
            line: start_line,
            column: start_column,
        })
    }
    
    fn string(&mut self) -> Result<TokenType, String> {
        let mut value = String::new();
        
        while !self.is_at_end() && self.peek() != '"' {
            let c = self.advance();
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            }
            value.push(c);
        }
        
        if self.is_at_end() {
            return Err("Unterminated string".to_string());
        }
        
        self.advance(); // Consume closing "
        Ok(TokenType::String(value))
    }
    
    fn number(&mut self, first_digit: char) -> Result<TokenType, String> {
        let mut value = String::new();
        value.push(first_digit);
        
        while !self.is_at_end() && (self.peek().is_ascii_digit() || self.peek() == '.') {
            value.push(self.advance());
        }
        
        match value.parse::<f64>() {
            Ok(num) => Ok(TokenType::Number(num)),
            Err(_) => Err(format!("Invalid number: {}", value)),
        }
    }
    
    fn identifier(&mut self, first_char: char) -> Result<TokenType, String> {
        let mut value = String::new();
        value.push(first_char);
        
        while !self.is_at_end() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            value.push(self.advance());
        }
        
        let token_type = match value.as_str() {
            "let" => TokenType::Let,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "for" => TokenType::For,
            "in" => TokenType::In,
            "range" => TokenType::Range,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "null" => TokenType::Null,
            "print" => TokenType::Print,
            _ => TokenType::Identifier(value),
        };
        
        Ok(token_type)
    }
    
    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                },
                '#' => {
                    // Skip comments until newline
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                    // Return to skip_whitespace to continue processing
                    continue;
                },
                _ => break,
            }
        }
    }
    
    fn advance(&mut self) -> char {
        let c = self.input[self.position];
        self.position += 1;
        self.column += 1;
        c
    }
    
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.input[self.position]
        }
    }
    
    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }
}
