use serde::{Deserialize, Serialize};

/// Roots - fundamental execution targets acting as immutable substrates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum RootEntity {
    MemoryPointer(usize),
    DatabaseTable(String),
    NetworkSocket(u16),
    StateObject(String), // Abstract business entity
}

/// Suffixes - strictly isolated functions with a single zone of responsibility.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum SuffixMorpheme {
    // Memory & State Management
    AllocHeap,
    MakeMutable,
    
    // I/O & Actions
    WriteToTarget,
    StreamData,
    
    // Cryptography & Protocols (PQC Ready)
    SignWithMLDSA,
    VerifyConsensus,
    
    // Control Flow (Temporal markers)
    IterateUntilEmpty,
}

/// The core registry emulating the Compatibility Matrix of the Kazak algorithm.
/// Defines strict topological rules for suffix agglutination.
pub struct MorphemeRegistry;

impl MorphemeRegistry {
    /// Validates if a suffix can be logically applied to the current state of a root,
    /// traversing the pre-existing suffix chain to guarantee Zero Hidden Context.
    /// Returns true if the topology is valid, false if it's a structural hallucination.
    pub fn is_compatible(root: &RootEntity, current_suffixes: &[SuffixMorpheme], new_suffix: &SuffixMorpheme) -> bool {
        match new_suffix {
            SuffixMorpheme::WriteToTarget => {
                // To write to memory or state, it must have been allocated or made mutable first
                match root {
                    RootEntity::DatabaseTable(_) | RootEntity::NetworkSocket(_) => true,
                    RootEntity::MemoryPointer(_) | RootEntity::StateObject(_) => {
                        current_suffixes.contains(&SuffixMorpheme::MakeMutable) || 
                        current_suffixes.contains(&SuffixMorpheme::AllocHeap)
                    }
                }
            }
            SuffixMorpheme::MakeMutable => {
                // A state cannot be made mutable twice in a row without reason
                !current_suffixes.contains(&SuffixMorpheme::MakeMutable) 
            }
            SuffixMorpheme::SignWithMLDSA => {
                // Cryptographic signatures are primarily applied to abstract State Objects or Mem Pointers
                matches!(root, RootEntity::StateObject(_) | RootEntity::MemoryPointer(_))
            }
            SuffixMorpheme::StreamData => {
                // Streaming is typically bounded to Network Sockets
                matches!(root, RootEntity::NetworkSocket(_))
            }
            // By default, other morphemes have unconstrained agglutination in this MVP scope
            _ => true,
        }
    }
}
