//! Language Parser: A complete compiler pipeline
//! 
//! Transforms source code through 5 phases:
//! 1. Lexer - Tokenization
//! 2. Parser - Syntax analysis (AST generation)
//! 3. Semantic Analyzer - Type checking & validation
//! 4. Code Generator - Three-Address Code (TAC) generation
//! 5. Interpreter - TAC execution on abstract machine

pub mod phases {
    pub mod lexer;
    pub mod parser;
    pub mod semantic;
    pub mod codegen;
}

pub mod ast;
pub mod token;
pub mod symbol_table;
pub mod tac;
pub mod interpreter;
pub mod error;

pub use error::{CompileError, CompileResult};

/// Compile source code through all phases
pub fn compile(source: &str) -> CompileResult<Vec<tac::Instr>> {
    // Phase 1: Lexical Analysis
    let tokens = phases::lexer::Lexer::tokenize(source)
        .iter()
        .map(|st| st.token.clone())
        .collect();

    // Phase 2: Syntax Analysis
    let mut parser = phases::parser::Parser::new(tokens);
    let ast = parser.parse()?;

    // Phase 3: Semantic Analysis
    let mut checker = phases::semantic::TypeChecker::new();
    checker.check_program(&ast)?;

    // Phase 4: Code Generation
    let mut gen = phases::codegen::CodeGen::new();
    for stmt in &ast {
        gen.gen_stmt(stmt);
    }

    Ok(gen.instrs)
}

/// Compile and execute
pub fn compile_and_run(source: &str) -> CompileResult<Option<i64>> {
    let instrs = compile(source)?;
    let mut interp = interpreter::Interpreter::new();
    Ok(interp.run(&instrs))
}