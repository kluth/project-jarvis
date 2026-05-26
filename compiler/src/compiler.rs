use crate::lexer::{Lexer, Token};
use crate::ast::{Node, Stmt, Expr, Type};
use std::fs;
use std::path::Path;

/// The JARVIS Production Compiler Pipeline.
/// Orchestrates Lexical Analysis -> Parsing -> Semantics -> Codegen.
pub struct JRVCompiler;

impl JRVCompiler {
    /// Compile a .jrv source file to JRV bytecode
    pub fn compile_file(path: &str) -> Result<CompileResult<'static>, String> {
        let source = fs::read_to_string(path)
            .map_err(|e| format!("Cannot read {}: {}", path, e))?;
        Self::compile_source(source, Some(path))
    }

    /// Compile JRV source string to bytecode
    pub fn compile_source(source: String, source_name: Option<&str>) -> Result<CompileResult, String> {
        // Leak the string to obtain a 'static lifetime for the AST borrows
        let source: &'static str = Box::leak(source.into_boxed_str());
        // Phase 1: Lexical Analysis
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        
        // Phase 2: Parsing
        let lexer2 = Lexer::new(source);
        let mut parser = crate::parser::Parser::new(lexer2);
        let ast = parser.parse_module()?;
        
        // Phase 3: Semantic Analysis
        let verifier = crate::semantics::OmegaVerifier::new();
        verifier.verify(&ast)?;
        
        // Phase 4: Type Checking
        let mut type_checker = crate::type_checker::TypeChecker::new();
        type_checker.check(&ast)?;
        
        // Phase 5: Code Generation
        let mut codegen = crate::codegen::CodeGenerator::new();
        let bytecode = codegen.generate(&ast)?;
        
        Ok(CompileResult {
            ast,
            bytecode,
            token_count: tokens.len(),
            module_name: source_name.unwrap_or("unknown").to_string(),
        })
    }

    /// Compile and execute a .jrv file
    pub fn run_file(path: &str) -> Result<(), String> {
        let result = Self::compile_file(path)?;
        
        let mut vm = crate::vm::VM::new();
        vm.execute(&result.bytecode);
        
        println!("[JRV] Program '{}' completed successfully", result.module_name);
        println!("[JRV] {} tokens, {} instructions", result.token_count, result.bytecode.instructions.len());
        Ok(())
    }
}

pub struct CompileResult {
    pub ast: crate::ast::Node<'static>,
    pub bytecode: crate::ir::Bytecode,
    pub token_count: usize,
    pub module_name: String,
}