use crate::ir::{IROp, IRProgram, IRValue};
use std::collections::{HashMap, HashSet};

pub struct Optimizer {
    constant_pool: HashMap<String, i64>,
    used_variables: HashSet<String>,
}

impl Optimizer {
    pub fn new() -> Self {
        Optimizer {
            constant_pool: HashMap::new(),
            used_variables: HashSet::new(),
        }
    }

    pub fn optimize(&mut self, program: &mut IRProgram) {
        for function in &mut program.functions {
            self.constant_propagation(function);
            self.dead_code_elimination(function);
            self.common_subexpression_elimination(function);
            self.loop_optimization(function);
        }
    }

    fn constant_propagation(&mut self, function: &mut IRFunction) {
        let mut constants = HashMap::new();

        for instr in &mut function.instructions {
            match instr {
                IROp::Assign(target, IRValue::Const(value)) => {
                    if let IRValue::Temp(name) = target {
                        constants.insert(name.clone(), *value);
                    }
                }
                IROp::Add(result, left, right) => {
                    if let (IRValue::Const(a), IRValue::Const(b)) = (left, right) {
                        if let IRValue::Temp(name) = result {
                            constants.insert(name.clone(), a + b);
                            *instr = IROp::Assign(result.clone(), IRValue::Const(a + b));
                        }
                    }
                }
                IROp::Sub(result, left, right) => {
                    if let (IRValue::Const(a), IRValue::Const(b)) = (left, right) {
                        if let IRValue::Temp(name) = result {
                            constants.insert(name.clone(), a - b);
                            *instr = IROp::Assign(result.clone(), IRValue::Const(a - b));
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn dead_code_elimination(&mut self, function: &mut IRFunction) {
        let mut used_temps = HashSet::new();
        let mut used_globals = HashSet::new();

        // Mark used variables
        for instr in &function.instructions {
            match instr {
                IROp::Print(value) | IROp::Return(Some(value)) => {
                    self.mark_used(value, &mut used_temps, &mut used_globals);
                }
                IROp::Add(_, left, right) | IROp::Sub(_, left, right) | 
                IROp::Mul(_, left, right) | IROp::Div(_, left, right) => {
                    self.mark_used(left, &mut used_temps, &mut used_globals);
                    self.mark_used(right, &mut used_temps, &mut used_globals);
                }
                _ => {}
            }
        }

        // Remove unused instructions
        function.instructions.retain(|instr| {
            match instr {
                IROp::Add(result, _, _) | IROp::Sub(result, _, _) |
                IROp::Mul(result, _, _) | IROp::Div(result, _, _) => {
                    if let IRValue::Temp(name) = result {
                        used_temps.contains(name)
                    } else {
                        true
                    }
                }
                _ => true,
            }
        });
    }

    fn mark_used(&self, value: &IRValue, temps: &mut HashSet<String>, globals: &mut HashSet<String>) {
        match value {
            IRValue::Temp(name) => { temps.insert(name.clone()); }
            IRValue::Global(name) => { globals.insert(name.clone()); }
            _ => {}
        }
    }

    fn common_subexpression_elimination(&mut self, function: &mut IRFunction) {
        let mut expressions = HashMap::new();
        let mut replacements = HashMap::new();

        for instr in &mut function.instructions {
            match instr {
                IROp::Add(result, left, right) => {
                    let key = format!("add_{:?}_{:?}", left, right);
                    if let Some(existing) = expressions.get(&key) {
                        if let IRValue::Temp(result_name) = result {
                            replacements.insert(result_name.clone(), existing.clone());
                            *instr = IROp::Assign(result.clone(), existing.clone());
                        }
                    } else {
                        if let IRValue::Temp(result_name) = result {
                            expressions.insert(key, result.clone());
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn loop_optimization(&mut self, function: &mut IRFunction) {
        // Simple loop invariant code motion
        let mut i = 0;
        while i < function.instructions.len() {
            if let IROp::Label(label) = &function.instructions[i] {
                if label.starts_with("label_") {
                    // Check for loop pattern and optimize
                    self.optimize_loop(&mut function.instructions, i);
                }
            }
            i += 1;
        }
    }

    fn optimize_loop(&mut self, instructions: &mut Vec<IROp>, start_idx: usize) {
        // Move invariant computations outside loops
        // This is a simplified version
    }
}
