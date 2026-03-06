use crate::morpheme_registry::{MorphemeRegistry, RootEntity, SuffixMorpheme};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// The fundamental building block of Qazaq IR Architecture.
/// Replaces traditional AST nodes with a flat, structurally guaranteed agglutinative chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct AgglutinativeToken {
    pub root: RootEntity,
    pub morphs: Vec<SuffixMorpheme>,
}

pub struct QazaqLexer;

impl QazaqLexer {
    /// Validates the agglutinative constraints of already parsed tokens in-memory.
    /// Operates in strict O(n) linear time across tokens, and O(1) bitwise validation for each suffix.
    pub fn validate_tokens(
        tokens: Vec<AgglutinativeToken>,
    ) -> Result<Vec<AgglutinativeToken>, String> {
        // O(n) Linear compilation and morphological validation
        for token in &tokens {
            // O(1) State Machine accumulation
            let mut current_state = crate::morpheme_registry::StateFlags::empty();
            let mut validated_morphs = Vec::new();

            for suffix in &token.morphs {
                // Absolute O(1) Compatibility Check based on registers
                if !MorphemeRegistry::is_compatible(&token.root, current_state, suffix) {
                    return Err(format!(
                        "FATAL HALLUCINATION: Suffix '{:?}' cannot be mathematically agglutinated to Root '{:?}' at State '{:?}' (preceding morphs: {:?})",
                        suffix, token.root, current_state, validated_morphs
                    ));
                }

                // Mutate state with Bitwise OR
                current_state |= suffix.as_flag();
                validated_morphs.push(suffix.clone());
            }
        }

        // If execution reaches here, the semantic map is perfectly deterministic.
        Ok(tokens)
    }
}
