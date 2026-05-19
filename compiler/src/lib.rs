pub mod lexer;
pub mod parser;
pub mod ast;
pub mod semantics;
pub mod type_checker;
pub mod ir;
pub mod codegen;
pub mod vm;

#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer, Token};
    use crate::parser::Parser;
    use crate::ast::Node;
    use crate::semantics::Analyzer;

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
    fn test_complexity_verification_pass() {
        let source = "module Audio complexity O(1) { func init() {} }";
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse_module().unwrap();
        
        let mut analyzer = Analyzer::new();
        assert!(analyzer.analyze(&ast).is_ok());
    }

    #[test]
    fn test_complexity_verification_fail() {
        let source = "module Audio complexity O(N) { func init() {} }";
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse_module().unwrap();
        
        let mut analyzer = Analyzer::new();
        let result = analyzer.analyze(&ast);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Complexity mismatch"));
    }
}
