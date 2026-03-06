use clap::{Parser, ValueEnum};
use colored::*;
use qazaq_ir::{CodegenBackend, LlvmBackend, SemanticRouter};
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

/// Qazaq IR Compiler (qazaqc)
/// Deterministic O(1) compiler for Sovereign AI & Hallucination-Free Code Generation.
#[derive(Parser, Debug)]
#[command(name = "qazaqc")]
#[command(version = "0.2.0")]
#[command(about = "Compiles Qazaq IR JSON intents into executable representations without hidden context.", long_about = None)]
struct Cli {
    /// Path to the input JSON file containing the Intent Payload
    #[arg(required_unless_present = "emit_schema")]
    input: Option<PathBuf>,

    /// Output file path (e.g. output.ll or output.rs)
    #[arg(short, long, required_unless_present = "emit_schema")]
    output: Option<PathBuf>,

    /// Target backend to emit (llvm or rust)
    #[arg(short, long, value_enum, required_unless_present = "emit_schema")]
    emit: Option<EmitTarget>,

    /// Export the required JSON Schema for LLM tool integration
    #[arg(long)]
    emit_schema: bool,
}

#[derive(ValueEnum, Clone, Debug)]
enum EmitTarget {
    Llvm,
    Rust,
}

fn main() {
    let cli = Cli::parse();

    if cli.emit_schema {
        println!("{}", qazaq_ir::LlmBridge::generate_ai_schema());
        return;
    }

    let input_path = cli.input.unwrap();
    let output_path = cli.output.unwrap();
    let emit_target = cli.emit.unwrap();

    println!(
        "\n{}\n",
        "=== Qazaq IR Compiler (qazaqc) v0.2.0 ===".bold().cyan()
    );
    println!("{} {}", "Input Payload:".bold(), input_path.display());
    println!("{} {:?} Target", "Emitting to:".bold(), emit_target);

    // Track total compilation time (to highlight our O(1) speed)
    let start_time = Instant::now();

    // 1. Read input JSON
    let json_content = match fs::read_to_string(&input_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{} Failed to read input file: {}", "ERROR:".red().bold(), e);
            std::process::exit(1);
        }
    };

    println!("{} JSON loaded. Routing Intent...", "»".cyan());

    // 2. Lex & Parse via Semantic Router (O(1) validation)
    let tokens = match SemanticRouter::parse_intent_payload(&json_content) {
        Ok(t) => t,
        Err(e) => {
            eprintln!(
                "\n{}\n",
                "HALLUCINATION DETECTED: COMPILATION ABORTED"
                    .red()
                    .bold()
                    .on_black()
            );
            eprintln!("{:?}", e);
            std::process::exit(1);
        }
    };

    println!(
        "{} Intent topology mathematically validated. No hallucinations detected. ({} tokens)",
        "»".green(),
        tokens.len()
    );

    // 3. Emit via requested backend
    let emitted_code = match emit_target {
        EmitTarget::Llvm => {
            println!("{} Generating LLVM IR via Backend...", "»".cyan());
            LlvmBackend::emit_module(&tokens)
        }
        EmitTarget::Rust => {
            println!("{} Generating Rust Code via Backend...", "»".cyan());
            CodegenBackend::emit_payload(&tokens)
        }
    };

    // 4. Write output file
    if let Err(e) = fs::write(&output_path, emitted_code) {
        eprintln!(
            "{} Failed to write output file: {}",
            "ERROR:".red().bold(),
            e
        );
        std::process::exit(1);
    }

    let elapsed = start_time.elapsed();

    println!(
        "\n{} Successfully compiled into {}",
        "SUCCESS:".green().bold(),
        output_path.display().to_string().yellow()
    );
    println!("{} {:?}", "Compilation Time:".bold(), elapsed);
    println!("{}\n", "=========================================".cyan());
}
