# CLI Compiler (`qazaqc`)

The Qazaq IR Engine can be used as a standalone executable compiler, bringing deterministic O(1) compilation directly to the terminal.

## Installation

Compile the tool directly using Cargo:

```bash
cargo build --release
```

The resulting binary `target/release/qazaqc` can be moved to your system `$PATH` for global use.

## Usage

The `qazaqc` tool accepts a JSON payload containing the intent tokens and emits executable representations using the specified compiler backend.

```bash
qazaqc <JSON_INPUT_FILE> --emit <backend> -o <OUTPUT_FILE>
```

### Compiler Backends
- `--emit llvm`: Emits bare-metal, Static Single Assignment (SSA) `.ll` LLVM Intermediate Representation. Best for low-level node execution and avoiding heavy FFI dependencies.
- `--emit rust`: Emits safe Rust code representing the same logical operations without dynamic memory overhead.

### Example

Input payload `intent.json`:
```json
{
    "tokens": [
        {
            "root": { "type": "StateObject", "value": "Transaction" },
            "morphs": [
                "AllocHeap", 
                { "SignWithMLDSA": "operator_cold_key" }
            ]
        }
    ]
}
```

Running the compiler:
```bash
$ qazaqc intent.json --emit llvm -o tx.ll

=== Qazaq IR Compiler (qazaqc) v0.2.0 ===

Input Payload: intent.json
Emitting to: Llvm Target
» JSON loaded. Routing Intent...
» Intent topology mathematically validated. No hallucinations detected. (1 tokens)
» Generating LLVM IR via Backend...

SUCCESS: Successfully compiled into tx.ll
Compilation Time: 554.167µs
=========================================
```

As demonstrated, the execution operates within microseconds, proving the linear algorithm's unmatched performance for LLM-driven environments.

### Hallucination Protection

Attempting to compile an invalid LLM hallucination (such as writing data *before* memory allocation) stops instantly in O(1) time before generating any code:

```bash
$ qazaqc examples/02_fatal_hallucination.json --emit llvm -o test.ll

=== Qazaq IR Compiler (qazaqc) v0.2.0 ===

Input Payload: examples/02_fatal_hallucination.json
Emitting to: Llvm Target
» JSON loaded. Routing Intent...

HALLUCINATION DETECTED: COMPILATION ABORTED

HallucinationDetected("FATAL HALLUCINATION: Suffix [WriteToTarget] illegally agglutinated to Root [StateObject(\"Transaction\")]. Intent rejected.")
```
