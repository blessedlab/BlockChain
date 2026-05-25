use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::transaction::Transaction;
pub const MINING_REWARD: f64 = 50.0;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub index: u64,

    pub timestamp: u64,

    pub transactions: Vec<Transaction>,

    pub previous_hash: String,

    pub nonce: u64,

    pub hash: String,
}

impl Block {
    pub fn new(
        index: u64,
        transactions: Vec<Transaction>,
        previous_hash: String,
    ) -> Self {
        let timestamp = Utc::now().timestamp() as u64;

        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            nonce: 0,
            hash: String::new(),
        };

        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let tx_data = serde_json::to_string(&self.transactions)
            .unwrap_or_default();

        let content = format!(
            "{}{}{}{}{}",
            self.index,
            self.timestamp,
            tx_data,
            self.previous_hash,
            self.nonce
        );

        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let result = hasher.finalize();

        hex::encode(result)
    }

    pub fn has_valid_hash(&self) -> bool {
        self.hash == self.calculate_hash()
    }
    
    pub fn genesis() -> Self {
        let genesis_tx = Transaction::coinbase(
            "genesis_address".to_string(),
            0.0,
        );

        let mut block = Block {
            index: 0,
            timestamp: 0,
            transactions: vec![genesis_tx],
            previous_hash: "0".repeat(64),
            nonce: 0,
            hash: String::new(),
        };

        block.hash = block.calculate_hash();
        block
    }
}