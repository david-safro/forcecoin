use crate::block::Block;
use crate::transaction::Transaction;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub pending_transactions: Vec<Transaction>,
    pub mining_reward: f64,
}

impl Blockchain {
    pub fn new(difficulty: usize, mining_reward: f64) -> Self {
        let mut chain = Vec::new();
        let genesis_block = Block::new(0, vec![], "0".to_string());
        chain.push(genesis_block);

        Blockchain {
            chain,
            difficulty,
            pending_transactions: Vec::new(),
            mining_reward,
        }
    }

    pub fn create_transaction(&mut self, transaction: Transaction) {
        if transaction.verify() {
            self.pending_transactions.push(transaction);
        } else {
            println!("Invalid transaction! Ignored.");
        }
    }

    pub fn mine_pending_transactions(&mut self, miner_address: String) {
        let reward_transaction = Transaction {
            sender: "0".to_string(),
            receiver: miner_address,
            amount: self.mining_reward,
            signature: String::new(),
        };

        self.pending_transactions.push(reward_transaction);

        let transactions = self.pending_transactions.clone();
        let previous_hash = self.chain.last().unwrap().hash.clone();
        let mut block = Block::new(self.chain.len() as u64, transactions, previous_hash);

        block.mine_block(self.difficulty);
        self.chain.push(block);

        self.pending_transactions.clear();
    }

    pub fn is_valid_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.hash != current.calculate_hash() || current.previous_hash != previous.hash {
                return false;
            }

            for transaction in &current.transactions {
                if !transaction.verify() {
                    return false;
                }
            }
        }
        true
    }
}
