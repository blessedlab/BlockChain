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

    pub fn print_merkle_tree(transactions: &[Transaction]) {
        println!("\nMerkle Tree ({} transaction(s))", transactions.len());
        println!("{}", "═".repeat(60));

        if transactions.is_empty() {
            println!("  (no transactions)");
            println!();
            return;
        }

        const H: usize = 8;
        const NW: usize = H + 2;
        const LEAF_GAP: usize = 4;

        let original_count = transactions.len();

        let mut leaves: Vec<String> = transactions.iter().map(|tx| tx.hash()).collect();
        let target_len = original_count.next_power_of_two();
        while leaves.len() < target_len {
            leaves.push(leaves.last().unwrap().clone());
        }

        let mut levels: Vec<Vec<String>> = vec![leaves];
        while levels.last().unwrap().len() > 1 {
            let current = levels.last().unwrap().clone();
            let next = current
                .chunks(2)
                .map(|chunk| {
                    let combined = format!("{}{}", chunk[0], chunk[1]);
                    let mut hasher = Sha256::new();
                    hasher.update(combined.as_bytes());
                    hex::encode(hasher.finalize())
                })
                .collect();
            levels.push(next);
        }

        let n_levels = levels.len();
        let n_leaves = levels[0].len();

        let mut centers: Vec<Vec<usize>> = vec![vec![]; n_levels];
        for j in 0..n_leaves {
            centers[0].push(j * (NW + LEAF_GAP) + NW / 2);
        }
        for d in 1..n_levels {
            for j in 0..levels[d].len() {
                let l = centers[d - 1][2 * j];
                let r = centers[d - 1][2 * j + 1];
                centers[d].push((l + r) / 2);
            }
        }

        let total_w = n_leaves * (NW + LEAF_GAP) + 2;

        for d in (0..n_levels).rev() {
            // Node row
            let mut row = vec![b' '; total_w];
            for (j, hash) in levels[d].iter().enumerate() {
                let start = centers[d][j].saturating_sub(NW / 2);
                let label = format!("[{}]", &hash[..H]);
                for (k, byte) in label.bytes().enumerate() {
                    if start + k < row.len() {
                        row[start + k] = byte;
                    }
                }
            }
            println!("{}", String::from_utf8_lossy(&row).trim_end());

            if d > 0 {
                let mut brow = vec![b' '; total_w];
                for j in 0..levels[d].len() {
                    let pc = centers[d][j];
                    let lc = centers[d - 1][2 * j];
                    let rc = centers[d - 1][2 * j + 1];
                    let slash = (pc + lc) / 2;
                    let backslash = (pc + rc) / 2;
                    if slash < brow.len() {
                        brow[slash] = b'/';
                    }
                    if backslash < brow.len() {
                        brow[backslash] = b'\\';
                    }
                }
                println!("{}", String::from_utf8_lossy(&brow).trim_end());
            }
        }

        println!();
        let mut lrow = vec![b' '; total_w];
        for j in 0..original_count {
            let label = if transactions[j].is_coinbase() {
                "COINBASE".to_string()
            } else {
                format!("TX#{}", j)
            };
            let start = centers[0][j].saturating_sub(label.len() / 2);
            for (k, byte) in label.bytes().enumerate() {
                if start + k < lrow.len() {
                    lrow[start + k] = byte;
                }
            }
        }
        for j in original_count..n_leaves {
            let label = "(dup)";
            let start = centers[0][j].saturating_sub(label.len() / 2);
            for (k, byte) in label.bytes().enumerate() {
                if start + k < lrow.len() {
                    lrow[start + k] = byte;
                }
            }
        }
        println!("{}", String::from_utf8_lossy(&lrow).trim_end());
        println!();
    }

    pub fn merkle_root(transactions: &[Transaction]) -> String {
        if transactions.is_empty() {
            return "0".repeat(64);
        }

        let mut hashes: Vec<String> = transactions
            .iter()
            .map(|tx| tx.hash())
            .collect();

        while hashes.len() > 1 {
            if hashes.len() % 2 != 0 {
                let last = hashes.last().unwrap().clone();
                hashes.push(last);
            }

            let mut next_level = Vec::new();
            for chunk in hashes.chunks(2) {
                let combined = format!("{}{}", chunk[0], chunk[1]);
                let mut hasher = Sha256::new();
                hasher.update(combined.as_bytes());
                next_level.push(hex::encode(hasher.finalize()));
            }

            hashes = next_level;
        }

        hashes.remove(0)
    }
}