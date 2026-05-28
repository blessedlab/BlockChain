use std::collections::HashMap;

use crate::block::{Block, MINING_REWARD};
use crate::mining::{meets_difficulty, mine_block};
use crate::transaction::Transaction;
use crate::wallet::Wallet;

pub struct Blockchain {

    ///chain of blocks:
    pub chain: Vec<Block>,

    ///pool of transactions
    pub mempool: Vec<Transaction>,

    ///address balances
    pub balances: HashMap<String, f64>,

    ///public key registry
    pub public_keys: HashMap<String, String>,
}

impl Blockchain {
    ///init new blockchain with genesis block
    pub fn new() -> Self {
        let genesis = Block::genesis();

        let mut bc = Blockchain {
            chain: vec![genesis],
            mempool: Vec::new(),
            balances: HashMap::new(),
            public_keys: HashMap::new(),
        };

        ///gifting coins to the blockchain creator
        bc.balances.insert("genesis_address".to_string(), 1_000_000.0);

        bc
    }

    pub fn last_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    ///register a public key
    pub fn register_public_key(&mut self, address: String, public_key: String) {
        self.public_keys.insert(address, public_key);
    }

    ///get address balance
    pub fn get_balance(&self, address: &str) -> f64 {
        *self.balances.get(address).unwrap_or(&0.0)
    }

    ///add transactions to mempool
    pub fn add_transaction(&mut self, tx: Transaction) -> Result<(), String> {
        if tx.is_coinbase() {
            return Err("Cannot manually add coinbase transactions".to_string());
        }

        if let Some(pub_key) = self.public_keys.get(&tx.sender) {
            if !Wallet::verify_transaction(tx.clone(), pub_key) {
                return Err("Invalid transaction signature".to_string());
            }
        } else {
            return Err(format!("Public key for {} not found", tx.sender));
        }

        /// Checking the balance
        let balance = self.get_balance(&tx.sender);
        if balance < tx.amount {
            return Err(format!(
                "Insufficient funds: balance {:.2}, need {:.2}",
                balance, tx.amount
            ));
        }

        if tx.amount <= 0.0 {
            return Err("Transaction amount should be positive".to_string());
        }

        *self.balances.entry(tx.sender.clone()).or_insert(0.0) -= tx.amount;

        self.mempool.push(tx);
        println!("Transaction added to mempool");
        Ok(())
    }

    pub fn mine_pending_transactions(&mut self, miner_address: String) {
        ///creating coinbase transaction(miner award)
        let coinbase = Transaction::coinbase(miner_address.clone(), MINING_REWARD);

        let mut transactions = vec![coinbase];
        transactions.extend(self.mempool.drain(..));

        let new_block = Block::new(
            self.chain.len() as u64,
            transactions,
            self.last_block().hash.clone(),
        );

        let mined_block = mine_block(new_block);


        for tx in &mined_block.transactions {
            if tx.is_coinbase() {
                *self.balances.entry(tx.reciever.clone()).or_insert(0.0) += tx.amount;
            } else {
                *self.balances.entry(tx.reciever.clone()).or_insert(0.0) += tx.amount;
            }
        }

        self.chain.push(mined_block);

        println!("Miner {} got the award: {} RTC", miner_address, MINING_REWARD);
    }

    ///key function which checks if hashes in blocks are valid
    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if !current.has_valid_hash() {
                println!("Block #{}: invalid hash", i);
                return false;
            }

            if current.previous_hash != previous.hash {
                println!("Block #{}: the connection with the previous block has lost", i);
                return false;
            }

            if !meets_difficulty(&current.hash) {
                println!("Block #{}: did not pass Proof of Work", i);
                return false;
            }
        }

        println!("Target valid ({} blocks)", self.chain.len());
        true
    }

    pub fn print_chain(&self) {
        println!("\nBlockchain ({} blocks):", self.chain.len());
        println!("{}", "─".repeat(80));

        for block in &self.chain {
            println!(
                "Block #{} | Hash: {}... | Transactions: {} | Nonce: {}",
                block.index,
                &block.hash[..16],
                block.transactions.len(),
                block.nonce
            );

            for tx in &block.transactions {
                if tx.is_coinbase() {
                    println!(
                        "COINBASE → {} : {:.2} RTC",
                        &tx.reciever[..8],
                        tx.amount
                    );
                } else {
                    println!(
                        "money {}... → {}... : {:.2} RTC",
                        &tx.sender[..8],
                        &tx.reciever[..8],
                        tx.amount
                    );
                }
            }
        }
        println!("{}", "─".repeat(80));
    }
}