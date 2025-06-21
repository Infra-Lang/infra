use crate::ast::*;
use crate::lexer::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }
    
    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            // Skip newlines at the top level
            if self.check(&TokenType::Newline) {
                self.advance();
                continue;
            }
            
            statements.push(self.statement()?);
        }
        
        Ok(Program { statements })
    }
    
    fn statement(&mut self) -> Result<Stmt, String> {
        if self.check(&TokenType::Let) {
            self.let_statement()
        } else if self.check(&TokenType::If) {
            self.if_statement()
        } else if self.check(&TokenType::While) {
            self.while_statement()
        } else if self.check(&TokenType::For) {
            self.for_statement()
        } else if self.check(&TokenType::Print) {
            self.print_statement()
        } else if self.check(&TokenType::LeftBrace) {
            self.block_statement()
        } else {
            let expr = self.expression()?;
            self.consume_newline_or_eof()?;
            Ok(Stmt::Expression(expr))
        }
    }
    
    fn let_statement(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'let'
        
        let name = match &self.advance().token_type {
            TokenType::Identifier(name) => name.clone(),
            _ => return Err("Expected variable name after 'let'".to_string()),
        };
        
        if !self.check(&TokenType::Equal) {
            return Err("Expected '=' after variable name".to_string());
        }
        self.advance(); // consume '='
        
        let value = self.expression()?;
        self.consume_newline_or_eof()?;
        
        Ok(Stmt::Let { name, value })
    }
    
    fn if_statement(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'if'
        
        let condition = self.expression()?;
        
        if !self.check(&TokenType::Colon) {
            return Err("Expected ':' after if condition".to_string());
        }
        self.advance(); // consume ':'
        
        // Skip optional newline after colon
        if self.check(&TokenType::Newline) {
            self.advance();
        }
        
        let then_stmt = Box::new(self.statement()?);
        
        let else_stmt = if self.check(&TokenType::Else) {
            self.advance(); // consume 'else'
            if !self.check(&TokenType::Colon) {
                return Err("Expected ':' after 'else'".to_string());
            }
            self.advance(); // consume ':'
            
            // Skip optional newline after colon
            if self.check(&TokenType::Newline) {
                self.advance();
            }
            
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        
        Ok(Stmt::If {
            condition,
            then_stmt,
            else_stmt,
        })
    }
    
    fn while_statement(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'while'
        
        let condition = self.expression()?;
        
        if !self.check(&TokenType::Colon) {
            return Err("Expected ':' after while condition".to_string());
        }
        self.advance(); // consume ':'
        
        // Skip optional newline after colon
        if self.check(&TokenType::Newline) {
            self.advance();
        }
        
        let body = Box::new(self.statement()?);
        
        Ok(Stmt::While { condition, body })
    }
    
    fn for_statement(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'for'
        
        let var = match &self.advance().token_type {
            TokenType::Identifier(name) => name.clone(),
            _ => return Err("Expected variable name in for loop".to_string()),
        };
        
        if !self.check(&TokenType::In) {
            return Err("Expected 'in' after for loop variable".to_string());
        }
        self.advance(); // consume 'in'
        
        if !self.check(&TokenType::Range) {
            return Err("Expected 'range' after 'in'".to_string());
        }
        self.advance(); // consume 'range'
        
        if !self.check(&TokenType::LeftParen) {
            return Err("Expected '(' after 'range'".to_string());
        }
        self.advance(); // consume '('
        
        let start = self.expression()?;
        
        if !self.check(&TokenType::Comma) {
            return Err("Expected ',' in range".to_string());
        }
        self.advance(); // consume ','
        
        let end = self.expression()?;
        
        if !self.check(&TokenType::RightParen) {
            return Err("Expected ')' after range".to_string());
        }
        self.advance(); // consume ')'
        
        if !self.check(&TokenType::Colon) {
            return Err("Expected ':' after for loop range".to_string());
        }
        self.advance(); // consume ':'
        
        // Skip optional newline after colon
        if self.check(&TokenType::Newline) {
            self.advance();
        }
        
        let body = Box::new(self.statement()?);
        
        Ok(Stmt::For { var, start, end, body })
    }
    
    fn print_statement(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume 'print'
        
        if !self.check(&TokenType::LeftParen) {
            return Err("Expected '(' after 'print'".to_string());
        }
        self.advance(); // consume '('
        
        let expr = self.expression()?;
        
        if !self.check(&TokenType::RightParen) {
            return Err("Expected ')' after print expression".to_string());
        }
        self.advance(); // consume ')'
        
        self.consume_newline_or_eof()?;
        
        Ok(Stmt::Print(expr))
    }
    
    fn block_statement(&mut self) -> Result<Stmt, String> {
        self.advance(); // consume '{'
        
        let mut statements = Vec::new();
        
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if self.check(&TokenType::Newline) {
                self.advance();
                continue;
            }
            statements.push(self.statement()?);
        }
        
        if !self.check(&TokenType::RightBrace) {
            return Err("Expected '}' after block".to_string());
        }
        self.advance(); // consume '}'
        
        Ok(Stmt::Block(statements))
    }
    
    fn expression(&mut self) -> Result<Expr, String> {
        self.or()
    }
    
    fn or(&mut self) -> Result<Expr, String> {
        let mut expr = self.and()?;
        
        while self.check(&TokenType::Or) {
            self.advance();
            let right = self.and()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: BinaryOp::Or,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn and(&mut self) -> Result<Expr, String> {
        let mut expr = self.equality()?;
        
        while self.check(&TokenType::And) {
            self.advance();
            let right = self.equality()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: BinaryOp::And,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;
        
        while matches!(self.peek().token_type, TokenType::EqualEqual | TokenType::BangEqual) {
            let operator = match self.advance().token_type {
                TokenType::EqualEqual => BinaryOp::Equal,
                TokenType::BangEqual => BinaryOp::NotEqual,
                _ => unreachable!(),
            };
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;
        
        while matches!(
            self.peek().token_type,
            TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual
        ) {
            let operator = match self.advance().token_type {
                TokenType::Greater => BinaryOp::Greater,
                TokenType::GreaterEqual => BinaryOp::GreaterEqual,
                TokenType::Less => BinaryOp::Less,
                TokenType::LessEqual => BinaryOp::LessEqual,
                _ => unreachable!(),
            };
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;
        
        while matches!(self.peek().token_type, TokenType::Minus | TokenType::Plus) {
            let operator = match self.advance().token_type {
                TokenType::Minus => BinaryOp::Subtract,
                TokenType::Plus => BinaryOp::Add,
                _ => unreachable!(),
            };
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;
        
        while matches!(
            self.peek().token_type,
            TokenType::Slash | TokenType::Star | TokenType::Percent
        ) {
            let operator = match self.advance().token_type {
                TokenType::Slash => BinaryOp::Divide,
                TokenType::Star => BinaryOp::Multiply,
                TokenType::Percent => BinaryOp::Modulo,
                _ => unreachable!(),
            };
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn unary(&mut self) -> Result<Expr, String> {
        if matches!(self.peek().token_type, TokenType::Bang | TokenType::Minus) {
            let operator = match self.advance().token_type {
                TokenType::Bang => UnaryOp::Not,
                TokenType::Minus => UnaryOp::Minus,
                _ => unreachable!(),
            };
            let operand = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                operand: Box::new(operand),
            });
        }
        
        self.primary()
    }
    
    fn primary(&mut self) -> Result<Expr, String> {
        match &self.advance().token_type {
            TokenType::True => Ok(Expr::Literal(Value::Boolean(true))),
            TokenType::False => Ok(Expr::Literal(Value::Boolean(false))),
            TokenType::Null => Ok(Expr::Literal(Value::Null)),
            TokenType::Number(n) => Ok(Expr::Literal(Value::Number(*n))),
            TokenType::String(s) => Ok(Expr::Literal(Value::String(s.clone()))),
            TokenType::Identifier(name) => Ok(Expr::Identifier(name.clone())),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                if !self.check(&TokenType::RightParen) {
                    return Err("Expected ')' after expression".to_string());
                }
                self.advance(); // consume ')'
                Ok(expr)
            }
            _ => Err("Expected expression".to_string()),
        }
    }
    
    fn consume_newline_or_eof(&mut self) -> Result<(), String> {
        if self.check(&TokenType::Newline) {
            self.advance();
            Ok(())
        } else if self.is_at_end() {
            Ok(())
        } else {
            Err("Expected newline or end of file".to_string())
        }
    }
    
    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(token_type)
        }
    }
    
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }
    
    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Eof)
    }
    
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
}
