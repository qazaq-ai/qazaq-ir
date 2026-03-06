use crate::semantic_router::RouterPayload;

/// The LLM Bridge: Provides a mathematically strict interface for integrating
/// external AI models (OpenAI, Anthropic, etc.) with the Qazaq IR Engine.
pub struct LlmBridge;

impl LlmBridge {
    /// Generates a strict JSON Schema representing the exact topographical constraints
    /// of the `RouterPayload`. Inject this into an LLM's `tools` or `response_format` JSON.
    pub fn generate_ai_schema() -> String {
        let schema = schemars::schema_for!(RouterPayload);
        serde_json::to_string_pretty(&schema).expect("Failed to serialize schema")
    }
}
