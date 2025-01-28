use crate::transaction::Transaction;
use serde::{Serialize, Deserialize};
use chrono::Utc;
use ring::digest::{Context, SHA256};
use hex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: String,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        }
    }

    pub fn calculate_hash(&self) -> String {
        let mut context = Context::new(&SHA256);
        context.update(self.index.to_string().as_bytes());
        context.update(self.timestamp.as_bytes());
        context.update(serde_json::to_string(&self.transactions).unwrap().as_bytes());
        context.update(self.previous_hash.as_bytes());
        context.update(self.nonce.to_string().as_bytes());
        let digest = context.finish();
        hex::encode(digest.as_ref())
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let prefix = "0".repeat(difficulty);
        while !self.hash.starts_with(&prefix) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        println!("Block mined: {}", self.hash);
    }
}
