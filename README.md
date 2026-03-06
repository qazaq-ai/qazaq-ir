# Qazaq IR (Intermediate Representation)

<p align="center">
  <img src="./assets/shanraq_neuron.svg" alt="Qazaq IR Logo: The Shanraq Neuron" width="128"/>
</p>

> **The First Agglutinative Intermediate Representation for Deterministic LLM Code Generation.**

#### 🌌 The Philosophy of the "Shanraq Neuron"
Our logo is not just a brand; it is the visual embodiment of the Qazaq IR architecture:
1. **The Shanraq:** The circular framework symbolizes the "Шанырақ" (Shanraq) — the structural core of a yurt, representing unity, foundation, and an indestructible architectural constant (the **Root Entity**).
2. **The Neural Pathways:** The intersecting lines represent neurons and the deterministic flow of electrical signals along these pathways, mirroring how our model processes logic linearly ($\mathcal{O}(n)$) through isolated functional morphemes (the **Suffixes**).
3. **The Blood Red:** The deep red color signifies vital energy (Қан) and unyielding sovereignty. It reflects the operational philosophy of Qazaq IR: pure, secure, and hallucination-free AI performance.
Modern Large Language Models (LLMs) suffer from severe energy inefficiencies and code hallucinations due to their reliance on analytical languages (like English). Processing analytical context requires quadratic O(n^2) computational complexity in attention mechanisms.

**Qazaq IR** introduces a paradigm shift. Inspired by the strict mathematical rules of the agglutinative Kazak language, it provides a deterministic, linear O(n) intermediate compilation layer for AI. By assembling logic through immutable "roots" and isolated functional "suffixes" (morphemes) without hidden contexts, Qazaq IR eliminates code ambiguity and drastically reduces the energy cost of automated code generation.

## 📄 Documentation
* Live Documentation: **[Qazaq IR Vitepress Site](https://qazaq-ai.github.io/qazaq-ir/)**
* The Whitepaper: **[Read the Qazaq IR Whitepaper](./WHITEPAPER.md)**

## 🚀 Quick Start (CLI Compiler)

Qazaq IR comes with the `qazaqc` command-line tool to compile JSON intents directly into executable artifacts without Hidden Context.

### Valid Transaction (ML-DSA)
Use the included example to generate mathematically validated LLVM IR for a Post-Quantum signature transaction:
```bash
$ cargo run --bin qazaqc -- examples/01_pqc_transaction.json --output test.ll --emit llvm

=== Qazaq IR Compiler (qazaqc) v0.2.0 ===

Input Payload: examples/01_pqc_transaction.json
Emitting to: Llvm Target
» JSON loaded. Routing Intent...
» Intent topology mathematically validated. No hallucinations detected. (1 tokens)
» Generating LLVM IR via Backend...

SUCCESS: Successfully compiled into test.ll
Compilation Time: 554.167µs
=========================================
```

### Hallucination Protection
Attempting to compile an invalid LLM hallucination (e.g. writing data *before* allocating memory) fails safely in O(1) time:
```bash
$ cargo run --bin qazaqc -- examples/02_fatal_hallucination.json --output test.ll --emit llvm

=== Qazaq IR Compiler (qazaqc) v0.2.0 ===

Input Payload: examples/02_fatal_hallucination.json
Emitting to: Llvm Target
» JSON loaded. Routing Intent...

HALLUCINATION DETECTED: COMPILATION ABORTED

HallucinationDetected("FATAL HALLUCINATION: Suffix [WriteToTarget] illegally agglutinated to Root [StateObject(\"Transaction\")]. Intent rejected.")
```

## ⚖️ License & Copyright

**Qazaq IR operates under a Dual-Licensing Strategy:**

1. **The Whitepaper (`WHITEPAPER.md`)** is licensed under **[Creative Commons Attribution-NoDerivatives 4.0 International License (CC BY-ND 4.0)](https://creativecommons.org/licenses/by-nd/4.0/)**. You are free to share it as long as you provide appropriate credit to Daulet Baimurza. No derivative works are allowed.

2. **The Source Code & Compiler (`LICENSE`)** is licensed under the **Business Source License 1.1 (BSL)**. You may freely read, modify, and test the code for non-production use. Commercial deployment, managed hosting, or offering Qazaq IR infrastructure as a service requires a commercial license. The code will transition to the Apache License 2.0 on March 5, 2030.

**Copyright (c) 2026 Daulet Baimurza.**
