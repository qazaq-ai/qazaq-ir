# Core Engine: The Heart of Qazaq IR

## O(1) Bitwise Validation

Qazaq IR is built for massive scale, targeting **100k+ Transactions Per Second (TPS)**. 
To achieve this, the IR compiler does not waste clock cycles iterating over arrays. We use a **Bitwise State Machine**.

As Morphemes attach to a Root, they flip deterministic bits natively on the processor register:

```rust
pub struct StateFlags: u32 {
    const IS_ALLOCATED = 1 << 0;
    const IS_MUTABLE   = 1 << 1;
    const IS_SIGNED    = 1 << 2;
    const IS_STREAMING = 1 << 3;
}
```

Validation (`is_compatible`) takes absolute O(1) time. 
If an LLM attempts to hallucinate—for example, trying to `WriteToTarget` before applying `AllocHeap`—the bitwise `AND` operation fails, instantly rejecting the token at the source.

---

## The Semantic Router (Constrained Decoding)

LLMs are probabilistic, but compilers must be deterministic. The **Semantic Router** serves as an unbreakable boundary ("straightjacket") between the two.

1. **Extraction:** The LLM receives natural language ("Create a session, allocate memory, sign and write") and acts merely as a Semantic Extractor.
2. **Constrained Schema:** It outputs a strict JSON payload defined by the Qazaq Lexer.
3. **Instant Validation:** The `SemanticRouter` processes the JSON string, and hands the typed `AgglutinativeToken` structs to the O(1) Lexer.

From English prompt to executed machine code without ASTs, hallucinations, or O(n^2) context windows. Welcome to the future.
