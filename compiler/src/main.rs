use std::fs;
use std::env;
use jarvis_compiler::lexer::Lexer;
use jarvis_compiler::parser::Parser;
use jarvis_compiler::semantics::OmegaVerifier;
use jarvis_compiler::backend::AotBackend;
use jarvis_compiler::nci::McpServer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: jrvc <file.jrv> [--mcp]");
        return;
    }

    if args.contains(&"--mcp".to_string()) {
        println!("JARVIS Neuro-Compiler Interface (NCI) Active.");
        println!("MCP Server listening for autonomous repair loops...");
        let _server = McpServer::new();
        // Mock server loop
        return;
    }

    let file_path = &args[1];
    let source = fs::read_to_string(file_path).expect("Failed to read source file");

    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    
    match parser.parse_module() {
        Ok(ast) => {
            let verifier = OmegaVerifier::new();
            
            // 1. Omega Verification (PDD/EFDD/EuDD Gatekeeper)
            match verifier.verify(&ast) {
                Ok(_) => {
                    println!("Compilation Successful: PDD/EFDD Verified.");
                    
                    // 2. AOT Native ELF Lowering
                    let mut backend = AotBackend::new();
                    match backend.lower_to_elf(&ast) {
                        Ok(elf) => {
                            println!("AOT Production ELF Generated.");
                            println!("Code Size: {} bytes.", elf.code_section.len());
                            println!("Metadata: {}.", String::from_utf8_lossy(&elf.metadata_section));
                            println!("Target: Custom Jarvis-ISA (Machine-Native).");
                        }
                        Err(e) => println!("Backend Error: {}", e),
                    }
                }
                Err(e) => println!("Verification Error: {}", e),
            }
        }
        Err(e) => println!("Parser Error: {:?}", e),
    }
}
