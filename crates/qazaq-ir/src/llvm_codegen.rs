use crate::morpheme_registry::{RootEntity, SuffixMorpheme};
use crate::qazaq_lexer::AgglutinativeToken;

/// The backend responsible for compiling the linear morphological chain
/// into raw, human-readable LLVM IR (.ll) text format.
pub struct LlvmBackend;

impl LlvmBackend {
    /// Emit a deterministic Sequence of SSA Instructions for a given token
    pub fn emit_token(token: &AgglutinativeToken, var_counter: &mut usize) -> String {
        let mut emit_buffer = String::new();

        // 1. Establish the immutable substrate (Root Entity)
        let root_ptr = match &token.root {
            RootEntity::MemoryPointer(addr) => {
                let ptr = format!("%root_{}", var_counter);
                *var_counter += 1;
                emit_buffer.push_str(&format!("  {} = inttoptr i64 {} to i8*\n", ptr, addr));
                ptr
            }
            RootEntity::DatabaseTable(name) => {
                let ptr = format!("%root_{}", var_counter);
                *var_counter += 1;
                emit_buffer.push_str(&format!("  ; Fetching DB Table {}\n", name));
                emit_buffer.push_str(&format!("  {} = call i8* @db_table_init()\n", ptr));
                ptr
            }
            RootEntity::NetworkSocket(port) => {
                let ptr = format!("%root_{}", var_counter);
                *var_counter += 1;
                emit_buffer.push_str(&format!(
                    "  {} = call i8* @socket_bind(i16 {})\n",
                    ptr, port
                ));
                ptr
            }
            RootEntity::StateObject(name) => {
                let ptr = format!("%root_{}", var_counter);
                *var_counter += 1;
                emit_buffer.push_str(&format!(
                    "  ; Initialize architectural StateObject: {}\n",
                    name
                ));
                emit_buffer.push_str(&format!("  {} = call i8* @state_new()\n", ptr));
                ptr
            }
        };

        // Track the current acting pointer as we mutate it through suffixes
        let mut current_ptr = root_ptr;

        // 2. Agglutinate the Suffix Primitives strictly sequentially
        for suffix in &token.morphs {
            match suffix {
                SuffixMorpheme::AllocHeap => {
                    let new_ptr = format!("%var_{}", var_counter);
                    *var_counter += 1;
                    emit_buffer.push_str("  ; [Suffix] AllocHeap: Reserving contiguous memory\n");
                    emit_buffer.push_str(&format!(
                        "  {} = call i8* @allocate_heap_memory(i32 1024)\n",
                        new_ptr
                    ));
                    current_ptr = new_ptr;
                }
                SuffixMorpheme::MakeMutable => {
                    emit_buffer.push_str("  ; [Suffix] MakeMutable: Opening execution lifecycle\n");
                    emit_buffer
                        .push_str(&format!("  call void @make_mutable(i8* {})\n", current_ptr));
                }
                SuffixMorpheme::SignWithMLDSA(key_alias) => {
                    let sig_ptr = format!("%var_{}", var_counter);
                    let alias_str = format!("@{}", key_alias);
                    *var_counter += 1;
                    emit_buffer.push_str(
                        "  ; [Suffix] SignWithMLDSA: Executing Post-Quantum Crypto Signature\n",
                    );

                    // Emulate declaring the key global string if needed, or simply pass the named pointer.
                    emit_buffer.push_str(&format!(
                        "  {} = call i8* @orda_pqc_mldsa_sign(i8* {}, i8* {})\n",
                        sig_ptr, current_ptr, alias_str
                    ));
                    // Operations usually continue on the root pointer for state mutations
                }
                SuffixMorpheme::WriteToTarget => {
                    emit_buffer
                        .push_str("  ; [Suffix] WriteToTarget: Committing deterministic state\n");
                    emit_buffer.push_str(&format!(
                        "  call void @storage_engine_commit(i8* {})\n",
                        current_ptr
                    ));
                }
                SuffixMorpheme::StreamData => {
                    emit_buffer.push_str("  ; [Suffix] StreamData: Opening network stream\n");
                    emit_buffer.push_str(&format!(
                        "  call void @net_layer_stream(i8* {})\n",
                        current_ptr
                    ));
                }
                SuffixMorpheme::IterateUntilEmpty => {
                    let loop_bb = format!("loop_{}", var_counter);
                    let end_bb = format!("end_{}", var_counter);
                    let cond_val = format!("%var_{}", var_counter);
                    *var_counter += 1;
                    emit_buffer.push_str("  ; [Suffix] IterateUntilEmpty: Temporal logic bound\n");
                    emit_buffer.push_str(&format!("  br label %{}\n\n", loop_bb));

                    emit_buffer.push_str(&format!("{}:\n", loop_bb));
                    emit_buffer.push_str(&format!(
                        "  {} = call i1 @is_empty(i8* {})\n",
                        cond_val, current_ptr
                    ));
                    emit_buffer.push_str(&format!(
                        "  br i1 {}, label %{}, label %{}\n\n",
                        cond_val, end_bb, loop_bb
                    ));

                    emit_buffer.push_str(&format!("{}:\n", end_bb));
                }
                SuffixMorpheme::BranchIfValid => {
                    let valid_bb = format!("valid_{}", var_counter);
                    let invalid_bb = format!("invalid_{}", var_counter);
                    let cond_val = format!("%var_{}", var_counter);
                    *var_counter += 1;
                    emit_buffer.push_str("  ; [Suffix] BranchIfValid: Conditional guard\n");
                    emit_buffer.push_str(&format!(
                        "  {} = call i1 @is_valid(i8* {})\n",
                        cond_val, current_ptr
                    ));
                    emit_buffer.push_str(&format!(
                        "  br i1 {}, label %{}, label %{}\n\n",
                        cond_val, valid_bb, invalid_bb
                    ));

                    emit_buffer.push_str(&format!("{}:\n", valid_bb));
                    // In a strictly linear IR, invalid_bb would represent the end of the conditionally evaluated block.
                }
                SuffixMorpheme::VerifyConsensus => {
                    emit_buffer.push_str(
                        "  ; [Suffix] VerifyConsensus: Validating state via network consensus\n",
                    );
                    emit_buffer.push_str(&format!(
                        "  call void @consensus_layer_verify_state(i8* {})\n",
                        current_ptr
                    ));
                }
            }
        }

        emit_buffer
    }

    /// Emits the entire IR payload into a single executable LLVM module string.
    pub fn emit_module(tokens: &[AgglutinativeToken]) -> String {
        let mut final_code = String::new();

        // 1. Module Header and External Declarations
        final_code.push_str("; ModuleID = 'qazaq_ir_module'\n");
        final_code.push_str("source_filename = \"qazaq-ir\"\n");
        final_code.push_str(
            "target datalayout = \"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"\n",
        );
        final_code.push_str("target triple = \"x86_64-unknown-linux-gnu\"\n\n");

        final_code.push_str("declare i8* @db_table_init()\n");
        final_code.push_str("declare i8* @socket_bind(i16)\n");
        final_code.push_str("declare i8* @state_new()\n");
        final_code.push_str("declare i8* @allocate_heap_memory(i32)\n");
        final_code.push_str("declare void @make_mutable(i8*)\n");
        final_code.push_str("declare i8* @orda_pqc_mldsa_sign(i8*, i8*)\n");
        final_code.push_str("declare void @storage_engine_commit(i8*)\n");
        final_code.push_str("declare void @net_layer_stream(i8*)\n");
        final_code.push_str("declare i1 @is_empty(i8*)\n");
        final_code.push_str("declare i1 @is_valid(i8*)\n");
        final_code.push_str("declare void @consensus_layer_verify_state(i8*)\n\n");

        // 2. Main Execution Block
        final_code.push_str("define i32 @qazaq_main() {\n");
        final_code.push_str("entry:\n");

        let mut var_counter = 1;
        for token in tokens {
            let token_code = Self::emit_token(token, &mut var_counter);
            final_code.push_str(&token_code);
            final_code.push('\n');
        }

        final_code.push_str("  ret i32 0\n");
        final_code.push_str("}\n");
        final_code
    }
}
