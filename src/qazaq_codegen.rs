use crate::morpheme_registry::{RootEntity, SuffixMorpheme};
use crate::qazaq_lexer::AgglutinativeToken;

/// The backend responsible for compiling the linear morphological chain
/// into an executable string (simulating LLVM IR or target code emission).
pub struct CodegenBackend;

impl CodegenBackend {
    /// Emit the final executable code block for a given agglutinative token.
    /// Operates deterministically by evaluating the Root and stacking the Suffix functions.
    pub fn emit_token(token: &AgglutinativeToken) -> String {
        let mut emit_buffer = String::new();

        // 1. Establish the immutable substrate (Root Entity)
        let root_var_name = match &token.root {
            RootEntity::MemoryPointer(addr) => format!("ptr_{}", addr),
            RootEntity::DatabaseTable(name) => format!("db_table_{}", name),
            RootEntity::NetworkSocket(port) => format!("socket_{}", port),
            RootEntity::StateObject(name) => {
                emit_buffer.push_str(&format!(
                    "// Initialize architectural StateObject: {}\n",
                    name
                ));
                emit_buffer.push_str(&format!(
                    "let mut {}_state = State::new(\"{}\");\n",
                    name.to_lowercase(),
                    name
                ));
                format!("{}_state", name.to_lowercase())
            }
        };

        // 2. Agglutinate the Suffix Primitives strictly sequentially
        for suffix in &token.morphs {
            match suffix {
                SuffixMorpheme::AllocHeap => {
                    emit_buffer.push_str("// [Suffix] AllocHeap: Reserving contiguous memory\n");
                    emit_buffer.push_str(&format!(
                        "let mut {} = allocate_heap_memory(1024);\n",
                        root_var_name
                    ));
                }
                SuffixMorpheme::MakeMutable => {
                    emit_buffer.push_str("// [Suffix] MakeMutable: Opening execution lifecycle\n");
                    emit_buffer.push_str(&format!("{}.make_mutable();\n", root_var_name));
                }
                SuffixMorpheme::SignWithMLDSA(key_alias) => {
                    emit_buffer.push_str(
                        "// [Suffix] SignWithMLDSA: Executing Post-Quantum Crypto Signature\n",
                    );
                    emit_buffer.push_str(&format!(
                        "let signature = orda_pqc::mldsa_sign(&{}, \"{}\");\n",
                        root_var_name, key_alias
                    ));
                }
                SuffixMorpheme::WriteToTarget => {
                    emit_buffer
                        .push_str("// [Suffix] WriteToTarget: Committing deterministic state\n");
                    emit_buffer.push_str(&format!("storage_engine::commit(&{});\n", root_var_name));
                }
                SuffixMorpheme::StreamData => {
                    emit_buffer.push_str("// [Suffix] StreamData: Opening network stream\n");
                    emit_buffer.push_str(&format!("net_layer::stream(&{});\n", root_var_name));
                }
                SuffixMorpheme::IterateUntilEmpty => {
                    emit_buffer.push_str("// [Suffix] IterateUntilEmpty: Temporal logic bound\n");
                    emit_buffer.push_str(&format!(
                        "while !{}.is_empty() {{\n    // Tick\n}}\n",
                        root_var_name
                    ));
                }
                SuffixMorpheme::VerifyConsensus => {
                    emit_buffer.push_str(
                        "// [Suffix] VerifyConsensus: Validating state via network consensus\n",
                    );
                    emit_buffer.push_str(&format!(
                        "consensus_layer::verify_state(&{});\n",
                        root_var_name
                    ));
                }
            }
        }

        emit_buffer
    }

    /// Emits the entire IR payload into a single executable bloc.
    pub fn emit_payload(tokens: &[AgglutinativeToken]) -> String {
        let mut final_code = String::from("fn qazaq_ir_main() {\n");

        for token in tokens {
            let token_code = Self::emit_token(token);
            for line in token_code.lines() {
                final_code.push_str(&format!("    {}\n", line));
            }
            final_code.push('\n');
        }

        final_code.push_str("}\n");
        final_code
    }
}
