use crate::mempool::TransactionPool;
use crate::state::State;
use colored::Colorize;
use qazaq_ir::{AgglutinativeToken, RootEntity, SuffixMorpheme};

use std::sync::{Arc, Mutex};
use tokio::time::{Duration, sleep};

/// The execution layer strictly responsible for draining the Transaction Pool
/// and applying deterministic actions to the unified State Machine.
pub struct ExecutionEngine;

impl ExecutionEngine {
    /// Placed on a background `tokio::spawn` task, endlessly drains the mempool
    pub async fn run_loop(mempool: Arc<Mutex<TransactionPool>>, state: Arc<Mutex<State>>) {
        loop {
            let tokens = {
                let mut pool = mempool.lock().unwrap();
                let drained = pool.drain_all();
                drop(pool); // explicitly drop to unlock for Axum
                drained
            };

            if tokens.is_empty() {
                // Sleep to avoid CPU hogging and YIELD to the tokio scheduler
                sleep(Duration::from_millis(500)).await;
                continue;
            }
            println!(
                "⚙️  [Engine] Drained {} token(s) from Mempool",
                tokens.len()
            );

            println!(
                "\n{}",
                "=== Execution Engine: Initializing Block Processing ==="
                    .bold()
                    .blue()
            );

            let mut successful_txs = 0;
            let mut state_lock = state.lock().unwrap();

            for token in tokens {
                if Self::execute_token(&token, &mut state_lock) {
                    successful_txs += 1;
                }
            }

            println!(
                "\n{}",
                format!(
                    "=== Execution Engine: Processed {} Transactions ===",
                    successful_txs
                )
                .bold()
                .green()
            );
        }
    }

    /// Parses and directly executes a single Qazaq IR morphological sequence in O(1) time.
    /// Fails fast and silently discards token execution if the topological requirements are violated.
    fn execute_token(token: &AgglutinativeToken, state: &mut State) -> bool {
        // Validation Rule 1: Security Must exist explicitly
        let is_signed = token
            .morphs
            .iter()
            .any(|s| matches!(s, SuffixMorpheme::SignWithMLDSA(_)));
        if !is_signed {
            println!(
                "    {} Transaction Rejected: Missing Post-Quantum Signature",
                "[X]".red()
            );
            return false;
        }

        // Processing Logic based on Root Type
        match &token.root {
            RootEntity::MemoryPointer(address) => {
                // Example hardcoded execution logic: Mint 100 on valid MemoryPointer transaction
                let minted_amount = 100;
                state.add_balance(token.root.clone(), minted_amount);

                println!(
                    "    {} Added {} units to memory pointer {}. Target Final Balance: {}",
                    "[+]".green(),
                    minted_amount.to_string().yellow(),
                    address,
                    state.get_balance(&token.root).to_string().cyan()
                );

                true
            }
            RootEntity::DatabaseTable(query) => {
                println!(
                    "    {} Simulating Database query updates for {}...",
                    "[~]".bright_blue(),
                    query.magenta()
                );
                true // Logic simulation
            }
            RootEntity::NetworkSocket(_) | RootEntity::StateObject(_) => {
                println!(
                    "    {} Root entity not recognized as an actionable transaction target.",
                    "[?]".dimmed()
                );
                false
            }
        }
    }
}
