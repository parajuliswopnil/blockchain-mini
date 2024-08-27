use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Executor {
    pub transaction_limit_per_block: u32,
    pub transaction_size_per_block: u32,
}

impl Executor {
    pub fn new(transaction_limit_per_block: u32, transaction_size_per_block: u32) -> Executor {
        Executor {
            transaction_limit_per_block,
            transaction_size_per_block,
        }
    }

    pub fn get_tx_limit_per_block(&self) -> u32 {
        self.transaction_limit_per_block
    }

    pub fn get_transaction_size_per_block(&self) -> u32 {
        self.transaction_size_per_block
    }
}
