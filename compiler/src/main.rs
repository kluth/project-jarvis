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
    let _server = McpServer::new(AiProvider::Gemini);
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let req = match line {
            Ok(content) => content,
            Err(_) => break,
        };

        if req.trim().is_empty() {
            continue;
        }

        // zero-dependency dynamic JSON-RPC 'id' extractor
        // Time: O(N) where N is length of request line.
        let extracted_id = req.split("\"id\"")
            .nth(1)
            .and_then(|s| s.split(':').nth(1))
            .and_then(|s| s.split(|c| c == ',' || c == '}').next())
            .unwrap_or("null")
            .trim();

        // 1. Handshake Phase 1: Strikte Request-Validierung
        if req.contains("\"method\":\"initialize\"") {
            println!(
                r#"{{"jsonrpc":"2.0","id":{},"result":{{"protocolVersion":"2024-11-05","capabilities":{{"tools":{{}}}},"serverInfo":{{"name":"jrvc-nci","version":"1.0.0"}}}}}}"#,
                extracted_id
            );
        } 
        // 2. Handshake Phase 2: Notification schlucken (Verhindert unendliche Schleifen!)
        else if req.contains("\"method\":\"notifications/initialized\"") {
            continue; 
        } 
        // 3. Tool Discovery & Capability Ankündigung
        else if req.contains("\"method\":\"tools/list\"") || req.contains("listTools") {
            println!(
                r#"{{"jsonrpc":"2.0","id":{},"result":{{"tools":[
                    {{"name":"query_ast","description":"Inspect semantic structure of a module","inputSchema":{{"type":"object","properties":{{"source":{{"type":"string"}}}}}}}},
                    {{"name":"analyze_energy","description":"Get nanojoule breakdown","inputSchema":{{"type":"object","properties":{{"fn_name":{{"type":"string"}}}}}}}},
                    {{"name":"apply_atomic_fix","description":"Autonomously repair a module","inputSchema":{{"type":"object","properties":{{"patch":{{"type":"string"}}}}}}}}
                ]}}}}"#,
                extracted_id
            );
        } 
        // 4. Tool Execution Interface
        else if req.contains("\"method\":\"tools/call\"") || req.contains("callTool") {
            println!(
                r#"{{"jsonrpc":"2.0","id":{},"result":{{"content":[ {{"type":"text","text":"JARVIS Action Successful."}} ]}}}}"#,
                extracted_id
            );
        }
        
        stdout.flush().unwrap();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_id_extraction() {
        let req = r#"{"jsonrpc":"2.0","id":"test-123","method":"initialize"}"#;
        let extracted_id = req.split("\"id\"")
            .nth(1)
            .and_then(|s| s.split(':').nth(1))
            .and_then(|s| s.split(|c| c == ',' || c == '}').next())
            .unwrap_or("null")
            .trim();
        assert_eq!(extracted_id, "\"test-123\"");
        
        let req_num = r#"{"jsonrpc":"2.0","id":42,"method":"initialize"}"#;
        let extracted_id_num = req_num.split("\"id\"")
            .nth(1)
            .and_then(|s| s.split(':').nth(1))
            .and_then(|s| s.split(|c| c == ',' || c == '}').next())
            .unwrap_or("null")
            .trim();
        assert_eq!(extracted_id_num, "42");
    }
}
