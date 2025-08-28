use crate::ir::{IRFunction, IROp, IRProgram, IRValue};
use crate::parser::ast::{Expr, Program, Stmt, Type};
use std::collections::HashMap;

pub struct IRBuilder {
    functions: Vec<IRFunction>,
    current_function: Option<String>,
    temp_counter: usize,
    label_counter: usize,
    string_literals: HashMap<String, String>,
    string_counter: usize,
}

impl IRBuilder {
    pub fn new() -> Self {
        IRBuilder {
            functions: Vec::new(),
            current_function: None,
            temp_counter: 0,
            label_counter: 0,
            string_literals: HashMap::new(),
            string_counter: 0,
        }
    }

    pub fn build(&mut self, program: &Program) -> IRProgram {
        for stmt in &program.statements {
            self.build_statement(stmt);
        }

        IRProgram {
            functions: self.functions.clone(),
            globals: HashMap::new(),
        }
    }

    fn build_statement(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Function { name, params, body, .. } => {
                self.current_function = Some(name.clone());
                let mut function = IRFunction {
                    name: name.clone(),
                    params: params.iter().map(|(name, _)| name.clone()).collect(),
                    instructions: Vec::new(),
                    locals: HashMap::new(),
                };

                // Build function body
                for body_stmt in body {
                    self.build_function_statement(&mut function, body_stmt);
                }

                self.functions.push(function);
                self.current_function = None;
            }
            _ => {
                // Global statements go to main function
                if let Some(main_func) = self.functions.iter_mut().find(|f| f.name == "main") {
                    self.build_function_statement(main_func, stmt);
                }
            }
        }
    }

    fn build_function_statement(&mut self, function: &mut IRFunction, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, value, .. } => {
                let value_result = self.build_expression(function, value);
                let local_var = IRValue::Local(name.clone());
                function.instructions.push(IROp::Assign(local_var, value_result));
                function.locals.insert(name.clone(), local_var);
            }
            Stmt::If { condition, then_block, else_block } => {
                let cond_result = self.build_expression(function, condition);
                let else_label = self.new_label();
                let end_label = self.new_label();

                function.instructions.push(IROp::JumpIfZero(cond_result, else_label.clone()));

                // Then block
                for then_stmt in then_block {
                    self.build_function_statement(function, then_stmt);
                }
                function.instructions.push(IROp::Jump(end_label.clone()));

                // Else block
                function.instructions.push(IROp::Label(else_label));
                if let Some(else_stmts) = else_block {
                    for else_stmt in else_stmts {
                        self.build_function_statement(function, else_stmt);
                    }
                }

                function.instructions.push(IROp::Label(end_label));
            }
            Stmt::While { condition, body } => {
                let start_label = self.new_label();
                let end_label = self.new_label();

                function.instructions.push(IROp::Label(start_label.clone()));
                let cond_result = self.build_expression(function, condition);
                function.instructions.push(IROp::JumpIfZero(cond_result, end_label.clone()));

                for body_stmt in body {
                    self.build_function_statement(function, body_stmt);
                }
                function.instructions.push(IROp::Jump(start_label));
                function.instructions.push(IROp::Label(end_label));
            }
            Stmt::Return(Some(expr)) => {
                let result = self.build_expression(function, expr);
                function.instructions.push(IROp::Return(Some(result)));
            }
            Stmt::Return(None) => {
                function.instructions.push(IROp::Return(None));
            }
            Stmt::Print(expr) => {
                let result = self.build_expression(function, expr);
                function.instructions.push(IROp::Print(result));
            }
            _ => {}
        }
    }

    fn build_expression(&mut self, function: &mut IRFunction, expr: &Expr) -> IRValue {
        match expr {
            Expr::Number(n) => IRValue::Const(*n),
            Expr::Boolean(b) => IRValue::Const(if *b { 1 } else { 0 }),
            Expr::String(s) => {
                let string_name = format!("str_{}", self.string_counter);
                self.string_counter += 1;
                self.string_literals.insert(string_name.clone(), s.clone());
                IRValue::Global(string_name)
            }
            Expr::Ident(name) => {
                if let Some(local) = function.locals.get(name) {
                    local.clone()
                } else {
                    IRValue::Global(name.clone())
                }
            }
            Expr::Infix { left, op, right } => {
                let left_result = self.build_expression(function, left);
                let right_result = self.build_expression(function, right);
                let temp = self.new_temp();

                let op_instruction = match op.as_str() {
                    "+" => IROp::Add(temp.clone(), left_result, right_result),
                    "-" => IROp::Sub(temp.clone(), left_result, right_result),
                    "*" => IROp::Mul(temp.clone(), left_result, right_result),
                    "/" => IROp::Div(temp.clone(), left_result, right_result),
                    "==" => IROp::CmpEq(temp.clone(), left_result, right_result),
                    "<" => IROp::CmpLt(temp.clone(), left_result, right_result),
                    _ => panic!("Operador no soportado: {}", op),
                };

                function.instructions.push(op_instruction);
                temp
            }
            Expr::Call { function: func_name, args } => {
                let arg_values: Vec<IRValue> = args
                    .iter()
                    .map(|arg| self.build_expression(function, arg))
                    .collect();
                let result = self.new_temp();
                function.instructions.push(IROp::Call(
                    func_name.clone(),
                    arg_values,
                    Some(result.clone()),
                ));
                result
            }
            _ => IRValue::Const(0), // Default
        }
    }

    fn new_temp(&mut self) -> IRValue {
        let temp_name = format!("t{}", self.temp_counter);
        self.temp_counter += 1;
        IRValue::Temp(temp_name)
    }

    fn new_label(&mut self) -> String {
        let label_name = format!("label_{}", self.label_counter);
        self.label_counter += 1;
        label_name
    }
}
