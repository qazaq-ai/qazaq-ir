use crate::morpheme_registry::{MorphemeRegistry, RootEntity, SuffixMorpheme};
use serde::{Deserialize, Serialize};

/// The fundamental building block of Qazaq IR Architecture.
/// Replaces traditional AST nodes with a flat, structurally guaranteed agglutinative chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgglutinativeToken {
    pub root: RootEntity,
    pub morphs: Vec<SuffixMorpheme>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IRPayload {
    pub tokens: Vec<AgglutinativeToken>,
}

pub struct QazaqLexer;

impl QazaqLexer {
    /// Parses a JSON Payload (simulating Intent extraction from LLM Semantic Router)
    /// into an in-memory Intermediate Representation and validates the agglutinative constraints.
    /// Operates in strict O(n) linear time.
    pub fn parse_and_validate(json_payload: &str) -> Result<Vec<AgglutinativeToken>, String> {
        let payload: IRPayload =
            serde_json::from_str(json_payload).map_err(|e| format!("Parsing error: {}", e))?;

        // O(n) Linear compilation and morphological validation
        for token in &payload.tokens {
            let mut validated_morphs = Vec::new();

            for suffix in &token.morphs {
                // O(1) Compatibility Check preventing structural hallucinations entirely
                if !MorphemeRegistry::is_compatible(&token.root, &validated_morphs, suffix) {
                    return Err(format!(
                        "FATAL HALLUCINATION: Suffix '{:?}' cannot be mathematically agglutinated to Root '{:?}' with preceding morphs {:?}",
                        suffix, token.root, validated_morphs
                    ));
                }
                validated_morphs.push(suffix.clone());
            }
        }

        // If execution reaches here, the semantic map is perfectly deterministic.
        Ok(payload.tokens)
    }
}
