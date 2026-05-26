use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::semantics::OmegaVerifier;
use crate::ir::Bytecode;
use crate::codegen::CodeGenerator;

/// The Living Evolution JIT Engine.
/// Orchestrates Source -> AST -> Verified AST -> IR conversion.
/// Time: O(N) where N is source size.
pub struct JitEngine {
    verifier: OmegaVerifier,
}

impl JitEngine {
    pub fn new() -> Self {
        Self {
            verifier: OmegaVerifier::new(),
        }
    }

    /// Compiles source code into verified bytecode in memory.
    /// PDD/EFDD/EuDD checks are enforced before any IR is emitted.
    pub fn compile(&self, source: &str) -> Result<Bytecode, String> {
        // 1. Lexical Analysis
        let lexer = Lexer::new(source);
        
        // 2. Syntactic Analysis
        let mut parser = Parser::new(lexer);
        let ast = parser.parse_module().map_err(|e| format!("Parser Error: {:?}", e))?;

        // 3. Omega Verification (PDD/EFDD/EuDD Gatekeeper)
        // This is the "Verification Primitive" mentioned in GEMINI.md
        self.verifier.verify(&ast)?;

        // 4. Code Generation
        // Only reached if verification passes.
        let mut codegen = CodeGenerator::new();
        let bytecode = codegen.generate(&ast)?;

        Ok(bytecode)
    }
}
