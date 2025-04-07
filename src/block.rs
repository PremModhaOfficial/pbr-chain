use std::fmt::Debug;

use crate::utils::{current_time_millis, generate_hash};

// Define a trait that combines Default and Debug
pub trait DefaultDebug: Default + Debug + PartialEq + Clone {}

// Implement it for all types that satisfy both bounds
impl<T> DefaultDebug for T where T: Default + Debug + PartialEq + Clone {}

#[derive(Debug, PartialEq, Clone )]
pub struct Block<T: DefaultDebug> {
    pub timestamp_milis: u64,
    pub last_hash: Option<String>,
    pub hash: Option<String>,
    pub data: T,
}

impl<T: DefaultDebug> Default for Block<T> {
    fn default() -> Self {
        let mut genesis = Self {
            timestamp_milis: current_time_millis(),
            last_hash: None,
            hash: None,
            data: Default::default(),
        };
        genesis.hash = Some(generate_hash(&genesis));

        genesis
    }
}

impl<T: DefaultDebug> Block<T> {
    pub fn new(timestamp_milis: u64, last_hash: &str, hash: &str, data: T) -> Self {
        Self {
            timestamp_milis,
            last_hash: Some(last_hash.to_string()),
            hash: Some(hash.to_string()),
            data,
        }
    }

    pub fn genesis() -> Self {
        Block::default()
    }

    pub fn mine_block(last_block: &Block<T>, data: T) -> Result<Block<T>, String> {
        let timestamp_milis = current_time_millis();
        if let Some(last_hash) = &last_block.hash {
            let hash = generate_hash(last_block);
            return Ok(Block::new(timestamp_milis, last_hash, &hash, data));
        }

        Err("Problem Mining Block".to_string())
    }

    pub fn hash(&self) -> Option<String> {
        self.hash.as_ref().cloned()
    }
}
