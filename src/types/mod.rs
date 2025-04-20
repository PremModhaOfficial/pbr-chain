pub mod block;
pub mod blockchain;
pub mod dev_test;
pub mod utils;

pub mod web_test;

pub mod web_types {
    use std::{collections::HashMap, sync::Mutex};

    use serde::{Deserialize, Serialize};

    use super::blockchain::BlockChain;

    #[derive(Serialize, Deserialize)]
    pub struct AppState {
        pub blockchain: Mutex<BlockChain<HashMap<String, String>>>,
    }

impl AppState {
    pub fn new(blockchain: Mutex<BlockChain<HashMap<String, String>>>) -> Self {
        Self { blockchain }
    }
}

}
