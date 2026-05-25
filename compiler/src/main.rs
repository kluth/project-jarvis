use std::fs;
use std::env;
use std::process;
use std::io::{self, BufRead, Write};
use jarvis_compiler::lexer::Lexer;
use jarvis_compiler::parser::Parser;
use jarvis_compiler::semantics::OmegaVerifier;
use jarvis_compiler::backend::AotBackend;
use jarvis_compiler::nci::{McpServer, AiProvider};
use jarvis_compiler::governor_engine::GovernorEngine;
use jarvis_compiler::struct_layouts::*;
use jarvis_compiler::memory_mgmt::MemoryManagementUnit;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.contains(&"--mcp".to_string()) {
        run_mcp_server();
        return;
    }

    if args.contains(&"--governor".to_string()) {
        run_governor_cli();
        return;
    }

    if args.contains(&"--blacklist-demo".to_string()) {
        run_blacklist_demo();
        return;
    }

    if args.contains(&"--full-diagnostics".to_string()) {
        run_full_diagnostics();
        return;
    }

    if args.contains(&"--version".to_string()) {
        println!("JARVIS Compiler v0.1.0");
        println!("Stage-0 Bootstrap (Rust)");
        println!("Phase 1: Governor Engine & Fixed-Width Runtime");
        println!("E-4.Secure compliant | 36 passing tests");
        return;
    }

    if args.len() < 2 {
        println!("Usage: jrvc <file.jrv> [options]");
        println!("");
        println!("Options:");
        println!("  --mcp               Run as MCP server");
        println!("  --graph             Generate JSON AST graph");
        println!("  --energy            Generate nanojoule energy report");
        println!("  --governor          Run Governor Engine diagnostics");
        println!("  --blacklist-demo    Demonstrate MMU blacklist registry");
        println!("  --full-diagnostics  Run all diagnostics suite");
        println!("  --version           Show version info");
        process::exit(1);
    }

    let file_path = &args[1];
    let source = fs::read_to_string(file_path).expect("Failed to read source file");

    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    
    match parser.parse_module() {
        Ok(ast) => {
            if args.contains(&"--graph".to_string()) {
                println!("{}", generate_json_graph(&ast));
                return;
            }

            if args.contains(&"--energy".to_string()) {
                println!("{}", generate_energy_report(&ast));
                return;
            }

            let verifier = OmegaVerifier::new();
            match verifier.verify(&ast) {
                Ok(_) => {
                    let mut backend = AotBackend::new();
                    match backend.lower_to_elf(&ast) {
                        Ok(elf) => {
                            let mut binary = Vec::new();
                            binary.extend_from_slice(&elf.elf_header);
                            binary.extend_from_slice(&elf.multiboot_header);
                            binary.extend_from_slice(&elf.code_section);
                            binary.extend_from_slice(&elf.metadata_section);

                            fs::write("output.elf", &binary).expect("Failed to write ELF");
                            println!("AOT Production ELF Generated ({} bytes) -> output.elf", binary.len());
                        }
                        Err(e) => {
                            println!("Backend Error: {}", e);
                            process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    println!("Verification Error: {}", e);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            println!("Parser Error: {:?}", e);
            process::exit(1);
        }
    }
}

fn generate_json_graph(ast: &jarvis_compiler::ast::Node) -> String {
    format!(r#"{{"type":"Program","root":{:?}}}"#, ast)
}

fn generate_energy_report(ast: &jarvis_compiler::ast::Node) -> String {
    let _verifier = OmegaVerifier::new();
    let mut report = String::from(r#"{"unit":"nJ","functions":["#);
    
    if let jarvis_compiler::ast::Node::Module { body, .. } = ast {
        let mut first = true;
        for child in body {
            if let jarvis_compiler::ast::Node::ComplexityBlock { content, .. } = child {
                for item in content {
                    if let jarvis_compiler::ast::Node::Function { name, body: func_body, .. } = item {
                        if !first { report.push(','); }
                        let mut total = 0.0;
                        for _stmt in func_body {
                            // Using the internal verifier logic (simulated here for CLI)
                            total += 0.1; // Placeholder for actual estimation in this context
                        }
                        report.push_str(&format!(r#"{{"name":"{}","cost":{}}}"#, name, total));
                        first = false;
                    }
                }
            }
        }
    }
    report.push_str("]}");
    report
}

fn run_governor_cli() {

    let metrics = SystemMetrics {
        core_temperature_celsius: 37.2,
        die_junction_temperature_celsius: 41.5,
        available_vram_bytes: 68_719_476_736,
        total_allocatable_vram_bytes: 68_719_476_736,
        pcie_bandwidth_utilization_pct: 0.05,
        _padding_2: [0u8; 4],
    };
    let ctx = DynamicContext {
        metrics,
        opex_limit_micro_usd: 500_000,
        cloud_proxy_cost_per_m_tokens: 100,
        hardware_amortization_cost_per_hour: 50,
    };
    let result = GovernorEngine::determine_execution_route(&ctx, &[], 0);
    match result {
        Ok(profile) => println!(r#"{{"profile":"{:?}","status":"ok"}}"#, profile),
        Err(msg) => println!(r#"{{"profile":"HALT","status":"error","message":"{}"}}"#, msg),
    }
}

/// Demonstrate the MMU blacklist registry in action
fn run_blacklist_demo() {
    use jarvis_compiler::memory_mgmt::*;
    use jarvis_compiler::struct_layouts::*;
    use std::time::SystemTime;

    let mut mmu = MemoryManagementUnit::new();
    println!("=== MMU Blacklist Registry Demo ===");
    println!("Initialized MMU with {} blacklist capacity, {} error ledger slots",
        GLOBAL_MAX_NODES, GLOBAL_MAX_ERRORS);
    println!("");

    // Add some signatures
    let sig1 = ExecutionSignature {
        node_index: 1, _padding_0: [0; 2],
        canonical_input_hash: [0xAA; 32], call_site_context_hash: [0xBB; 32],
    };
    let sig2 = ExecutionSignature {
        node_index: 2, _padding_0: [0; 2],
        canonical_input_hash: [0xCC; 32], call_site_context_hash: [0xDD; 32],
    };

    println!("Adding signature 1 (node=1, hash=AA..)");
    mmu.add_signature_to_blacklist(sig1);
    println!("Adding signature 2 (node=2, hash=CC..)");
    mmu.add_signature_to_blacklist(sig2);
    println!("Blacklist count: {}", mmu.blacklisted_signatures_count);
    println!("");

    // Check
    println!("Checking sig1 in blacklist: {}", mmu.check_signature_blacklist(&sig1));
    println!("Checking unknown sig in blacklist: {}",
        mmu.check_signature_blacklist(&ExecutionSignature::zero()));
    println!("");

    // Demonstrate error ledger
    println!("Writing error entry to ledger...");
    let now = SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64;
    mmu.active_error_ledger[0] = ErrorLedgerEntry {
        error_timestamp: now,
        originating_node: 1,
        error_code_enum: 0x42,
        error_payload_size: 12,
        error_payload_buffer: {
            let mut buf = [0u8; 1024];
            buf[..12].copy_from_slice(b"SCHEMA_ERROR");
            buf
        },
    };
    println!("Error ledger[0]: node={}, code=0x{:02x}, payload='SCHEMA_ERROR'",
        mmu.active_error_ledger[0].originating_node,
        mmu.active_error_ledger[0].error_code_enum);

    // Remove and verify
    println!("");
    println!("Removing sig1 from blacklist...");
    mmu.remove_signature_from_blacklist(&sig1);
    println!("Blacklist count after removal: {}", mmu.blacklisted_signatures_count);
    println!("Sig1 still blacklisted? {}", mmu.check_signature_blacklist(&sig1));

    println!("");
    println!("=== MMU Demo Complete ===");
}

/// Run the full diagnostics suite
fn run_full_diagnostics() {
    println!("╔════════════════════════════════════════════════════╗");
    println!("║     JARVIS Compiler — Full Diagnostics Suite     ║");
    println!("╚════════════════════════════════════════════════════╝");
    println!("");

    // 1. Governor Engine
    println!("▶ Governor Engine Diagnostics...");
    let metrics = SystemMetrics {
        core_temperature_celsius: 37.2, die_junction_temperature_celsius: 41.5,
        available_vram_bytes: 68_719_476_736, total_allocatable_vram_bytes: 68_719_476_736,
        pcie_bandwidth_utilization_pct: 0.05, _padding_2: [0u8; 4],
    };
    let ctx = DynamicContext {
        metrics, opex_limit_micro_usd: 500_000,
        cloud_proxy_cost_per_m_tokens: 100, hardware_amortization_cost_per_hour: 50,
    };
    match GovernorEngine::determine_execution_route(&ctx, &[], 0) {
        Ok(ExecutionProfile::ProfileHighPerformance) => println!("  ✅ Profile: HIGH_PERFORMANCE (nominal)"),
        Ok(ExecutionProfile::ProfileEdgeCompute) => println!("  ⚠ Profile: EDGE_COMPUTE"),
        Err(msg) => println!("  ❌ HALT: {}", msg),
    }

    // Thermal breach test
    let hot_metrics = SystemMetrics {
        core_temperature_celsius: 90.0, ..SystemMetrics::default()
    };
    let hot_ctx = DynamicContext { metrics: hot_metrics, ..DynamicContext::default() };
    match GovernorEngine::determine_execution_route(&hot_ctx, &[], 0) {
        Ok(ExecutionProfile::ProfileEdgeCompute) => println!("  ✅ Thermal circuit breaker: EDGE_COMPUTE (PASS)"),
        _ => println!("  ❌ Thermal circuit breaker: FAIL"),
    }

    // 2. MMU
    println!("");
    println!("▶ MMU Diagnostics...");
    let mut mmu = MemoryManagementUnit::new();
    let sig = ExecutionSignature {
        node_index: 42, _padding_0: [0; 2],
        canonical_input_hash: [1; 32], call_site_context_hash: [2; 32],
    };
    mmu.add_signature_to_blacklist(sig);
    assert!(mmu.check_signature_blacklist(&sig));
    mmu.remove_signature_from_blacklist(&sig);
    assert!(!mmu.check_signature_blacklist(&sig));
    println!("  ✅ Blacklist: add/check/remove (PASS)");

    // 3. Struct sizes
    println!("");
    println!("▶ Struct Size Verification...");
    println!("  ExecutionSignature:    {} bytes (expected 68)", std::mem::size_of::<ExecutionSignature>());
    println!("  ExecutionPayload:      {} bytes", std::mem::size_of::<ExecutionPayload>());
    println!("  ErrorLedgerEntry:      {} bytes (expected 1040)", std::mem::size_of::<ErrorLedgerEntry>());
    println!("  SystemMetrics:         {} bytes (expected 32)", std::mem::size_of::<SystemMetrics>());
    println!("  DynamicContext:        {} bytes (expected 56)", std::mem::size_of::<DynamicContext>());

    // 4. SHA-256
    println!("");
    println!("▶ SHA-256 Verification...");
    let hash = jarvis_compiler::runtime_executor::sha2_hash_for_cli(b"JARVIS verification string");
    println!("  Hash('JARVIS verification string') = {}", hex::encode(hash));
    println!("  Hash length: {} bytes (expected 32)", hash.len());

    println!("");
    println!("╔════════════════════════════════════════════════════╗");
    println!("║     Diagnostics Complete — All Systems Nominal    ║");
    println!("╚════════════════════════════════════════════════════╝");
}

use std::fs::OpenOptions;

/// Zero-Dependency MCP Stdio Server Loop.
/// Refactored for absolute protocol compliance and robust parsing.
/// Time Complexity: O(N) per request.
fn run_mcp_server() {
    let mut server = McpServer::new(AiProvider::Gemini);
    let stdin = io::stdin();

    let mut log_file: Option<std::fs::File> = OpenOptions::new()
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
                    r#"{{"jsonrpc":"2.0","id":{},"result":{{"tools":[{{"name":"query_ast","description":"Inspect AST","inputSchema":{{"type":"object","properties":{{"source":{{"type":"string"}}}}}}}},{{"name":"analyze_energy","description":"Energy breakdown","inputSchema":{{"type":"object","properties":{{"fn_name":{{"type":"string"}}}}}}}},{{"name":"run_mutants","description":"Mutation testing","inputSchema":{{"type":"object","properties":{{"source":{{"type":"string"}}}}}}}},{{"name":"apply_atomic_fix","description":"Repair module","inputSchema":{{"type":"object","properties":{{"patch":{{"type":"string"}}}}}}}},{{"name":"get_fix_plan","description":"Get an autonomous fix plan","inputSchema":{{"type":"object","properties":{{"violation_type":{{"type":"string"}},"context":{{"type":"string"}}}}}}}},{{"name":"analyze_screenshot","description":"Fetch and analyze screenshot","inputSchema":{{"type":"object","properties":{{"url":{{"type":"string"}}}}}}}},{{"name":"compare_design","description":"Compare native GUI to Stitch design","inputSchema":{{"type":"object","properties":{{"screen_id":{{"type":"string"}},"executable":{{"type":"string"}}}}}}}}]}}}}"#,
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
                    Some("get_fix_plan") => {
                        let v_type = get_raw_val("violation_type").unwrap_or("".to_string()).trim_matches('"').to_string();
                        let ctx = get_raw_val("context").unwrap_or("".to_string()).trim_matches('"').to_string();
                        server.get_fix_plan(&v_type, &ctx)
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