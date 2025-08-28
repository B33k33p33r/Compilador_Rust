mod lexer;
mod parser;
mod semantic;
mod ir;
mod optimizer;
mod codegen;
mod runtime;
mod types;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::semantic::SemanticAnalyzer;
use crate::ir::builder::IRBuilder;
use crate::optimizer::Optimizer;
use crate::codegen::generate_code;
use crate::runtime::generate_runtime;
use target_lexicon::HOST;
use std::env;
use std::fs;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Uso: {} <archivo_fuente> <archivo_salida>", args[0]);
        std::process::exit(1);
    }

    let source_file = &args[1];
    let output_file = &args[2];
    
    // Leer código fuente
    let source_code = fs::read_to_string(source_file)?;
    
    // Etapa 1: Lexical Analysis
    let lexer = Lexer::new(source_code);
    
    // Etapa 2: Parsing
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program()?;
    
    // Etapa 3: Semantic Analysis
    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze(&program)?;
    
    // Etapa 4: IR Generation
    let mut ir_builder = IRBuilder::new();
    let mut ir_program = ir_builder.build(&program);
    
    // Etapa 5: Optimization
    let mut optimizer = Optimizer::new();
    optimizer.optimize(&mut ir_program);
    
    // Etapa 6: Code Generation
    let asm_code = generate_code(ir_program, HOST.operating_system);
    
    // Etapa 7: Runtime Generation
    let runtime_code = generate_runtime(HOST.operating_system);
    
    // Escribir archivos de salida
    fs::write(format!("{}.s", output_file), asm_code)?;
    fs::write(format!("{}_runtime.c", output_file), runtime_code)?;
    
    println!("Compilación completada!");
    println!("Archivos generados:");
    println!("  - {}.s (código ensamblador)", output_file);
    println!("  - {}_runtime.c (runtime)", output_file);
    
    Ok(())
}
