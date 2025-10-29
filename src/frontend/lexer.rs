use crate::core::{InfraError, Result};
use crate::frontend::{Token, TokenType};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
    start_position: usize,
    start_line: usize,
    start_column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
            start_position: 0,
            start_line: 1,
            start_column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.skip_whitespace();

            if self.is_at_end() {
                break;
            }

            self.start_token();
            let token = self.next_token()?;
            tokens.push(token);
        }

        tokens.push(Token::eof(self.line, self.column));
        Ok(tokens)
    }

    fn start_token(&mut self) {
        self.start_position = self.position;
        self.start_line = self.line;
        self.start_column = self.column;
    }

    fn next_token(&mut self) -> Result<Token> {
        let c = self.advance();

        let token_type = match c {
            '+' => TokenType::Plus,
            '-' => {
                if self.match_char('>') {
                    TokenType::Arrow
                } else {
                    TokenType::Minus
                }
            }
            '*' => TokenType::Star,
            '/' => {
                if self.match_char('/') {
                    // Handle // style comments
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                    // Return the next token after the comment
                    return self.next_token();
                } else {
                    TokenType::Slash
                }
            }
            '%' => TokenType::Percent,
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            '[' => TokenType::LeftBracket,
            ']' => TokenType::RightBracket,
            ',' => TokenType::Comma,
            ':' => TokenType::Colon,
            ';' => TokenType::Semicolon,
            '.' => TokenType::Dot,
            '\n' => {
                self.line += 1;
                self.column = 1;
                TokenType::Newline
            }
            '=' => {
                if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                }
            }
            '!' => {
                if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                }
            }
            '<' => {
                if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                }
            }
            '>' => {
                if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                }
            }
            '&' => {
                if self.match_char('&') {
                    TokenType::And
                } else {
                    return self.error("Unexpected character '&'");
                }
            }
            '|' => {
                if self.match_char('|') {
                    TokenType::Or
                } else {
                    TokenType::Pipe // Single pipe for union types
                }
            }
            '"' => self.string()?,
            _ if c.is_ascii_digit() => self.number(c)?,
            _ if c.is_alphabetic() || c == '_' => self.identifier(c)?,
            _ => return self.error(&format!("Unexpected character '{}'", c)),
        };

        Ok(self.make_token(token_type))
    }

    fn string(&mut self) -> Result<TokenType> {
        let mut value = String::new();

        while !self.is_at_end() && self.peek() != '"' {
            let c = self.advance();
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            }
            // Handle escape sequences
            if c == '\\' && !self.is_at_end() {
                let escaped = self.advance();
                match escaped {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    '\\' => value.push('\\'),
                    '"' => value.push('"'),
                    _ => {
                        value.push('\\');
                        value.push(escaped);
                    }
                }
            } else {
                value.push(c);
            }
        }

        if self.is_at_end() {
            return self.error("Unterminated string");
        }

        self.advance(); // Consume closing "
        Ok(TokenType::String(value))
    }

    fn number(&mut self, first_digit: char) -> Result<TokenType> {
        let mut value = String::new();
        value.push(first_digit);

        while !self.is_at_end() && self.peek().is_ascii_digit() {
            value.push(self.advance());
        }

        // Handle decimal point
        if !self.is_at_end() && self.peek() == '.' && self.peek_next().is_ascii_digit() {
            value.push(self.advance()); // consume '.'
            while !self.is_at_end() && self.peek().is_ascii_digit() {
                value.push(self.advance());
            }
        }

        match value.parse::<f64>() {
            Ok(num) => Ok(TokenType::Number(num)),
            Err(_) => self.error(&format!("Invalid number: {}", value)),
        }
    }

    fn identifier(&mut self, first_char: char) -> Result<TokenType> {
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
            "return" => TokenType::Return,
            "function" => TokenType::Function,
            "def" => TokenType::Def,
            "try" => TokenType::Try,
            "catch" => TokenType::Catch,
            "import" => TokenType::Import,
            "export" => TokenType::Export,
            "from" => TokenType::From,
            "as" => TokenType::As,
            "async" => TokenType::Async,
            "await" => TokenType::Await,
            "class" => TokenType::Class,
            "extends" => TokenType::Extends,
            "this" => TokenType::This,
            "super" => TokenType::Super,
            "init" => TokenType::Init,
            "new" => TokenType::New,
            // Type keywords
            "number" => TokenType::NumberType,
            "string" => TokenType::StringType,
            "boolean" => TokenType::BooleanType,
            _ => TokenType::Identifier(value),
        };

        Ok(token_type)
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '#' => {
                    // Skip comments until newline
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                }
                '/' => {
                    // Check for // style comments
                    if self.position + 1 < self.input.len() && self.peek_next() == '/' {
                        // Skip // comments until newline
                        while !self.is_at_end() && self.peek() != '\n' {
                            self.advance();
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.peek() != expected {
            false
        } else {
            self.advance();
            true
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

    fn peek_next(&self) -> char {
        if self.position + 1 >= self.input.len() {
            '\0'
        } else {
            self.input[self.position + 1]
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        let lexeme = self.input[self.start_position..self.position]
            .iter()
            .collect();
        Token::new(token_type, self.start_line, self.start_column, lexeme)
    }

    fn error<T>(&self, message: &str) -> Result<T> {
        Err(InfraError::LexError {
            message: message.to_string(),
            line: self.start_line,
            column: self.start_column,
            source_code: None,
        })
    }
}
