use crate::ir::{IRFunction, IROp, IRProgram, IRValue};

pub fn generate_windows_asm(program: IRProgram) -> String {
    let mut output = String::new();
    
    // Header for Windows
    output.push_str("section .text\n");
    output.push_str("extern printf\n");
    output.push_str("global main\n\n");
    
    // Generate functions
    for func in program.functions {
        output.push_str(&format!("{}:\n", func.name));
        output.push_str("    push rbp\n");
        output.push_str("    mov rbp, rsp\n");
        
        // Windows calling convention
        // RCX, RDX, R8, R9 for first 4 args, rest on stack
        
        // Generate instructions
        for instr in func.instructions {
            output.push_str(&generate_instruction(&instr));
        }
        
        output.push_str("    mov rsp, rbp\n");
        output.push_str("    pop rbp\n");
        output.push_str("    ret\n\n");
    }
    
    // Main entry point
    output.push_str("main:\n");
    output.push_str("    sub rsp, 40\n"); // Shadow space + alignment
    output.push_str("    call main_func\n");
    output.push_str("    add rsp, 40\n");
    output.push_str("    ret\n");
    
    output
}

// Similar implementation to unix.rs but with Windows conventions
fn generate_instruction(instr: &IROp) -> String {
    // Implementation similar to unix.rs but adjusted for Windows
    // For brevity, using the same logic but you'd adjust for Windows specifics
    match instr {
        IROp::Add(result, left, right) => {
            format!("    mov rax, {}\n    add rax, {}\n    mov {}, rax\n",
                    ir_value_to_asm(left),
                    ir_value_to_asm(right),
                    ir_value_to_asm(result))
        }
        // ... other instructions
        _ => String::new(),
    }
}

fn ir_value_to_asm(value: &IRValue) -> String {
    // Same as unix version
    match value {
        IRValue::Const(n) => n.to_string(),
        IRValue::Local(name) => format!("[rbp - {}]", get_local_offset(name) * 8),
        IRValue::Global(name) => format!("[{}]", name),
        IRValue::Temp(name) => format!("rax"),
    }
}

fn get_local_offset(name: &str) -> usize {
    name.chars().last().unwrap_or('0') as usize - '0' as usize
}
