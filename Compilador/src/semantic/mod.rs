use crate::parser::ast::{Expr, Program, Stmt, Type};
use crate::types::TypeSystem;
use std::collections::HashMap;
use anyhow::{Result, bail};

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    pub type_: Type,
    pub is_function: bool,
    pub params: Option<Vec<Type>>,
}

pub struct SemanticAnalyzer {
    symbols: HashMap<String, Symbol>,
    type_system: TypeSystem,
    current_function: Option<String>,
    current_return_type: Option<Type>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        let mut analyzer = SemanticAnalyzer {
            symbols: HashMap::new(),
            type_system: TypeSystem::new(),
            current_function: None,
            current_return_type: None,
        };
        
        // Built-in functions
        analyzer.add_builtin_function("print", vec![Type::Int], Type::Void);
        analyzer.add_builtin_function("print_string", vec![Type::String], Type::Void);
        analyzer.add_builtin_function("len", vec![Type::String], Type::Int);
        
        analyzer
    }

    fn add_builtin_function(&mut self, name: &str, params: Vec<Type>, return_type: Type) {
        self.symbols.insert(
            name.to_string(),
            Symbol {
                name: name.to_string(),
                type_: return_type,
                is_function: true,
                params: Some(params),
            },
        );
    }

    pub fn analyze(&mut self, program: &Program) -> Result<()> {
        // First pass: collect function declarations
        for stmt in &program.statements {
            if let Stmt::Function { name, params, return_type, .. } = stmt {
                let param_types: Vec<Type> = params.iter().map(|(_, t)| t.clone()).collect();
                self.symbols.insert(
                    name.clone(),
                    Symbol {
                        name: name.clone(),
                        type_: return_type.clone(),
                        is_function: true,
                        params: Some(param_types),
                    },
                );
            }
        }

        // Second pass: analyze all statements
        for stmt in &program.statements {
            self.analyze_statement(stmt)?;
        }

        Ok(())
    }

    fn analyze_statement(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::Let { name, type_annotation, value } => {
                let expr_type = self.analyze_expression(value)?;
                
                if let Some(annotated_type) = type_annotation {
                    if !self.type_system.is_compatible(&expr_type, annotated_type) {
                        bail!("Tipo incompatible en declaración de variable '{}'", name);
                    }
                }
                
                self.symbols.insert(
                    name.clone(),
                    Symbol {
                        name: name.clone(),
                        type_: type_annotation.clone().unwrap_or(expr_type),
                        is_function: false,
                        params: None,
                    },
                );
            }
            Stmt::Assign { target, value } => {
                if let Some(symbol) = self.symbols.get(target) {
                    let value_type = self.analyze_expression(value)?;
                    if !self.type_system.is_compatible(&value_type, &symbol.type_) {
                        bail!("Tipo incompatible en asignación a '{}'", target);
                    }
                } else {
                    bail!("Variable '{}' no declarada", target);
                }
            }
            Stmt::If { condition, then_block, else_block } => {
                let cond_type = self.analyze_expression(condition)?;
                if cond_type != Type::Bool {
                    bail!("Condición del if debe ser booleana");
                }
                
                for stmt in then_block {
                    self.analyze_statement(stmt)?;
                }
                
                if let Some(else_stmts) = else_block {
                    for stmt in else_stmts {
                        self.analyze_statement(stmt)?;
                    }
                }
            }
            Stmt::While { condition, body } => {
                let cond_type = self.analyze_expression(condition)?;
                if cond_type != Type::Bool {
                    bail!("Condición del while debe ser booleana");
                }
                
                for stmt in body {
                    self.analyze_statement(stmt)?;
                }
            }
            Stmt::For { init, condition, increment, body } => {
                self.analyze_statement(init)?;
                let cond_type = self.analyze_expression(condition)?;
                if cond_type != Type::Bool {
                    bail!("Condición del for debe ser booleana");
                }
                self.analyze_statement(increment)?;
                
                for stmt in body {
                    self.analyze_statement(stmt)?;
                }
            }
            Stmt::Function { name, params, return_type, body } => {
                self.current_function = Some(name.clone());
                self.current_return_type = Some(return_type.clone());
                
                // Add parameters to symbol table
                for (param_name, param_type) in params {
                    self.symbols.insert(
                        param_name.clone(),
                        Symbol {
                            name: param_name.clone(),
                            type_: param_type.clone(),
                            is_function: false,
                            params: None,
                        },
                    );
                }
                
                for stmt in body {
                    self.analyze_statement(stmt)?;
                }
                
                self.current_function = None;
                self.current_return_type = None;
            }
            Stmt::Return(Some(expr)) => {
                let expr_type = self.analyze_expression(expr)?;
                if let Some(expected_type) = &self.current_return_type {
                    if !self.type_system.is_compatible(&expr_type, expected_type) {
                        bail!("Tipo de retorno incompatible");
                    }
                }
            }
            Stmt::Return(None) => {
                if let Some(Type::Void) = &self.current_return_type {
                    // OK
                } else {
                    bail!("Función debe retornar un valor");
                }
            }
            Stmt::Expression(expr) => {
                self.analyze_expression(expr)?;
            }
            Stmt::Print(expr) => {
                self.analyze_expression(expr)?;
            }
        }
        Ok(())
    }

    fn analyze_expression(&mut self, expr: &Expr) -> Result<Type> {
        match expr {
            Expr::Number(_) => Ok(Type::Int),
            Expr::Boolean(_) => Ok(Type::Bool),
            Expr::String(_) => Ok(Type::String),
            Expr::Ident(name) => {
                if let Some(symbol) = self.symbols.get(name) {
                    if symbol.is_function {
                        bail!("'{}' es una función, no una variable", name);
                    }
                    Ok(symbol.type_.clone())
                } else {
                    bail!("Variable '{}' no declarada", name);
                }
            }
            Expr::ArrayLiteral(elements) => {
                if elements.is_empty() {
                    return Ok(Type::Array(Box::new(Type::Int))); // Default
                }
                
                let first_type = self.analyze_expression(&elements[0])?;
                for element in &elements[1..] {
                    let element_type = self.analyze_expression(element)?;
                    if !self.type_system.is_compatible(&element_type, &first_type) {
                        bail!("Elementos del array deben tener el mismo tipo");
                    }
                }
                Ok(Type::Array(Box::new(first_type)))
            }
            Expr::ArrayIndex { array, index } => {
                let array_type = self.analyze_expression(array)?;
                let index_type = self.analyze_expression(index)?;
                
                if index_type != Type::Int {
                    bail!("Índice de array debe ser entero");
                }
                
                match array_type {
                    Type::Array(inner_type) => Ok(*inner_type),
                    _ => bail!("No es un array"),
                }
            }
            Expr::Infix { left, op, right } => {
                let left_type = self.analyze_expression(left)?;
                let right_type = self.analyze_expression(right)?;
                
                match op.as_str() {
                    "+" | "-" | "*" | "/" => {
                        if left_type == Type::Int && right_type == Type::Int {
                            Ok(Type::Int)
                        } else if left_type == Type::String && op == "+" {
                            Ok(Type::String)
                        } else {
                            bail!("Operación aritmética inválida entre {:?} y {:?}", left_type, right_type)
                        }
                    }
                    "==" | "!=" | "<" | ">" | "<=" | ">=" => {
                        if self.type_system.is_comparable(&left_type, &right_type) {
                            Ok(Type::Bool)
                        } else {
                            bail!("No se pueden comparar {:?} y {:?}", left_type, right_type)
                        }
                    }
                    _ => bail!("Operador desconocido: {}", op),
                }
            }
            Expr::Call { function, args } => {
                if let Some(symbol) = self.symbols.get(function) {
                    if !symbol.is_function {
                        bail!("'{}' no es una función", function);
                    }
                    
                    if let Some(expected_params) = &symbol.params {
                        if args.len() != expected_params.len() {
                            bail!("Número incorrecto de argumentos para '{}'", function);
                        }
                        
                        for (arg, expected_type) in args.iter().zip(expected_params.iter()) {
                            let arg_type = self.analyze_expression(arg)?;
                            if !self.type_system.is_compatible(&arg_type, expected_type) {
                                bail!("Tipo de argumento incorrecto");
                            }
                        }
                    }
                    
                    Ok(symbol.type_.clone())
                } else {
                    bail!("Función '{}' no declarada", function);
                }
            }
            Expr::Grouped(expr) => self.analyze_expression(expr),
        }
    }
}
