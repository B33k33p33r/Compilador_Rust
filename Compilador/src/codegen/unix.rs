use crate::ir::{IRFunction, IROp, IRProgram, IRValue};

pub fn generate_unix_asm(program: IRProgram) -> String {
    let mut output = String::new();
    
    // Header
    output.push_str("section .text\n");
    output.push_str("extern print_int\n");
    output.push_str("global _start\n\n");
    
    // Generate functions
    for func in program.functions {
        output.push_str(&format!("{}:\n", func.name));
        output.push_str("    push rbp\n");
        output.push_str("    mov rbp, rsp\n");
        
        // Allocate stack space for locals
        let local_count = func.locals.len() as i64;
        if local_count > 0 {
            output.push_str(&format!("    sub rsp, {}\n", local_count * 8));
        }
        
        // Generate instructions
        for instr in func.instructions {
            output.push_str(&generate_instruction(&instr));
        }
        
        output.push_str("    mov rsp, rbp\n");
        output.push_str("    pop rbp\n");
        output.push_str("    ret\n\n");
    }
    
    // Main entry point
    output.push_str("_start:\n");
    output.push_str("    call main\n");
    output.push_str("    mov rax, 60\n"); // sys_exit
    output.push_str("    mov rdi, 0\n");
    output.push_str("    syscall\n");
    
    output
}

fn generate_instruction(instr: &IROp) -> String {
    match instr {
        IROp::Add(result, left, right) => {
            format!("    mov rax, {}\n    add rax, {}\n    mov {}, rax\n",
                    ir_value_to_asm(left),
                    ir_value_to_asm(right),
                    ir_value_to_asm(result))
        }
        IROp::Sub(result, left, right) => {
            format!("    mov rax, {}\n    sub rax, {}\n    mov {}, rax\n",
                    ir_value_to_asm(left),
                    ir_value_to_asm(right),
                    ir_value_to_asm(result))
        }
        IROp::Mul(result, left, right) => {
            format!("    mov rax, {}\n    mov rbx, {}\n    imul rax, rbx\n    mov {}, rax\n",
                    ir_value_to_asm(left),
                    ir_value_to_asm(right),
                    ir_value_to_asm(result))
        }
        IROp::Div(result, left, right) => {
            format!("    mov rax, {}\n    mov rbx, {}\n    cqo\n    idiv rbx\n    mov {}, rax\n",
                    ir_value_to_asm(left),
                    ir_value_to_asm(right),
                    ir_value_to_asm(result))
        }
        IROp::Assign(target, source) => {
            format!("    mov rax, {}\n    mov {}, rax\n",
                    ir_value_to_asm(source),
                    ir_value_to_asm(target))
        }
        IROp::Print(value) => {
            format!("    mov rdi, {}\n    call print_int\n",
                    ir_value_to_asm(value))
        }
        IROp::Label(name) => {
            format!("{}:\n", name)
        }
        IROp::Jump(label) => {
            format!("    jmp {}\n", label)
        }
        IROp::JumpIfZero(value, label) => {
            format!("    cmp {}, 0\n    je {}\n",
                    ir_value_to_asm(value),
                    label)
        }
        IROp::JumpIfNotZero(value, label) => {
            format!("    cmp {}, 0\n    jne {}\n",
                    ir_value_to_asm(value),
                    label)
        }
        IROp::Return(Some(value)) => {
            format!("    mov rax, {}\n    mov rsp, rbp\n    pop rbp\n    ret\n",
                    ir_value_to_asm(value))
        }
        IROp::Return(None) => {
            "    mov rsp, rbp\n    pop rbp\n    ret\n".to_string()
        }
        _ => String::new(),
    }
}

fn ir_value_to_asm(value: &IRValue) -> String {
    match value {
        IRValue::Const(n) => n.to_string(),
        IRValue::Local(name) => format!("[rbp - {}]", get_local_offset(name) * 8),
        IRValue::Global(name) => format!("[{}]", name),
        IRValue::Temp(name) => format!("rax"), // Simplified
    }
}

fn get_local_offset(name: &str) -> usize {
    // This would be managed by the codegen context
    name.chars().last().unwrap_or('0') as usize - '0' as usize
}
