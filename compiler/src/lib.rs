pub mod lexer;
pub mod parser;
pub mod ast;
pub mod semantics;
pub mod type_checker;
pub mod ir;
pub mod codegen;
pub mod vm;
pub mod aot;
pub mod backend;
pub mod nci;
pub mod fixed_types;

#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer, Token};
    use crate::parser::Parser;
    use crate::ast::Node;
    use crate::semantics::OmegaVerifier;

    #[test]
    fn test_lex_basic_keywords() {
        let source = "module Voice complexity O(N) func verify";
        let mut lexer = Lexer::new(source);
        
        assert_eq!(lexer.next_token(), Token::Module);
        assert_eq!(lexer.next_token(), Token::Identifier("Voice"));
        assert_eq!(lexer.next_token(), Token::Complexity);
        assert_eq!(lexer.next_token(), Token::BigO("N"));
        assert_eq!(lexer.next_token(), Token::Func);
        assert_eq!(lexer.next_token(), Token::Verify);
    }

    #[test]
    fn test_parse_minimal_module() {
        let source = "module Audio complexity O(1) { func init() {} } verify { test \"ping\" {} }";
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse_module().expect("Failed to parse module");

        if let Node::Module { name, .. } = ast {
            assert_eq!(name, "Audio");
        } else {
            panic!("Expected Module node");
        }
    }

    #[test]
    fn test_omega_verification_pass() {
        let source = "module Audio complexity O(1) { verify { test \"init\" {} } func init() {} }";
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse_module().unwrap();
        
        let verifier = OmegaVerifier::new();
        assert!(verifier.verify(&ast).is_ok());
    }

    #[test]
    fn test_omega_verification_fail() {
        // Declared O(N^2) but it's actually O(1)
        let source = "module Audio complexity O(N^2) { verify { test \"init\" {} } func init() {} }";
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse_module().unwrap();
        
        let verifier = OmegaVerifier::new();
        let result = verifier.verify(&ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("PDD VIOLATION"));
    }

    #[test]
    fn test_system_programming_features() {
        let source = r#"
            module OS
            complexity O(1)
            {
                @no_mangle
                @section(".boot")
                func _start() {
                    asm { "cli; hlt" }
                }

                func main() {
                    volatile write(0x1000, 42);
                    volatile read(0x1000) -> x;
                    atomic store(x, 100);
                }

                allocator Heap {
                    let pool = 0;
                }
            }
        "#;
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse_module().expect("Failed to parse OS module");

        use crate::backend::AotBackend;
        let mut backend = AotBackend::new();
        let elf = backend.lower_to_elf(&ast).expect("Failed to lower to ELF");
        
        let metadata = String::from_utf8_lossy(&elf.metadata_section);
        assert!(metadata.contains("NO_MANGLE"));
        assert!(metadata.contains("SECTION:.boot"));
        assert!(metadata.contains("GLOBAL_ALLOCATOR"));
        
        // Verify opcodes in bytecode
        let code = &elf.code_section;
        assert!(code.contains(&0x17)); // AsmBlock
        assert!(code.contains(&0x18)); // VolatileWrite
        assert!(code.contains(&0x19)); // VolatileRead
        assert!(code.contains(&0x1A)); // AtomicGeneric
    }

    #[test]
    fn test_multiboot2_and_elf64_emission() {
        let source = "module Boot complexity O(1) { func _start() {} }";
        let lexer = crate::lexer::Lexer::new(source);
        let mut parser = crate::parser::Parser::new(lexer);
        let ast = parser.parse_module().unwrap();

        let mut backend = crate::backend::AotBackend::new();
        let elf = backend.lower_to_elf(&ast).expect("Failed to lower to ELF");

        // Verify ELF64 Header
        assert_eq!(&elf.elf_header[0..4], b"\x7fELF");
        assert_eq!(elf.elf_header[4], 2); // 64-bit
        assert_eq!(elf.elf_header[5], 1); // Little Endian
        assert_eq!(elf.elf_header[18], 0x3E); // x86_64 (machine)

        // Verify Multiboot2 Header
        let mb = &elf.multiboot_header;
        assert_eq!(mb[0..4], 0xE85250D6u32.to_le_bytes()); // Magic
        assert_eq!(mb[4..8], 0u32.to_le_bytes()); // Arch
        assert_eq!(mb[8..12], 16u32.to_le_bytes()); // Length
        
        // Verify Checksum
        let magic = 0xE85250D6u32;
        let arch = 0u32;
        let length = 16u32;
        let checksum = u32::from_le_bytes(mb[12..16].try_into().unwrap());
        assert_eq!(magic.wrapping_add(arch).wrapping_add(length).wrapping_add(checksum), 0);
    }

    #[test]
    fn test_hal_intrinsics() {
        let source = r#"
            module HAL
            complexity O(1)
            {
                @interrupt("0x80")
                func handle_syscall() {
                    port write(0x20, 0x20); // EOI
                    port read(0x60) -> key;
                }
            }
        "#;
        let lexer = crate::lexer::Lexer::new(source);
        let mut parser = crate::parser::Parser::new(lexer);
        let ast = parser.parse_module().expect("Failed to parse HAL module");

        let mut backend = crate::backend::AotBackend::new();
        let elf = backend.lower_to_elf(&ast).expect("Failed to lower to ELF");
        
        let metadata = String::from_utf8_lossy(&elf.metadata_section);
        assert!(metadata.contains("INTERRUPT:0x80"));
        
        let code = &elf.code_section;
        assert!(code.contains(&0x1F)); // PortWrite
        assert!(code.contains(&0x20)); // PortRead
    }

    #[test]
    fn test_structural_types_struct() {
        let source = r#"
            module Data
            complexity O(1)
            {
                struct Point {
                    x: f32,
                    y: f32,
                    id: i32
                }

                verify { test "move" {} }
                func move(p: Point) {
                    let new_x = 10.0;
                }
            }
        "#;
        let lexer = crate::lexer::Lexer::new(source);
        let mut parser = crate::parser::Parser::new(lexer);
        let ast = parser.parse_module().expect("Failed to parse Data module");

        let verifier = crate::semantics::OmegaVerifier::new();
        assert!(verifier.verify(&ast).is_ok());

        if let Node::Module { body, .. } = ast {
            let complexity_block = &body[0];
            if let Node::ComplexityBlock { content, .. } = complexity_block {
                let struct_node = &content[0];
                if let Node::Struct { name, fields } = struct_node {
                    assert_eq!(*name, "Point");
                    assert_eq!(fields.len(), 3);
                    assert_eq!(fields[0].0, "x");
                } else {
                    panic!("Expected Struct node, got {:?}", struct_node);
                }
            }
        }
    }

    #[test]
    fn test_multi_file_import() {
        let source = r#"
            import "std/math.jrv"
            import Network
            module App
            complexity O(1) {
                verify { test "start" {} }
                func start() {
                }
            }
        "#;
        let lexer = crate::lexer::Lexer::new(source);
        let mut parser = crate::parser::Parser::new(lexer);
        let ast = parser.parse_module().expect("Failed to parse App module");

        if let Node::Module { body, .. } = ast {
            assert!(matches!(body[0], Node::Import { path: "std/math.jrv" }));
            assert!(matches!(body[1], Node::Import { path: "Network" }));
        }
    }

    #[test]
    fn test_static_memory_initialization() {
        let source = r#"
            module Boot
            complexity O(1) {
                static VGA_BUFFER [0xB8000] {
                    size: 32000
                }

                verify { test "clear" {} }
                func clear_screen() {
                }
            }
        "#;
        let lexer = crate::lexer::Lexer::new(source);
        let mut parser = crate::parser::Parser::new(lexer);
        let ast = parser.parse_module().expect("Failed to parse Boot module");

        let verifier = crate::semantics::OmegaVerifier::new();
        assert!(verifier.verify(&ast).is_ok());

        if let Node::Module { body, .. } = ast {
            let complexity_block = &body[0];
            if let Node::ComplexityBlock { content, .. } = complexity_block {
                let static_node = &content[0];
                if let Node::Static { name, address, size } = static_node {
                    assert_eq!(*name, "VGA_BUFFER");
                    assert_eq!(*address, 0xB8000);
                    assert_eq!(*size, 32000);
                } else {
                    panic!("Expected Static node, got {:?}", static_node);
                }
            }
        }
    }

    #[test]
    fn test_for_loop_and_scifi_energy() {
        let source = r#"
            module Scifi
            complexity O(N) {
                verify { test "loops" {} }
                func process(data: Stream) {
                    budget { power: 30000_nj }
                    for x in data {
                        print(x, 0, 0, 1);
                    }
                    
                    prob {
                        0.5 => return 1,
                        0.5 => return 0,
                    }
                }
            }
        "#;
        let lexer = crate::lexer::Lexer::new(source);
        let mut parser = crate::parser::Parser::new(lexer);
        let ast = parser.parse_module().expect("Failed to parse Scifi module");

        let verifier = crate::semantics::OmegaVerifier::new();
        if let Err(e) = verifier.verify(&ast) {
            panic!("Verifier Error: {}", e);
        }

        let mut backend = crate::backend::AotBackend::new();
        let elf = backend.lower_to_elf(&ast).expect("Failed to lower to ELF");
        assert!(elf.code_section.len() > 0);
    }

    #[test]
    fn test_windowing_and_rendering() {
        let source = r#"
            module GUI
            complexity O(N)
            {
                verify { test "render_test" { assert(true); } }

                render Dashboard() {
                    layout "grid" {
                        component "Status"();
                    }
                }

                func main() {
                    window "JARVIS" [1920, 1080];
                    event "click" {
                        poll;
                        Dashboard();
                    }
                }
            }
        "#;
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse_module().expect("Failed to parse GUI module");

        use crate::backend::AotBackend;
        let mut backend = AotBackend::new();
        let elf = backend.lower_to_elf(&ast).expect("Failed to lower to ELF");
        
        let code = &elf.code_section;
        assert!(code.contains(&0x0E)); // WinCreate
        assert!(code.contains(&0x12)); // WinPoll
        assert!(code.contains(&0x08)); // UIRender
        assert!(code.contains(&0x0A)); // UIComponent
    }

    #[test]
    fn test_scifi_ui_primitives() {
        let source = r#"
            module SciFi
            complexity O(1)
            {
                render HUD() {
                    hologram "parallax" (1.0) {
                        component "ParticleField"();
                    }
                }

                func main() {
                    post_process "glitch" (0.8) {
                        HUD();
                    }
                    neuro_adapt (0.5) {
                        poll;
                    }
                }
            }
        "#;
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse_module().expect("Failed to parse SciFi module");

        use crate::backend::AotBackend;
        let mut backend = AotBackend::new();
        let elf = backend.lower_to_elf(&ast).expect("Failed to lower to ELF");
        
        let code = &elf.code_section;
        assert!(code.contains(&0x1B)); // UIHologramStart
        assert!(code.contains(&0x1C)); // UIHologramEnd
        assert!(code.contains(&0x1D)); // UIPostProcess
        assert!(code.contains(&0x1E)); // UINeuroAdapt
    }
}
