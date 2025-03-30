use crate::transaction::Transaction;
use serde::{Serialize, Deserialize};
use chrono::Utc;
use ring::digest::{Context, SHA256};
use hex;
use thiserror::Error;
use std::time::{Duration, Instant};
use rayon::prelude::*;
#[derive(Debug, Error)]
pub enum BlockError {
    #[error("Mining timeout")]
    MiningTimeout,

    #[error("Mining aborted")]
    MiningAborted,

    #[error("Invalid difficulty: {0}")]
    InvalidDifficulty(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: String,
    pub timestamp1: u64,
    pub previous_hash: String,
    pub transactions: Vec<Transaction>,
    pub hash: String,
    pub nonce: u64,
    pub difficulty: usize,
    pub merkle_root: String,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        let timestamp1 = Utc::now().timestamp() as u64;

        // Calculate merkle root from transactions
        let merkle_root = Self::calculate_merkle_root(&transactions);

        Block {
            index,
            timestamp,
            timestamp1,
            previous_hash,
            transactions,
            hash: String::new(),
            nonce: 0,
            difficulty: 0, // Will be set during mining
            merkle_root,
        }
    }

    pub fn calculate_merkle_root(transactions: &[Transaction]) -> String {
        if transactions.is_empty() {
            return "0".repeat(64);
        }

        // Generate hashes of all transactions
        let mut hashes: Vec<String> = transactions
            .iter()
            .map(|tx| tx.calculate_id())
            .collect();

        // Combine hashes until only one remains (the merkle root)
        while hashes.len() > 1 {
            let mut next_level = Vec::new();

            for chunk in hashes.chunks(2) {
                let mut context = Context::new(&SHA256);
                context.update(chunk[0].as_bytes());

                // If odd number of elements, duplicate the last one
                if chunk.len() > 1 {
                    context.update(chunk[1].as_bytes());
                } else {
                    context.update(chunk[0].as_bytes());
                }

                let digest = context.finish();
                next_level.push(hex::encode(digest.as_ref()));
            }

            hashes = next_level;
        }

        hashes[0].clone()
    }

    pub fn calculate_hash(&self) -> String {
        let mut context = Context::new(&SHA256);

        context.update(self.index.to_string().as_bytes());
        context.update(self.timestamp.as_bytes());
        context.update(self.timestamp1.to_string().as_bytes());
        context.update(self.previous_hash.as_bytes());
        context.update(self.merkle_root.as_bytes());
        context.update(self.nonce.to_string().as_bytes());
        context.update(self.difficulty.to_string().as_bytes());

        let digest = context.finish();
        hex::encode(digest.as_ref())
    }

    pub fn mine_block(&mut self, difficulty: usize) -> Result<(), BlockError> {
        if difficulty == 0 {
            return Err(BlockError::InvalidDifficulty("Difficulty cannot be zero".to_string()));
        }

        self.difficulty = difficulty;
        let prefix = "0".repeat(difficulty);
        let start_time = std::time::Instant::now();
        let timeout = std::time::Duration::from_secs(300); // 5 minute timeout

        // Sequential mining implementation (remove parallel code for now)
        let mut nonce = 0u64;

        loop {
            if start_time.elapsed() > timeout {
                return Err(BlockError::MiningTimeout);
            }

            self.nonce = nonce;
            let hash = self.calculate_hash();

            if hash.starts_with(&prefix) {
                self.hash = hash;
                println!("Block {} mined with nonce {}: {}",
                         self.index, self.nonce, self.hash);
                return Ok(());
            }

            nonce += 1;
        }
    }

    pub fn is_valid(&self, difficulty: usize) -> bool {
        // Check that the hash starts with the correct number of zeros
        let prefix = "0".repeat(difficulty);
        if !self.hash.starts_with(&prefix) {
            return false;
        }

        // Verify that the hash is correct
        if self.hash != self.calculate_hash() {
            return false;
        }

        // Verify the merkle root
        let calculated_merkle = Self::calculate_merkle_root(&self.transactions);
        if calculated_merkle != self.merkle_root {
            return false;
        }

        true
    }
}