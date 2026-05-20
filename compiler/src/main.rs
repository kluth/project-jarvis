use std::fs;
use std::env;
use std::io::{self, BufRead, Write};
use jarvis_compiler::lexer::Lexer;
use jarvis_compiler::parser::Parser;
use jarvis_compiler::semantics::OmegaVerifier;
use jarvis_compiler::backend::AotBackend;
use jarvis_compiler::nci::{McpServer, AiProvider};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.contains(&"--mcp".to_string()) {
        run_mcp_server();
        return;
    }

    if args.len() < 2 {
        println!("Usage: jrvc <file.jrv> [--mcp]");
        return;
    }

    let file_path = &args[1];
    let source = fs::read_to_string(file_path).expect("Failed to read source file");

    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    
    match parser.parse_module() {
        Ok(ast) => {
            let verifier = OmegaVerifier::new();
            match verifier.verify(&ast) {
                Ok(_) => {
                    let mut backend = AotBackend::new();
                    match backend.lower_to_elf(&ast) {
                        Ok(elf) => {
                            println!("AOT Production ELF Generated ({} bytes).", elf.code_section.len());
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

/// Zero-Dependency MCP Stdio Server Loop
/// Time: O(1) loop, O(N) per request.
fn run_mcp_server() {
    let mut server = McpServer::new(AiProvider::Gemini);
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let req = line.unwrap();
        if req.contains("initialize") {
            println!(r#"{{"jsonrpc":"2.0","id":1,"result":{{"protocolVersion":"2024-11-05","capabilities":{{}},"serverInfo":{{"name":"jrvc-nci","version":"1.0.0"}}}}}}"#);
        } else if req.contains("listTools") {
            println!(r#"{{"jsonrpc":"2.0","id":2,"result":{{"tools":[
                {{"name":"query_ast","description":"Inspect semantic structure of a module","inputSchema":{{"type":"object","properties":{{"source":{{"type":"string"}}}}}}}},
                {{"name":"analyze_energy","description":"Get nanojoule breakdown","inputSchema":{{"type":"object","properties":{{"fn_name":{{"type":"string"}}}}}}}},
                {{"name":"apply_atomic_fix","description":"Autonomously repair a module","inputSchema":{{"type":"object","properties":{{"patch":{{"type":"string"}}}}}}}}
            ]}}}}"#);
        } else if req.contains("callTool") {
            // In a production implementation, we'd use a JSON parser.
            // Here we provide a deterministic proof-of-concept response.
            println!(r#"{{"jsonrpc":"2.0","id":3,"result":{{"content":[ {{"type":"text","text":"JARVIS Action Successful."}} ]}}}}"#);
        }
        stdout.flush().unwrap();
    }
}
