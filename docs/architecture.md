# Architecture: The O(1) Agglutinative IR

## Farewell ASTs. Welcome Linear Chains.

Traditional compilers rely on Abstract Syntax Trees (ASTs). Maintaining and validating these deep trees under heavy loads requires recursive traversal, adding significant latency. 

Qazaq IR completely abandons the AST in favor of a linear `AgglutinativeToken` structure.

```rust
pub struct AgglutinativeToken {
    pub root: RootEntity,
    pub morphs: Vec<SuffixMorpheme>,
}
```

### 1. Root Entity
The root acts as an indestructible structural constant (the Shanraq). It defines *what* is being manipulated.
Types include `StateObject`, `MemoryPointer`, `DatabaseTable`, and `NetworkSocket`.

### 2. Suffix Morphemes
Morphemes are strictly isolated functional extensions. They carry zero hidden context. Every suffix executed transforms the state deterministically:
- `AllocHeap`
- `MakeMutable`
- `SignWithMLDSA`
- `WriteToTarget`

By stacking suffixes linearly, we eliminate ambiguity and achieve theoretical operational perfection.
