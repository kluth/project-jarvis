use std::fs;
use std::env;
use jarvis_compiler::lexer::Lexer;
use jarvis_compiler::parser::Parser;
use jarvis_compiler::semantics::OmegaVerifier;
use jarvis_compiler::type_checker::TypeChecker;
use jarvis_compiler::codegen::CodeGenerator;
use jarvis_compiler::vm::VM;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: jrvc <file.jrv>");
        return;
    }

    let file_path = &args[1];
    let source = fs::read_to_string(file_path).expect("Failed to read source file");

    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    
    match parser.parse_module() {
        Ok(ast) => {
            let verifier = OmegaVerifier::new();
            let mut checker = TypeChecker::new();
            
            // 1. Omega Verification (PDD/EFDD/EuDD Gatekeeper)
            match verifier.verify(&ast) {
                Ok(_) => {
                    // 2. Type Checking
                    match checker.check(&ast) {
                        Ok(_) => {
                            println!("Compilation Successful: PDD Verified, Types Checked.");
                            
                            // 3. Code Generation
                            let mut codegen = CodeGenerator::new();
                            match codegen.generate(&ast) {
                                Ok(bytecode) => {
                                    println!("Bytecode Generated: {} instructions.", bytecode.instructions.len());
                                    
                                    println!("--- VM Execution Start ---");
                                    let mut vm = VM::new();
                                    vm.execute(&bytecode);
                                    println!("--- VM Execution End ---");
                                }
                                Err(e) => println!("Codegen Error: {}", e),
                            }
                        }
                        Err(e) => println!("Type Error: {}", e),
                    }
                }
                Err(e) => println!("Verification Error: {}", e),
            }
        }
        Err(e) => println!("Parser Error: {:?}", e),
    }
}
