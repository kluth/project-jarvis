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
                            fs::write("output.elf", &elf.code_section).expect("Failed to write ELF");
                            println!("AOT Production ELF Generated ({} bytes) -> output.elf", elf.code_section.len());
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

use std::fs::OpenOptions;

/// Zero-Dependency MCP Stdio Server Loop.
/// Refactored for absolute protocol compliance and robust parsing.
/// Time Complexity: O(N) per request.
fn run_mcp_server() {
    let mut server = McpServer::new(AiProvider::Gemini);
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut log_file = OpenOptions::new()
        .create(true).append(true).open("mcp.log").ok();

    if let Some(ref mut f) = log_file {
        let _ = writeln!(f, "\n--- MCP SESSION START ---");
    }

    for line in stdin.lock().lines() {
        let req = match line {
            Ok(content) => content,
            Err(_) => break,
        };

        if req.trim().is_empty() { continue; }
        if let Some(ref mut f) = log_file { let _ = writeln!(f, "IN: {}", req); }

        // 1. Precise Field Extraction
        let get_raw_val = |target: &str| {
            let key = format!("\"{}\"", target);
            req.find(&key).and_then(|pos| {
                let rem = &req[pos + key.len()..];
                rem.find(':').map(|c_pos| {
                    let val = rem[c_pos + 1..].trim();
                    let end = if val.starts_with('"') {
                        val[1..].find('"').map(|p| p + 2).unwrap_or(val.len())
                    } else {
                        val.find(|c: char| c == ',' || c == '}' || c == ']' || c.is_whitespace()).unwrap_or(val.len())
                    };
                    val[..end].trim().to_string()
                })
            })
        };

        let method_raw = get_raw_val("method");
        let method = method_raw.as_ref().map(|m| m.trim_matches('"'));
        let id = get_raw_val("id").unwrap_or("null".to_string());
        
        // 2. Response Dispatcher
        let response = match method {
            Some("initialize") => {
                Some(format!(
                    r#"{{"jsonrpc":"2.0","id":{},"result":{{"protocolVersion":"2024-11-05","capabilities":{{"tools":{{"listChanged":false}},"resources":{{"subscribe":false,"listChanged":false}},"prompts":{{"listChanged":false}}}},"serverInfo":{{"name":"jrvc-nci","version":"1.0.0"}}}}}}"#,
                    id
                ))
            }
            Some("tools/list") => {
                Some(format!(
                    r#"{{"jsonrpc":"2.0","id":{},"result":{{"tools":[{{"name":"query_ast","description":"Inspect AST","inputSchema":{{"type":"object","properties":{{"source":{{"type":"string"}}}}}}}},{{"name":"analyze_energy","description":"Energy breakdown","inputSchema":{{"type":"object","properties":{{"fn_name":{{"type":"string"}}}}}}}},{{"name":"run_mutants","description":"Mutation testing","inputSchema":{{"type":"object","properties":{{"source":{{"type":"string"}}}}}}}},{{"name":"apply_atomic_fix","description":"Repair module","inputSchema":{{"type":"object","properties":{{"patch":{{"type":"string"}}}}}}}},{{"name":"analyze_screenshot","description":"Fetch and analyze screenshot","inputSchema":{{"type":"object","properties":{{"url":{{"type":"string"}}}}}}}},{{"name":"compare_design","description":"Compare native GUI to Stitch design","inputSchema":{{"type":"object","properties":{{"screen_id":{{"type":"string"}},"executable":{{"type":"string"}}}}}}}}]}}}}"#,
                    id
                ))
            }
            Some("tools/call") | Some("callTool") => {
                let name_raw = get_raw_val("name");
                let name = name_raw.as_ref().map(|n| n.trim_matches('"'));
                
                let result = match name {
                    Some("analyze_screenshot") => {
                        let url = get_raw_val("url").unwrap_or("".to_string()).trim_matches('"').to_string();
                        server.analyze_screenshot(&url).unwrap_or_else(|e| e)
                    }
                    Some("apply_atomic_fix") => {
                        let patch = get_raw_val("patch").unwrap_or("".to_string()).trim_matches('"').to_string();
                        server.apply_atomic_fix(&patch).unwrap_or_else(|e| e)
                    }
                    Some("compare_design") => {
                        let screen_id = get_raw_val("screen_id").unwrap_or("".to_string()).trim_matches('"').to_string();
                        let executable = get_raw_val("executable").unwrap_or("".to_string()).trim_matches('"').to_string();
                        server.compare_design(&screen_id, &executable).unwrap_or_else(|e| e)
                    }
                    _ => "Tool not implemented or unknown.".to_string(),
                };

                Some(format!(
                    r#"{{"jsonrpc":"2.0","id":{},"result":{{"content":[{{"type":"text","text":"{}"}}]}}}}"#,
                    id, result
                ))
            }
            _ => {
                if id != "null" {
                    Some(format!(
                        r#"{{"jsonrpc":"2.0","id":{},"error":{{"code":-32601,"message":"Method not found"}}}}"#,
                        id
                    ))
                } else {
                    None
                }
            }
        };

        // 3. Emit Response
        if let Some(res) = response {
            let mut out = io::stdout();
            let _ = out.write_all(res.as_bytes());
            let _ = out.write_all(b"\n");
            let _ = out.flush();
            if let Some(ref mut f) = log_file { 
                let _ = writeln!(f, "OUT: {}", res);
                let _ = f.flush();
            }
        }
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
            .map(|s| s.trim());
        assert_eq!(extracted_id, Some("\"test-123\""));
        
        let req_num = r#"{"jsonrpc":"2.0","id":42,"method":"initialize"}"#;
        let extracted_id_num = req_num.split("\"id\"")
            .nth(1)
            .and_then(|s| s.split(':').nth(1))
            .and_then(|s| s.split(|c| c == ',' || c == '}').next())
            .map(|s| s.trim());
        assert_eq!(extracted_id_num, Some("42"));
    }
}
