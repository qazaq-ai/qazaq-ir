use crate::qazaq_lexer::{AgglutinativeToken, QazaqLexer};
use serde::{Deserialize, Serialize};

/// The rigid Semantic Router acting as a "straightjacket" for LLM generated intents.
/// It strictly defines what JSON is accepted from the model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouterPayload {
    pub tokens: Vec<AgglutinativeToken>,
}

#[derive(Debug)]
pub enum RouterError {
    DeserializationFailed(String),
    HallucinationDetected(String),
}

pub struct SemanticRouter;

impl SemanticRouter {
    /// Takes a raw JSON string from an LLM, parses it directly into structured definitions,
    /// and immediately validates it mathematically via the QazaqLexer.
    pub fn parse_intent_payload(raw_json: &str) -> Result<Vec<AgglutinativeToken>, RouterError> {
        // 1. Constrained Decoding: Enforce JSON Schema (LLM acts purely as a Semantic Extractor)
        let payload: RouterPayload = serde_json::from_str(raw_json).map_err(|e| {
            RouterError::DeserializationFailed(format!("Schema validation failed: {}", e))
        })?;

        // 2. Deterministic Validation: Send strictly typed structs to the O(1) QazaqLexer pipeline
        QazaqLexer::validate_tokens(payload.tokens)
            .map_err(|e| RouterError::HallucinationDetected(e))
    }
}
