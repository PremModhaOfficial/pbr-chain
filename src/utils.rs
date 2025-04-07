use std::time::{SystemTime, UNIX_EPOCH};

use sha2::{Digest, Sha256};

use crate::block::{Block, DefaultDebug};

pub fn current_time_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time problem")
        .as_micros() as u64
}

pub fn generate_hash<T: DefaultDebug>(input: &Block<T>) -> String {
    let mut hasher = Sha256::new();
    let input = format!(
        "|{:?}|{:?}|{:?}|",
        input.timestamp_milis, input.last_hash, input.data
    );
    hasher.update(input.as_bytes());

    let res = hasher.finalize();
    format!("{:x}", res)
}
