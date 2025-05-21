use crate::block::Block;
use crate::transaction::Transaction;
use crate::coin::Coin;
use std::collections::{HashMap, HashSet};
use thiserror::Error;
use serde::{Serialize, Deserialize};
use std::sync::RwLock;

#[derive(Debug, Error)]
pub enum BlockchainError {
    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),

    #[error("Invalid block: {0}")]
    InvalidBlock(String),

    #[error("Chain integrity error: {0}")]
    ChainIntegrity(String),

    #[error("Double spend detected")]
    DoubleSpend,

    #[error("Mining error: {0}")]
    MiningError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainStats {
    pub blocks: usize,
    pub transactions: usize,
    pub difficulty: usize,
    pub average_block_time: f64,
    pub current_mempool_size: usize,
}

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub pending_transactions: Vec<Transaction>,
    pub mining_reward: f64,
    pub target_timestamp: u64,
    pub coin: Coin,

    // Efficient data structures for quick lookups
    transaction_index: HashMap<String, (u64, usize)>, // txid -> (block_index, tx_index)
    address_balances: RwLock<HashMap<String, f64>>,   // address -> balance
    spent_outputs: HashSet<String>,                   // track spent transaction outputs to prevent double spend
}

impl Blockchain {
    pub fn new(difficulty: usize, mining_reward: f64, coin: Coin) -> Self {
        let mut chain = Vec::new();
        let mut genesis_block = Block::new(0, vec![], "0".to_string());
        genesis_block.hash = genesis_block.calculate_hash();
        chain.push(genesis_block);

        Blockchain {
            chain,
            difficulty,
            pending_transactions: Vec::new(),
            mining_reward,
            target_timestamp: 300,
            coin,
            transaction_index: HashMap::new(),
            address_balances: RwLock::new(HashMap::new()),
            spent_outputs: HashSet::new(),
        }
    }

    pub fn create_transaction(&mut self, transaction: Transaction) -> Result<(), BlockchainError> {
        // Verify signature
        if !transaction.verify() {
            return Err(BlockchainError::InvalidTransaction("Invalid signature".to_string()));
        }

        // Check for double spend (simplified implementation)
        let tx_id = transaction.calculate_id();
        if self.spent_outputs.contains(&tx_id) {
            return Err(BlockchainError::DoubleSpend);
        }

        // Verify sender has sufficient balance (except for mining rewards)
        if transaction.sender != "0" {
            let balances = self.address_balances.read().unwrap();
            let sender_balance = balances.get(&transaction.sender).cloned().unwrap_or(0.0);

            if sender_balance < transaction.amount {
                return Err(BlockchainError::InvalidTransaction(format!(
                    "Insufficient balance: {} < {}", sender_balance, transaction.amount
                )));
            }
        }

        // Add to pending transactions
        self.pending_transactions.push(transaction);
        Ok(())
    }

    pub fn dynamic_difficulty(&mut self) -> usize {
        if self.chain.len() < 2 {
            return self.difficulty;
        }

        let start_time = self.chain[self.chain.len() - 2].timestamp1;
        let end_time = self.chain.last().unwrap().timestamp1;
        let elapsed = end_time - start_time;

        if elapsed < self.target_timestamp {
            self.difficulty = self.difficulty.saturating_add(1);
        } else if elapsed > self.target_timestamp {
            self.difficulty = self.difficulty.saturating_sub(1);
        }

        println!("Adjusted difficulty: {} (block time: {}s)", self.difficulty, elapsed);
        self.difficulty
    }

    pub fn mine_pending_transactions(&mut self, miner_address: String) -> Result<Block, BlockchainError> {
        if self.pending_transactions.is_empty() {
            return Err(BlockchainError::MiningError("No transactions to mine".to_string()));
        }

        let reward_transaction = Transaction {
            sender: "0".to_string(),
            receiver: miner_address,
            amount: self.mining_reward,
            signature: String::new(),
            timestamp: chrono::Utc::now().timestamp() as u64,
            fee: 0.0,
            nonce: 0,
        };

        let mut transactions = self.pending_transactions.clone();
        transactions.push(reward_transaction);

        // Sort transactions by fee (simplified, would be more complex in real implementation)
        transactions.sort_by(|a, b| b.fee.partial_cmp(&a.fee).unwrap());

        let previous_hash = self.chain.last().unwrap().hash.clone();
        let mut block = Block::new(self.chain.len() as u64, transactions, previous_hash);

        self.difficulty = self.dynamic_difficulty();

        // Mine the block
        match block.mine_block(self.difficulty) {
            Ok(_) => {
                // Add mined block to the chain
                self.chain.push(block.clone());

                // Update our indexes
                self.update_indexes(&block);

                // Clear pending transactions
                self.pending_transactions.clear();

                Ok(block)
            },
            Err(e) => Err(BlockchainError::MiningError(format!("Mining failed: {:?}", e))),
        }
    }

    fn update_indexes(&mut self, block: &Block) {
        for (tx_idx, tx) in block.transactions.iter().enumerate() {
            // Update transaction index
            let tx_id = tx.calculate_id();
            self.transaction_index.insert(tx_id.clone(), (block.index, tx_idx));

            // Mark output as spent
            self.spent_outputs.insert(tx_id);

            // Update balances
            let mut balances = self.address_balances.write().unwrap();

            // Deduct from sender (except mining rewards)
            if tx.sender != "0" {
                let sender_balance = balances.entry(tx.sender.clone()).or_insert(0.0);
                *sender_balance -= tx.amount;
            }

            // Add to receiver
            let receiver_balance = balances.entry(tx.receiver.clone()).or_insert(0.0);
            *receiver_balance += tx.amount;
        }
    }

    pub fn is_valid_chain(&self) -> Result<bool, BlockchainError> {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            // Verify hash integrity
            if current.hash != current.calculate_hash() {
                return Err(BlockchainError::ChainIntegrity(
                    format!("Invalid hash at block {}", i)
                ));
            }

            // Verify chain links
            if current.previous_hash != previous.hash {
                return Err(BlockchainError::ChainIntegrity(
                    format!("Broken chain at block {}", i)
                ));
            }

            // Verify all transactions
            for (tx_idx, transaction) in current.transactions.iter().enumerate() {
                if !transaction.verify() && transaction.sender != "0" {
                    return Err(BlockchainError::InvalidTransaction(
                        format!("Invalid signature in block {} transaction {}", i, tx_idx)
                    ));
                }
            }
        }

        Ok(true)
    }

    pub fn get_balance(&self, address: &str) -> f64 {
        let balances = self.address_balances.read().unwrap();
        *balances.get(address).unwrap_or(&0.0)
    }

    pub fn get_transaction(&self, tx_id: &str) -> Option<Transaction> {
        if let Some((block_idx, tx_idx)) = self.transaction_index.get(tx_id) {
            if let Some(block) = self.chain.get(*block_idx as usize) {
                if let Some(tx) = block.transactions.get(*tx_idx) {
                    return Some(tx.clone());
                }
            }
        }
        None
    }

    pub fn get_stats(&self) -> BlockchainStats {
        let total_tx: usize = self.chain.iter().map(|block| block.transactions.len()).sum();

        let avg_block_time = if self.chain.len() > 1 {
            let first_time = self.chain.first().unwrap().timestamp1;
            let last_time = self.chain.last().unwrap().timestamp1;
            let elapsed = last_time - first_time;
            elapsed as f64 / (self.chain.len() as f64 - 1.0)
        } else {
            0.0
        };

        BlockchainStats {
            blocks: self.chain.len(),
            transactions: total_tx,
            difficulty: self.difficulty,
            average_block_time: avg_block_time,
            current_mempool_size: self.pending_transactions.len(),
        }
    }
}