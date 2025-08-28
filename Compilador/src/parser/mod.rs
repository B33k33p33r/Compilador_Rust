use crate::lexer::{token::Token, Lexer};
use crate::parser::ast::{Expr, Program, Stmt, Type};
use anyhow::Result;

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let cur_token = lexer.clone().next_token().unwrap();
        Parser { lexer, cur_token }
    }

    fn next_token(&mut self) -> Result<()> {
        self.cur_token = self.lexer.next_token()?;
        Ok(())
    }

    pub fn parse_program(&mut self) -> Result<Program> {
        let mut statements = Vec::new();

        while self.cur_token != Token::Eof {
            if let Some(stmt) = self.parse_statement()? {
                statements.push(stmt);
            }
            self.next_token()?;
        }

        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Option<Stmt>> {
        match &self.cur_token {
            Token::Let => self.parse_let_statement(),
            Token::If => self.parse_if_statement(),
            Token::While => self.parse_while_statement(),
            Token::For => self.parse_for_statement(),
            Token::Fn => self.parse_function_statement(),
            Token::Return => self.parse_return_statement(),
            Token::Print => self.parse_print_statement(),
            Token::Ident(_) => {
                // Podría ser asignación o expresión
                let expr = self.parse_expression(0)?;
                self.expect_token(Token::Semicolon)?;
                Ok(Some(Stmt::Expression(expr)))
            }
            Token::Eof => Ok(None),
            _ => {
                let expr = self.parse_expression(0)?;
                self.expect_token(Token::Semicolon)?;
                Ok(Some(Stmt::Expression(expr)))
            }
        }
    }

    fn parse_let_statement(&mut self) -> Result<Option<Stmt>> {
        self.next_token()?; // skip 'let'
        
        if let Token::Ident(name) = &self.cur_token {
            let var_name = name.clone();
            self.next_token()?; // skip ident
            
            let mut type_annotation = None;
            if self.cur_token == Token::Colon {
                self.next_token()?; // skip ':'
                type_annotation = Some(self.parse_type()?);
                self.next_token()?;
            }
            
            self.expect_token(Token::Eq)?;
            self.next_token()?;
            let expr = self.parse_expression(0)?;
            self.expect_token(Token::Semicolon)?;
            
            Ok(Some(Stmt::Let {
                name: var_name,
                type_annotation,
                value: expr,
            }))
        } else {
            Err(anyhow::anyhow!("Se esperaba identificador después de 'let'"))
        }
    }

    fn parse_if_statement(&mut self) -> Result<Option<Stmt>> {
        self.next_token()?; // skip 'if'
        self.expect_token(Token::LParen)?;
        self.next_token()?;
        let condition = self.parse_expression(0)?;
        self.expect_token(Token::RParen)?;
        self.next_token()?;
        
        let then_block = self.parse_block()?;
        
        let else_block = if self.cur_token == Token::Else {
            self.next_token()?;
            Some(self.parse_block()?)
        } else {
            None
        };
        
        Ok(Some(Stmt::If {
            condition,
            then_block,
            else_block,
        }))
    }

    fn parse_while_statement(&mut self) -> Result<Option<Stmt>> {
        self.next_token()?; // skip 'while'
        self.expect_token(Token::LParen)?;
        self.next_token()?;
        let condition = self.parse_expression(0)?;
        self.expect_token(Token::RParen)?;
        self.next_token()?;
        
        let body = self.parse_block()?;
        
        Ok(Some(Stmt::While { condition, body }))
    }

    fn parse_for_statement(&mut self) -> Result<Option<Stmt>> {
        self.next_token()?; // skip 'for'
        self.expect_token(Token::LParen)?;
        self.next_token()?;
        
        let init = Box::new(self.parse_statement()?.unwrap());
        self.next_token()?;
        
        let condition = self.parse_expression(0)?;
        self.expect_token(Token::Semicolon)?;
        self.next_token()?;
        
        let increment = Box::new(self.parse_statement()?.unwrap());
        self.expect_token(Token::RParen)?;
        self.next_token()?;
        
        let body = self.parse_block()?;
        
        Ok(Some(Stmt::For {
            init,
            condition,
            increment,
            body,
        }))
    }

    fn parse_function_statement(&mut self) -> Result<Option<Stmt>> {
        self.next_token()?; // skip 'fn'
        
        if let Token::Ident(name) = &self.cur_token {
            let func_name = name.clone();
            self.next_token()?; // skip name
            self.expect_token(Token::LParen)?;
            self.next_token()?;
            
            let mut params = Vec::new();
            while self.cur_token != Token::RParen {
                if let Token::Ident(param_name) = &self.cur_token {
                    let name = param_name.clone();
                    self.next_token()?;
                    self.expect_token(Token::Colon)?;
                    self.next_token()?;
                    let param_type = self.parse_type()?;
                    params.push((name, param_type));
                    
                    if self.cur_token == Token::Comma {
                        self.next_token()?;
                    }
                } else {
                    break;
                }
            }
            
            self.expect_token(Token::RParen)?;
            self.next_token()?;
            
            let return_type = if self.cur_token == Token::Colon {
                self.next_token()?;
                self.next_token()?;
                self.parse_type()?
            } else {
                Type::Void
            };
            
            let body = self.parse_block()?;
            
            Ok(Some(Stmt::Function {
                name: func_name,
                params,
                return_type,
                body,
            }))
        } else {
            Err(anyhow::anyhow!("Se esperaba nombre de función"))
        }
    }

    fn parse_return_statement(&mut self) -> Result<Option<Stmt>> {
        self.next_token()?; // skip 'return'
        
        if self.cur_token == Token::Semicolon {
            Ok(Some(Stmt::Return(None)))
        } else {
            let expr = self.parse_expression(0)?;
            self.expect_token(Token::Semicolon)?;
            Ok(Some(Stmt::Return(Some(expr))))
        }
    }

    fn parse_print_statement(&mut self) -> Result<Option<Stmt>> {
        self.next_token()?; // skip 'print'
        self.expect_token(Token::LParen)?;
        self.next_token()?;
        let expr = self.parse_expression(0)?;
        self.expect_token(Token::RParen)?;
        self.next_token()?;
        self.expect_token(Token::Semicolon)?;
        Ok(Some(Stmt::Print(expr)))
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>> {
        self.expect_token(Token::LBrace)?;
        self.next_token()?;
        
        let mut statements = Vec::new();
        while self.cur_token != Token::RBrace && self.cur_token != Token::Eof {
            if let Some(stmt) = self.parse_statement()? {
                statements.push(stmt);
            }
            self.next_token()?;
        }
        
        self.expect_token(Token::RBrace)?;
        Ok(statements)
    }

    fn parse_type(&mut self) -> Result<Type> {
        match &self.cur_token {
            Token::Ident(name) => {
                match name.as_str() {
                    "int" => Ok(Type::Int),
                    "bool" => Ok(Type::Bool),
                    "string" => Ok(Type::String),
                    "void" => Ok(Type::Void),
                    _ => Err(anyhow::anyhow!("Tipo desconocido: {}", name)),
                }
            }
            Token::LBracket => {
                self.next_token()?; // skip '['
                let inner_type = self.parse_type()?;
                self.expect_token(Token::RBracket)?;
                Ok(Type::Array(Box::new(inner_type)))
            }
            _ => Err(anyhow::anyhow!("Se esperaba tipo")),
        }
    }

    fn parse_expression(&mut self, precedence: u8) -> Result<Expr> {
        let mut left = self.parse_primary()?;
        
        while precedence < self.current_precedence() {
            left = match self.cur_token {
                Token::Plus | Token::Minus | Token::Star | Token::Slash |
                Token::EqEq | Token::NotEq | Token::Lt | Token::Gt |
                Token::LtEq | Token::GtEq => {
                    let op = self.current_op().unwrap();
                    self.next_token()?;
                    let right = self.parse_expression(self.current_precedence())?;
                    Expr::Infix {
                        left: Box::new(left),
                        op,
                        right: Box::new(right),
                    }
                }
                Token::LParen => {
                    self.next_token()?; // skip '('
                    let mut args = Vec::new();
                    while self.cur_token != Token::RParen {
                        args.push(self.parse_expression(0)?);
                        if self.cur_token == Token::Comma {
                            self.next_token()?;
                        }
                    }
                    self.expect_token(Token::RParen)?;
                    if let Expr::Ident(name) = left {
                        Expr::Call { function: name, args }
                    } else {
                        return Err(anyhow::anyhow!("Solo se pueden llamar funciones"));
                    }
                }
                Token::LBracket => {
                    self.next_token()?; // skip '['
                    let index = self.parse_expression(0)?;
                    self.expect_token(Token::RBracket)?;
                    Expr::ArrayIndex {
                        array: Box::new(left),
                        index: Box::new(index),
                    }
                }
                _ => break,
            };
        }
        
        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expr> {
        match &self.cur_token {
            Token::Number(n) => {
                self.next_token()?;
                Ok(Expr::Number(*n))
            }
            Token::True => {
                self.next_token()?;
                Ok(Expr::Boolean(true))
            }
            Token::False => {
                self.next_token()?;
                Ok(Expr::Boolean(false))
            }
            Token::String(s) => {
                let str_val = s.clone();
                self.next_token()?;
                Ok(Expr::String(str_val))
            }
            Token::Ident(s) => {
                let ident = s.clone();
                self.next_token()?;
                if self.cur_token == Token::LParen {
                    // Es una llamada a función
                    self.next_token()?; // skip '('
                    let mut args = Vec::new();
                    while self.cur_token != Token::RParen {
                        args.push(self.parse_expression(0)?);
                        if self.cur_token == Token::Comma {
                            self.next_token()?;
                        }
                    }
                    self.expect_token(Token::RParen)?;
                    self.next_token()?;
                    Ok(Expr::Call { function: ident, args })
                } else {
                    Ok(Expr::Ident(ident))
                }
            }
            Token::LBracket => {
                self.next_token()?; // skip '['
                let mut elements = Vec::new();
                while self.cur_token != Token::RBracket {
                    elements.push(self.parse_expression(0)?);
                    if self.cur_token == Token::Comma {
                        self.next_token()?;
                    }
                }
                self.expect_token(Token::RBracket)?;
                self.next_token()?;
                Ok(Expr::ArrayLiteral(elements))
            }
            Token::LParen => {
                self.next_token()?; // skip '('
                let expr = self.parse_expression(0)?;
                self.expect_token(Token::RParen)?;
                self.next_token()?;
                Ok(Expr::Grouped(Box::new(expr)))
            }
            _ => Err(anyhow::anyhow!("Expresión no válida: {:?}", self.cur_token)),
        }
    }

    fn expect_token(&mut self, expected: Token) -> Result<()> {
        if self.cur_token == expected {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Se esperaba {:?}, encontrado {:?}",
                expected,
                self.cur_token
            ))
        }
    }

    fn current_op(&self) -> Option<String> {
        match &self.cur_token {
            Token::Plus => Some("+".to_string()),
            Token::Minus => Some("-".to_string()),
            Token::Star => Some("*".to_string()),
            Token::Slash => Some("/".to_string()),
            Token::EqEq => Some("==".to_string()),
            Token::NotEq => Some("!=".to_string()),
            Token::Lt => Some("<".to_string()),
            Token::Gt => Some(">".to_string()),
            Token::LtEq => Some("<=".to_string()),
            Token::GtEq => Some(">=".to_string()),
            _ => None,
        }
    }

    fn current_precedence(&self) -> u8 {
        match &self.cur_token {
            Token::EqEq | Token::NotEq | Token::Lt | Token::Gt | Token::LtEq | Token::GtEq => 1,
            Token::Plus | Token::Minus => 2,
            Token::Star | Token::Slash => 3,
            _ => 0,
        }
    }
}


