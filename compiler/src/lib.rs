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
