use std::fs;
use std::env;
use jarvis_compiler::lexer::Lexer;
use jarvis_compiler::parser::Parser;
use jarvis_compiler::semantics::Analyzer;

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
            let mut analyzer = Analyzer::new();
            match analyzer.analyze(&ast) {
                Ok(_) => println!("Compilation Successful: PDD Verified, TDD Blocks Parsed."),
                Err(e) => println!("Semantic Error: {}", e),
            }
        }
        Err(e) => println!("Parser Error: {}", e),
    }
}
