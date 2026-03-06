pub mod mempool;

use colored::*;
use mempool::TransactionPool;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!(
        "{}",
        "=== Orda Node (Post-Quantum Blockchain) ===".bold().cyan()
    );
    println!(
        "{}",
        "Initializing Genesis State and Mempool...".bold().black()
    );

    let mut mempool = TransactionPool::new();

    // In a real blockchain, this would be an API server receiving REST/RPC calls.
    let intent_file = PathBuf::from("crates/qazaq-ir/examples/01_pqc_transaction.json");

    println!(
        "{} Receiving raw AI-generated payload from P2P network...",
        "»".yellow()
    );

    let raw_payload = fs::read_to_string(&intent_file).unwrap_or_else(|_| {
        eprintln!("Failed to read simulated payload file.");
        std::process::exit(1);
    });

    println!(
        "{} Routing payload through Qazaq IR Mathematical Validation...",
        "»".yellow()
    );

    // Mempool attempts to process the raw string intent
    match mempool.process_incoming_intent(&raw_payload) {
        Ok(_) => {
            println!(
                "{} Transaction intent mathematically verified in O(1) time.",
                "SUCCESS:".green().bold()
            );
            println!(
                "{} Explicit SignWithMLDSA cryptographic suffix proven.",
                "SUCCESS:".green().bold()
            );
            println!(
                "{} Current Unconfirmed Txs: {}",
                "»".cyan(),
                mempool.unconfirmed_count()
            );
        }
        Err(e) => {
            eprintln!("{} {}", "NODE REJECT:".red().bold(), e);
        }
    }

    // Now test a hallucination
    println!(
        "\n{} Simulating hostile hallucinated payload...",
        "»".yellow()
    );
    let hostile_file = PathBuf::from("crates/qazaq-ir/examples/02_fatal_hallucination.json");
    if let Ok(hostile_payload) = fs::read_to_string(&hostile_file) {
        match mempool.process_incoming_intent(&hostile_payload) {
            Ok(_) => println!("Uh oh. Hallucination bypassed protection."),
            Err(e) => {
                println!("{} {}", "DEFENDED:".green().bold(), e);
                println!("Node state is pure. Hallucination blocked at the IR layer.");
            }
        }
    }
}
