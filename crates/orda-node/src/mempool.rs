use qazaq_ir::{AgglutinativeToken, RouterError, SemanticRouter, SuffixMorpheme};

/// The Simulated Transaction Pool (Mempool) for Orda Node.
/// Stores mathematically validated and zero-context protected intents.
pub struct TransactionPool {
    verified_transactions: Vec<AgglutinativeToken>,
}

impl TransactionPool {
    pub fn new() -> Self {
        Self {
            verified_transactions: Vec::new(),
        }
    }

    /// Processes an incoming raw JSON intent received from an external DApp/LLM client.
    /// Passes the string through the Semantic Router for O(1) topological and hallucination verification.
    /// Explicitly requires the `SignWithMLDSA` suffix for PQC assurance.
    pub fn process_incoming_intent(&mut self, raw_json: &str) -> Result<(), String> {
        // 1. Route and Validate in O(1) mathematically
        let tokens = SemanticRouter::parse_intent_payload(raw_json).map_err(|e| match e {
            RouterError::DeserializationFailed(msg) => format!("Mempool Reject (Format): {}", msg),
            RouterError::HallucinationDetected(msg) => {
                format!("Mempool Reject (Hallucination): {}", msg)
            }
        })?;

        // 2. Enforce Post-Quantum Cryptography logic for Orda Node
        for token in &tokens {
            let has_valid_pqc = token
                .morphs
                .iter()
                .any(|suffix| matches!(suffix, SuffixMorpheme::SignWithMLDSA(_)));

            if !has_valid_pqc {
                return Err("Mempool Reject (Security): Transaction missing explicit SignWithMLDSA cryptographic suffix.".into());
            }
        }

        // 3. Commit to Mempool
        self.verified_transactions.extend(tokens);
        Ok(())
    }

    pub fn unconfirmed_count(&self) -> usize {
        self.verified_transactions.len()
    }
}
