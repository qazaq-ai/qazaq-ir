use qazaq_ir::RootEntity;
use sled::Db;
use std::path::Path;

/// The Persistent State Machine for Orda Node.
/// Stores the global balance map corresponding to each immutable `RootEntity` on disk.
pub struct State {
    db: Db,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        // Initialize or open the Sled database in the `orda_data` directory
        let db = sled::open(Path::new("orda_data")).expect("Failed to open Orda Node Sled DB");
        Self { db }
    }

    /// Helper to convert a RootEntity into a unique deterministic byte slice key
    fn entity_to_bytes(entity: &RootEntity) -> Vec<u8> {
        match entity {
            RootEntity::MemoryPointer(id) => id.to_be_bytes().to_vec(),
            // Future entity types can implement their own serialization
            _ => panic!("Only MemoryPointer is currently supported as a State key"),
        }
    }

    /// Retrieve the current balance for a deterministic entity.
    pub fn get_balance(&self, entity: &RootEntity) -> u64 {
        let key = Self::entity_to_bytes(entity);
        match self.db.get(&key) {
            Ok(Some(ivec)) => {
                let bytes: [u8; 8] = ivec.as_ref().try_into().unwrap_or([0; 8]);
                u64::from_be_bytes(bytes)
            }
            _ => 0, // Fallback to 0 if not found or corrupted
        }
    }

    /// Add amount to the specified deterministic entity's balance.
    pub fn add_balance(&mut self, entity: RootEntity, amount: u64) {
        let current = self.get_balance(&entity);
        let new_balance = current.saturating_add(amount);

        let key = Self::entity_to_bytes(&entity);
        let value = new_balance.to_be_bytes();

        self.db
            .insert(key, value.as_slice())
            .expect("Failed to write balance to Sled");
        self.db.flush().expect("Failed to flush to disk");
    }

    /// Subtract amount from the specified deterministic entity's balance.
    /// Returns `Err` if there are insufficient funds.
    pub fn sub_balance(&mut self, entity: &RootEntity, amount: u64) -> Result<(), String> {
        let current = self.get_balance(entity);
        if current < amount {
            return Err(format!(
                "Insufficient funds: expected {}, got {}",
                amount, current
            ));
        }

        let new_balance = current - amount;
        let key = Self::entity_to_bytes(entity);
        let value = new_balance.to_be_bytes();

        self.db
            .insert(key, value.as_slice())
            .expect("Failed to write balance to Sled");
        self.db.flush().expect("Failed to flush to disk");
        Ok(())
    }
}
