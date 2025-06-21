use crate::core::{ast::*, InfraError, Result, Value};
use crate::frontend::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }
    
    pub fn parse(&mut self) -> Result<Program> {
        let mut program = Program::new();
        
        while !self.is_at_end() {
            // Skip newlines at the top level
            if self.check(&TokenType::Newline) {
                self.advance();
                continue;
            }
            
            program.add_statement(self.statement()?);
        }
        
        Ok(program)
    }
    
    fn statement(&mut self) -> Result<Stmt> {
        match &self.peek().token_type {
            TokenType::Let => self.let_statement(),
            TokenType::If => self.if_statement(),
            TokenType::While => self.while_statement(),
            TokenType::For => self.for_statement(),
            TokenType::Print => self.print_statement(),
            TokenType::LeftBrace => self.block_statement(),
            TokenType::Return => self.return_statement(),
            TokenType::Function | TokenType::Def => self.function_statement(),
            TokenType::Try => self.try_statement(),
            TokenType::Import => self.import_statement(),
            TokenType::Export => self.export_statement(),
            _ => {
                let expr = self.expression()?;
                
                // Check if this is an assignment
                if matches!(self.peek().token_type, TokenType::Equal) {
                    self.advance(); // consume '='
                    let value = self.expression()?;
                    self.consume_newline_or_eof()?;
                    
                    let target = match expr {
                        Expr::Identifier(name) => AssignmentTarget::Identifier(name),
                        Expr::Property { object, property } => AssignmentTarget::Property { object, property },
                        Expr::Index { object, index } => AssignmentTarget::Index { object, index },
                        _ => return Err(InfraError::ParseError { 
                            message: "Invalid assignment target".to_string(),
                            line: self.peek().line,
                            column: self.peek().column,
                        }),
                    };
                    
                    return Ok(Stmt::Assignment { target, value });
                }
                
                self.consume_newline_or_eof()?;
                Ok(Stmt::Expression(expr))
            }
        }
    }
    
    fn let_statement(&mut self) -> Result<Stmt> {
        self.advance(); // consume 'let'
        
        let name = self.consume_identifier("Expected variable name after 'let'")?;
        
        // Parse optional type annotation: let x: number = 5
        let type_annotation = self.parse_optional_type()?;
        
        self.consume(&TokenType::Equal, "Expected '=' after variable name")?;
        
        let value = self.expression()?;
        self.consume_newline_or_eof()?;
        
        Ok(Stmt::Let { 
            name, 
            type_annotation,
            value 
        })
    }
    
    fn if_statement(&mut self) -> Result<Stmt> {
        self.advance(); // consume 'if'
        
        let condition = self.expression()?;
        
        self.consume(&TokenType::Colon, "Expected ':' after if condition")?;
        self.skip_optional_newline();
        
        let then_stmt = Box::new(self.statement()?);
        
        // Skip any newlines before checking for else
        self.skip_optional_newline();
        
        let else_stmt = if self.check(&TokenType::Else) {
            self.advance(); // consume 'else'
            self.consume(&TokenType::Colon, "Expected ':' after 'else'")?;
            self.skip_optional_newline();
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
    
    fn while_statement(&mut self) -> Result<Stmt> {
        self.advance(); // consume 'while'
        
        let condition = self.expression()?;
        
        self.consume(&TokenType::Colon, "Expected ':' after while condition")?;
        self.skip_optional_newline();
        
        let body = Box::new(self.statement()?);
        
        Ok(Stmt::While { condition, body })
    }
    
    fn for_statement(&mut self) -> Result<Stmt> {
        self.advance(); // consume 'for'
        
        let var = self.consume_identifier("Expected variable name in for loop")?;
        
        self.consume(&TokenType::In, "Expected 'in' after for loop variable")?;
        self.consume(&TokenType::Range, "Expected 'range' after 'in'")?;
        self.consume(&TokenType::LeftParen, "Expected '(' after 'range'")?;
        
        let start = self.expression()?;
        
        self.consume(&TokenType::Comma, "Expected ',' in range")?;
        
        let end = self.expression()?;
        
        self.consume(&TokenType::RightParen, "Expected ')' after range")?;
        self.consume(&TokenType::Colon, "Expected ':' after for loop range")?;
        self.skip_optional_newline();
        
        let body = Box::new(self.statement()?);
        
        Ok(Stmt::For { var, start, end, body })
    }
    
    fn print_statement(&mut self) -> Result<Stmt> {
        self.advance(); // consume 'print'
        
        self.consume(&TokenType::LeftParen, "Expected '(' after 'print'")?;
        
        let expr = self.expression()?;
        
        self.consume(&TokenType::RightParen, "Expected ')' after print expression")?;
        self.consume_newline_or_eof()?;
        
        Ok(Stmt::Print(expr))
    }
    
    fn return_statement(&mut self) -> Result<Stmt> {
        self.advance(); // consume 'return'
        
        let value = if self.check(&TokenType::Newline) || self.is_at_end() {
            None
        } else {
            Some(self.expression()?)
        };
        
        self.consume_newline_or_eof()?;
        Ok(Stmt::Return(value))
    }
    
    fn block_statement(&mut self) -> Result<Stmt> {
        self.advance(); // consume '{'
        
        let mut statements = Vec::new();
        
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if self.check(&TokenType::Newline) {
                self.advance();
                continue;
            }
            statements.push(self.statement()?);
        }
        
        self.consume(&TokenType::RightBrace, "Expected '}' after block")?;
        
        Ok(Stmt::Block(statements))
    }
    
    fn function_statement(&mut self) -> Result<Stmt> {
        self.advance(); // consume 'function' or 'def'
        
        let name = self.consume_identifier("Expected function name")?;
        
        self.consume(&TokenType::LeftParen, "Expected '(' after function name")?;
        
        let mut params = Vec::new();
        let mut param_types = Vec::new();
        
        if !self.check(&TokenType::RightParen) {
            loop {
                let param = self.consume_identifier("Expected parameter name")?;
                params.push(param);
                
                // Parse optional parameter type: func(x: number, y: string)
                let param_type = self.parse_optional_type()?;
                param_types.push(param_type);
                
                if !self.check(&TokenType::Comma) {
                    break;
                }
                self.advance(); // consume ','
            }
        }
        
        self.consume(&TokenType::RightParen, "Expected ')' after parameters")?;
        
        // Parse optional return type: func() -> number:
        let return_type = if self.check(&TokenType::Arrow) {
            self.advance(); // consume '->'
            Some(self.parse_type()?)
        } else {
            None
        };
        
        self.consume(&TokenType::Colon, "Expected ':' after function signature")?;
        self.skip_optional_newline();
        
        let body = Box::new(self.statement()?);
        
        Ok(Stmt::Function { 
            name, 
            params, 
            param_types,
            return_type,
            body 
        })
    }
    
    fn try_statement(&mut self) -> Result<Stmt> {
        self.advance(); // consume 'try'
        self.consume(&TokenType::Colon, "Expected ':' after 'try'")?;
        self.skip_optional_newline();
        
        let try_block = Box::new(self.statement()?);
        
        self.skip_optional_newline();
        self.consume(&TokenType::Catch, "Expected 'catch' after try block")?;
        
        let catch_var = self.consume_identifier("Expected catch variable name")?;
        
        self.consume(&TokenType::Colon, "Expected ':' after catch variable")?;
        self.skip_optional_newline();
        
        let catch_block = Box::new(self.statement()?);
        
        Ok(Stmt::Try { try_block, catch_var, catch_block })
    }
    
    fn import_statement(&mut self) -> Result<Stmt> {
        self.advance(); // consume 'import'
        
        // Handle different import syntaxes:
        // import "module_path"
        // import {a, b} from "module_path"
        // import * from "module_path"
        // import module_name from "module_path"
        // import module_name as alias from "module_path"
        
        let mut alias = None;
        let items = if self.check(&TokenType::LeftBrace) {
            // import {a, b} from "module"
            self.advance(); // consume '{'
            let mut named_imports = Vec::new();
            
            while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
                let name = self.consume_identifier("Expected import name")?;
                let item_alias = if self.check(&TokenType::As) {
                    self.advance(); // consume 'as'
                    Some(self.consume_identifier("Expected alias name")?)
                } else {
                    None
                };
                named_imports.push(ImportItem { name, alias: item_alias });
                
                if self.check(&TokenType::Comma) {
                    self.advance();
                }
            }
            
            self.consume(&TokenType::RightBrace, "Expected '}' after import list")?;
            self.consume(&TokenType::From, "Expected 'from' after import list")?;
            ImportItems::Named(named_imports)
        } else if self.check(&TokenType::Star) {
            // import * from "module"
            self.advance(); // consume '*'
            self.consume(&TokenType::From, "Expected 'from' after '*'")?;
            ImportItems::All
        } else {
            // import module_name [as alias] from "module" OR import "module"
            if let TokenType::String(_) = self.peek().token_type {
                // Direct import: import "module"
                let module_path = self.consume_string("Expected module path")?;
                if self.check(&TokenType::As) {
                    self.advance(); // consume 'as'
                    alias = Some(self.consume_identifier("Expected alias name")?);
                }
                self.consume_newline_or_eof()?;
                
                return Ok(Stmt::Import {
                    module_path,
                    items: ImportItems::All,
                    alias,
                });
            } else {
                // import module_name [as alias] from "module"
                let default_name = self.consume_identifier("Expected module name")?;
                if self.check(&TokenType::As) {
                    self.advance(); // consume 'as'
                    alias = Some(self.consume_identifier("Expected alias name")?);
                }
                self.consume(&TokenType::From, "Expected 'from' after module name")?;
                ImportItems::Default(default_name)
            }
        };
        
        let module_path = self.consume_string("Expected module path")?;
        self.consume_newline_or_eof()?;
        
        Ok(Stmt::Import {
            module_path,
            items,
            alias,
        })
    }
    
    fn export_statement(&mut self) -> Result<Stmt> {
        self.advance(); // consume 'export'
        
        match &self.peek().token_type {
            TokenType::Function | TokenType::Def => {
                // export function name(params) { ... }
                let func_stmt = self.function_statement()?;
                if let Stmt::Function { name, params, body, .. } = func_stmt {
                    let param_count = params.len();
                    Ok(Stmt::Export {
                        item: ExportItem::Function { 
                            name, 
                            params, 
                            param_types: vec![None; param_count], // TODO: Parse parameter types
                            return_type: None,                    // TODO: Parse return type
                            body 
                        },
                    })
                } else {
                    unreachable!()
                }
            }
            TokenType::Let => {
                // export let name = value
                self.advance(); // consume 'let'
                let name = self.consume_identifier("Expected variable name")?;
                self.consume(&TokenType::Equal, "Expected '=' after variable name")?;
                let value = self.expression()?;
                self.consume_newline_or_eof()?;
                
                Ok(Stmt::Export {
                    item: ExportItem::Variable { 
                        name, 
                        type_annotation: None, // TODO: Parse variable type
                        value 
                    },
                })
            }
            _ => Err(InfraError::ParseError {
                message: "Expected 'function' or 'let' after 'export'".to_string(),
                line: self.peek().line,
                column: self.peek().column,
            }),
        }
    }
    
    fn expression(&mut self) -> Result<Expr> {
        self.or()
    }
    
    fn or(&mut self) -> Result<Expr> {
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
    
    fn and(&mut self) -> Result<Expr> {
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
    
    fn equality(&mut self) -> Result<Expr> {
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
    
    fn comparison(&mut self) -> Result<Expr> {
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
    
    fn term(&mut self) -> Result<Expr> {
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
    
    fn factor(&mut self) -> Result<Expr> {
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
    
    fn unary(&mut self) -> Result<Expr> {
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
        
        self.call()
    }
    
    fn call(&mut self) -> Result<Expr> {
        let mut expr = self.primary()?;
        
        loop {
            if self.check(&TokenType::LeftParen) {
                self.advance(); // consume '('
                
                let mut args = Vec::new();
                
                if !self.check(&TokenType::RightParen) {
                    loop {
                        args.push(self.expression()?);
                        
                        if !self.check(&TokenType::Comma) {
                            break;
                        }
                        self.advance(); // consume ','
                    }
                }
                
                self.consume(&TokenType::RightParen, "Expected ')' after arguments")?;
                
                expr = Expr::Call {
                    callee: Box::new(expr),
                    args,
                };
            } else if self.check(&TokenType::LeftBracket) {
                self.advance(); // consume '['
                
                let index = self.expression()?;
                
                self.consume(&TokenType::RightBracket, "Expected ']' after array index")?;
                
                expr = Expr::Index {
                    object: Box::new(expr),
                    index: Box::new(index),
                };
            } else if self.check(&TokenType::Dot) {
                self.advance(); // consume '.'
                
                let property = self.consume_identifier("Expected property name after '.'")?;
                
                // Check if this is module access (simple identifier on the left)
                if let Expr::Identifier(module_name) = &expr {
                    // For now, we'll assume all known module names should be treated as modules
                    // In a more sophisticated parser, we'd check against a known module list
                    if is_module_name(module_name) {
                        expr = Expr::ModuleAccess {
                            module: module_name.clone(),
                            function: property,
                        };
                    } else {
                        expr = Expr::Property {
                            object: Box::new(expr),
                            property,
                        };
                    }
                } else {
                    expr = Expr::Property {
                        object: Box::new(expr),
                        property,
                    };
                }
            } else {
                break;
            }
        }
        
        Ok(expr)
    }
    
    fn primary(&mut self) -> Result<Expr> {
        let token = self.advance();
        match &token.token_type {
            TokenType::True => Ok(Expr::Literal(Value::Boolean(true))),
            TokenType::False => Ok(Expr::Literal(Value::Boolean(false))),
            TokenType::Null => Ok(Expr::Literal(Value::Null)),
            TokenType::Number(n) => Ok(Expr::Literal(Value::Number(*n))),
            TokenType::String(s) => Ok(Expr::Literal(Value::String(s.clone()))),
            TokenType::Identifier(name) => Ok(Expr::Identifier(name.clone())),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(&TokenType::RightParen, "Expected ')' after expression")?;
                Ok(expr)
            }
            TokenType::LeftBracket => {
                // Array literal
                let mut elements = Vec::new();
                
                if !self.check(&TokenType::RightBracket) {
                    loop {
                        elements.push(self.expression()?);
                        
                        if !self.check(&TokenType::Comma) {
                            break;
                        }
                        self.advance(); // consume ','
                    }
                }
                
                self.consume(&TokenType::RightBracket, "Expected ']' after array elements")?;
                Ok(Expr::Array(elements))
            }
            TokenType::LeftBrace => {
                // Object literal - we've already consumed the '{'
                let mut properties = Vec::new();
                
                if !self.check(&TokenType::RightBrace) {
                    loop {
                        // Parse key (must be a string for now)
                        let key = match &self.peek().token_type {
                            TokenType::String(s) => {
                                let key = s.clone();
                                self.advance();
                                key
                            }
                            TokenType::Identifier(name) => {
                                let key = name.clone();
                                self.advance();
                                key
                            }
                            _ => return self.error("Expected property name"),
                        };
                        
                        self.consume(&TokenType::Colon, "Expected ':' after property name")?;
                        
                        let value = self.expression()?;
                        properties.push((key, value));
                        
                        if !self.check(&TokenType::Comma) {
                            break;
                        }
                        self.advance(); // consume ','
                    }
                }
                
                self.consume(&TokenType::RightBrace, "Expected '}' after object properties")?;
                Ok(Expr::Object(properties))
            }
            _ => self.error("Expected expression"),
        }
    }
    
    // Type parsing methods
    fn parse_type(&mut self) -> Result<Type> {
        let base_type = self.parse_base_type()?;
        
        // Check for union types (pipe operator)
        if self.check(&TokenType::Pipe) {
            let mut types = vec![base_type];
            
            while self.check(&TokenType::Pipe) {
                self.advance(); // consume '|'
                types.push(self.parse_base_type()?);
            }
            
            // If we only have one type, don't create a union
            if types.len() == 1 {
                Ok(types.into_iter().next().unwrap())
            } else {
                Ok(Type::Union(types))
            }
        } else {
            Ok(base_type)
        }
    }
    
    fn parse_base_type(&mut self) -> Result<Type> {
        match &self.peek().token_type {
            TokenType::NumberType => {
                self.advance();
                Ok(Type::Number)
            }
            TokenType::StringType => {
                self.advance();
                Ok(Type::String)
            }
            TokenType::BooleanType => {
                self.advance();
                Ok(Type::Boolean)
            }
            TokenType::LeftBracket => {
                self.advance(); // consume '['
                let element_type = Box::new(self.parse_type()?);
                self.consume(&TokenType::RightBracket, "Expected ']' after array element type")?;
                Ok(Type::Array(element_type))
            }
            TokenType::LeftBrace => {
                self.advance(); // consume '{'
                let mut properties = Vec::new();
                
                if !self.check(&TokenType::RightBrace) {
                    loop {
                        let property_name = self.consume_identifier("Expected property name")?;
                        self.consume(&TokenType::Colon, "Expected ':' after property name")?;
                        let property_type = self.parse_type()?;
                        properties.push((property_name, property_type));
                        
                        if !self.check(&TokenType::Comma) {
                            break;
                        }
                        self.advance(); // consume ','
                    }
                }
                
                self.consume(&TokenType::RightBrace, "Expected '}' after object type")?;
                Ok(Type::Object(properties))
            }
            TokenType::LeftParen => {
                // Function type: (param_types) -> return_type
                self.advance(); // consume '('
                let mut param_types = Vec::new();
                
                if !self.check(&TokenType::RightParen) {
                    loop {
                        param_types.push(self.parse_type()?);
                        
                        if !self.check(&TokenType::Comma) {
                            break;
                        }
                        self.advance(); // consume ','
                    }
                }
                
                self.consume(&TokenType::RightParen, "Expected ')' after function parameter types")?;
                self.consume(&TokenType::Arrow, "Expected '->' after function parameter types")?;
                let return_type = Box::new(self.parse_type()?);
                
                Ok(Type::Function {
                    params: param_types,
                    return_type,
                })
            }
            _ => {
                // Default to Any type for unrecognized types
                // This allows for graceful degradation
                Ok(Type::Any)
            }
        }
    }
    
    fn parse_optional_type(&mut self) -> Result<Option<Type>> {
        if self.check(&TokenType::Colon) {
            self.advance(); // consume ':'
            Ok(Some(self.parse_type()?))
        } else {
            Ok(None)
        }
    }

    // Helper methods
    fn consume_identifier(&mut self, message: &str) -> Result<String> {
        match &self.peek().token_type {
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            _ => self.error(message),
        }
    }
    
    fn consume_string(&mut self, message: &str) -> Result<String> {
        match &self.peek().token_type {
            TokenType::String(value) => {
                let value = value.clone();
                self.advance();
                Ok(value)
            }
            _ => self.error(message),
        }
    }
    
    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<()> {
        if self.check(token_type) {
            self.advance();
            Ok(())
        } else {
            self.error(message)
        }
    }
    
    fn skip_optional_newline(&mut self) {
        if self.check(&TokenType::Newline) {
            self.advance();
        }
    }
    
    fn consume_newline_or_eof(&mut self) -> Result<()> {
        if self.check(&TokenType::Newline) {
            self.advance();
            Ok(())
        } else if self.is_at_end() {
            Ok(())
        } else {
            self.error("Expected newline or end of file")
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
        self.peek().is_eof()
    }
    
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    
    fn error<T>(&self, message: &str) -> Result<T> {
        let token = self.peek();
        Err(InfraError::ParseError {
            message: message.to_string(),
            line: token.line,
            column: token.column,
        })
    }
}

// Helper function to check if an identifier is a known module name
fn is_module_name(name: &str) -> bool {
    matches!(name, "math" | "string" | "array" | "io")
}
