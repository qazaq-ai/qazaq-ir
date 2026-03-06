pub mod morpheme_registry;
pub mod qazaq_codegen;
pub mod qazaq_lexer;
pub mod semantic_router;

// Architectural export for the Qazaq IR MVP Compiler
pub use morpheme_registry::{MorphemeRegistry, RootEntity, SuffixMorpheme};
pub use qazaq_codegen::CodegenBackend;
pub use qazaq_lexer::{AgglutinativeToken, QazaqLexer};
pub use semantic_router::{RouterError, RouterPayload, SemanticRouter};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_e2e_semantic_router_valid_agglutination() {
        let valid_payload = r#"{
            "tokens": [
                {
                    "root": { "type": "StateObject", "value": "UserSession" },
                    "morphs": ["AllocHeap", "SignWithMLDSA", "WriteToTarget"]
                }
            ]
        }"#;

        let result = SemanticRouter::parse_intent_payload(valid_payload);
        assert!(result.is_ok(), "Valid topology should pass compilation");

        let tokens = result.unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens[0].root,
            RootEntity::StateObject("UserSession".to_string())
        );
    }

    #[test]
    fn test_e2e_fatal_hallucination() {
        // Attempting to write to a StateObject without first allocating or making mutable
        let hallucinated_payload = r#"{
            "tokens": [
                {
                    "root": { "type": "StateObject", "value": "UserSession" },
                    "morphs": ["WriteToTarget", "AllocHeap"]
                }
            ]
        }"#;

        let result = SemanticRouter::parse_intent_payload(hallucinated_payload);
        assert!(
            result.is_err(),
            "Hallucinated topology must fail immediately"
        );

        match result.unwrap_err() {
            RouterError::HallucinationDetected(msg) => {
                assert!(msg.contains("FATAL HALLUCINATION"));
            }
            _ => panic!("Expected hallucination detection"),
        }
    }

    #[test]
    fn test_e2e_codegen_emission() {
        let valid_payload = r#"{
            "tokens": [
                {
                    "root": { "type": "StateObject", "value": "UserSession" },
                    "morphs": ["AllocHeap", "SignWithMLDSA", "WriteToTarget"]
                }
            ]
        }"#;

        let tokens = SemanticRouter::parse_intent_payload(valid_payload).unwrap();
        let emitted_code = CodegenBackend::emit_payload(&tokens);

        assert!(emitted_code.contains("fn qazaq_ir_main()"));
        assert!(emitted_code.contains("let mut usersession_state = State::new(\"UserSession\");"));
        assert!(emitted_code.contains("let mut usersession_state = allocate_heap_memory(1024);"));
        assert!(emitted_code.contains("let signature = orda_pqc::mldsa_sign(&usersession_state);"));
        assert!(emitted_code.contains("storage_engine::commit(&usersession_state);"));
    }
}
