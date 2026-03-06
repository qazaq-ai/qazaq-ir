pub mod llm_bridge;
pub mod llvm_codegen;
pub mod morpheme_registry;
pub mod orda_pqc;
pub mod qazaq_codegen;
pub mod qazaq_lexer;
pub mod semantic_router;

// Architectural export for the Qazaq IR MVP Compiler
pub use llm_bridge::LlmBridge;
pub use llvm_codegen::LlvmBackend;
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
                    "morphs": ["AllocHeap", { "SignWithMLDSA": "node_operator_key" }, "WriteToTarget"]
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
                    "morphs": ["AllocHeap", { "SignWithMLDSA": "node_operator_key" }, "WriteToTarget"]
                }
            ]
        }"#;

        let tokens = SemanticRouter::parse_intent_payload(valid_payload).unwrap();
        let emitted_code = CodegenBackend::emit_payload(&tokens);

        assert!(emitted_code.contains("fn qazaq_ir_main()"));
        assert!(emitted_code.contains("let mut usersession_state = State::new(\"UserSession\");"));
        assert!(emitted_code.contains("let mut usersession_state = allocate_heap_memory(1024);"));
        assert!(emitted_code.contains(
            "let signature = orda_pqc::mldsa_sign(&usersession_state, \"node_operator_key\");"
        ));
        assert!(emitted_code.contains("storage_engine::commit(&usersession_state);"));
    }

    #[test]
    fn test_e2e_llvm_emission() {
        let valid_payload = r#"{
            "tokens": [
                {
                    "root": { "type": "StateObject", "value": "UserSession" },
                    "morphs": ["AllocHeap", { "SignWithMLDSA": "node_operator_key" }, "WriteToTarget"]
                }
            ]
        }"#;

        let tokens = SemanticRouter::parse_intent_payload(valid_payload).unwrap();
        let emitted_ll = LlvmBackend::emit_module(&tokens);

        assert!(emitted_ll.contains("target datalayout = "));
        assert!(emitted_ll.contains("define i32 @qazaq_main() {"));
        assert!(emitted_ll.contains("%root_1 = call i8* @state_new()"));
        assert!(emitted_ll.contains("%var_2 = call i8* @allocate_heap_memory(i32 1024)"));
        assert!(emitted_ll.contains(
            "%var_3 = call i8* @orda_pqc_mldsa_sign(i8* %var_2, i8* @node_operator_key)"
        ));
        assert!(emitted_ll.contains("call void @storage_engine_commit(i8* %var_2)"));
    }
}
