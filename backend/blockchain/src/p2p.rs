use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use crate::block::Block;
use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::collections::HashSet;

pub struct P2P {
    pub blockchain: Arc<Mutex<Blockchain>>,
    pub peers: Arc<Mutex<HashSet<String>>>,
}

impl P2P {
    pub fn new(blockchain: Arc<Mutex<Blockchain>>) -> Self {
        P2P {
            blockchain,
            peers: Arc::new(Mutex::new(HashSet::new()))
        }
    }

    pub fn start(&self, address: &str) {
        let listener = TcpListener::bind(address).unwrap();
        println!("Node listening on {}", address);

        // Clone necessary fields for the thread
        let peers = Arc::clone(&self.peers);
        let blockchain = Arc::clone(&self.blockchain);

        // Thread to handle incoming connections
        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let peers_clone = Arc::clone(&peers);
                        let blockchain_clone = Arc::clone(&blockchain);

                        thread::spawn(move || {
                            P2P::handle_client(stream, blockchain_clone, peers_clone);
                        });
                    }
                    Err(e) => eprintln!("Failed to establish connection: {}", e),
                }
            }
        });
    }

    pub fn connect_to_peer(&self, peer_address: &str) {
        match TcpStream::connect(peer_address) {
            Ok(mut stream) => {
                println!("Connected to peer: {}", peer_address);

                // Add peer to known peers
                self.peers.lock().unwrap().insert(peer_address.to_string());

                // Request their blockchain
                stream.write("GET_CHAIN".as_bytes()).unwrap();
                stream.flush().unwrap();

                // Process response
                let mut buffer = [0; 8192];
                match stream.read(&mut buffer) {
                    Ok(_) => {
                        let response = String::from_utf8_lossy(&buffer);
                        match serde_json::from_str::<Vec<Block>>(&response.trim_end_matches(char::from(0))) {
                            Ok(remote_chain) => {
                                let mut local_blockchain = self.blockchain.lock().unwrap();

                                // Simple chain validation and replacement if longer
                                if remote_chain.len() > local_blockchain.chain.len() {
                                    local_blockchain.chain = remote_chain;
                                    println!("Blockchain updated from peer");
                                }
                            },
                            Err(e) => eprintln!("Failed to parse chain: {}", e),
                        }
                    },
                    Err(e) => eprintln!("Failed to receive data: {}", e),
                }
            },
            Err(e) => eprintln!("Failed to connect to peer {}: {}", peer_address, e),
        }
    }

    pub fn broadcast_transaction(&self, transaction: &Transaction) {
        for peer in self.peers.lock().unwrap().iter() {
            if let Ok(mut stream) = TcpStream::connect(peer) {
                let tx_json = serde_json::to_string(&transaction).unwrap();
                let message = format!("NEW_TRANSACTION {}", tx_json);

                stream.write(message.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        }
    }

    pub fn broadcast_block(&self, block: &Block) {
        for peer in self.peers.lock().unwrap().iter() {
            if let Ok(mut stream) = TcpStream::connect(peer) {
                let block_json = serde_json::to_string(&block).unwrap();
                let message = format!("NEW_BLOCK {}", block_json);

                stream.write(message.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        }
    }

    // Helper function to calculate balance for an address
    fn calculate_balance(chain: &Blockchain, address: &str) -> f64 {
        let mut balance = 0.0;

        // Check all blocks for transactions involving this address
        for block in &chain.chain {
            for tx in &block.transactions {
                if tx.sender == address {
                    balance -= tx.amount;
                }
                if tx.receiver == address {
                    balance += tx.amount;
                }
            }
        }

        // Check pending transactions as well
        for tx in &chain.pending_transactions {
            if tx.sender == address {
                balance -= tx.amount;
            }
            if tx.receiver == address {
                balance += tx.amount;
            }
        }

        balance
    }

    fn handle_client(mut stream: TcpStream, blockchain: Arc<Mutex<Blockchain>>, peers: Arc<Mutex<HashSet<String>>>) {
        let mut buffer = [0; 8192];
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                let request = String::from_utf8_lossy(&buffer[..bytes_read]);
                let request_parts: Vec<&str> = request.trim_end_matches(char::from(0)).split_whitespace().collect();

                if request_parts.is_empty() {
                    return;
                }

                let command = request_parts[0];

                // Get peer address for future communication
                if let Ok(peer_addr) = stream.peer_addr() {
                    peers.lock().unwrap().insert(peer_addr.to_string());
                }

                let response = match command {
                    "GET_CHAIN" => {
                        let chain = blockchain.lock().unwrap();
                        serde_json::to_string(&chain.chain).unwrap()
                    },
                    "NEW_BLOCK" => {
                        if request_parts.len() > 1 {
                            let block_json = request_parts[1..].join(" ");
                            match serde_json::from_str::<Block>(&block_json) {
                                Ok(block) => {
                                    let mut chain = blockchain.lock().unwrap();
                                    if block.index == chain.chain.len() as u64 &&
                                        block.previous_hash == chain.chain.last().unwrap().hash {
                                        chain.chain.push(block);
                                        "Block accepted".to_string()
                                    } else {
                                        "Invalid block".to_string()
                                    }
                                },
                                Err(_) => "Invalid block format".to_string(),
                            }
                        } else {
                            "Missing block data".to_string()
                        }
                    },
                    "NEW_TRANSACTION" => {
                        if request_parts.len() > 1 {
                            let tx_json = request_parts[1..].join(" ");
                            match serde_json::from_str::<Transaction>(&tx_json) {
                                Ok(transaction) => {
                                    // Skip balance check for mining rewards
                                    if transaction.sender == "0" {
                                        let mut chain = blockchain.lock().unwrap();
                                        chain.pending_transactions.push(transaction);
                                        "Transaction accepted".to_string()
                                    } else {
                                        // Verify signature
                                        if !transaction.verify() {
                                            "Invalid transaction signature".to_string()
                                        } else {
                                            // Check if sender has enough balance
                                            let chain = blockchain.lock().unwrap();
                                            let sender_balance = Self::calculate_balance(&chain, &transaction.sender);

                                            if sender_balance < transaction.amount {
                                                format!("Insufficient balance: {} < {}", sender_balance, transaction.amount)
                                            } else {
                                                // Add to pending transactions
                                                drop(chain); // Release the lock before acquiring it again
                                                let mut chain = blockchain.lock().unwrap();
                                                chain.pending_transactions.push(transaction);
                                                "Transaction accepted".to_string()
                                            }
                                        }
                                    }
                                },
                                Err(_) => "Invalid transaction format".to_string(),
                            }
                        } else {
                            "Missing transaction data".to_string()
                        }
                    },
                    // In the MINE command handler in p2p.rs
                    "MINE" => {
                        if request_parts.len() > 1 {
                            let miner_address = request_parts[1].to_string();
                            let mut chain = blockchain.lock().unwrap();

                            // Create a reward transaction for the miner
                            let reward_transaction = Transaction {
                                sender: "0".to_string(),  // "0" indicates a mining reward
                                receiver: miner_address.clone(),
                                amount: chain.mining_reward,
                                signature: String::new(), // No signature needed for mining rewards
                                timestamp: chrono::Utc::now().timestamp() as u64,
                                fee: 0.0,
                                nonce: 0,
                            };

                            // Add the reward transaction to pending transactions
                            chain.pending_transactions.push(reward_transaction);

                            // Mine block with all pending transactions (including the reward)
                            let _ = chain.mine_pending_transactions(miner_address);

                            "Block mined successfully".to_string()
                        } else {
                            "Missing miner address".to_string()
                        }
                    },
                    "GET_BALANCE" => {
                        if request_parts.len() > 1 {
                            let address = request_parts[1].to_string();
                            let chain = blockchain.lock().unwrap();
                            let balance = Self::calculate_balance(&chain, &address);
                            balance.to_string()
                        } else {
                            "Missing address parameter".to_string()
                        }
                    },
                    "GET_STATS" => {
                        let chain = blockchain.lock().unwrap();

                        // Calculate basic stats
                        let block_count = chain.chain.len();
                        let transaction_count: usize = chain.chain.iter().map(|block| block.transactions.len()).sum();
                        let pending_tx_count = chain.pending_transactions.len();
                        let difficulty = chain.difficulty;

                        // Calculate average block time if we have at least 2 blocks
                        let avg_block_time = if block_count > 1 {
                            let mut sum_time = 0;
                            for i in 1..block_count {
                                let prev_time = chain.chain[i-1].timestamp1;
                                let curr_time = chain.chain[i].timestamp1;
                                sum_time += curr_time - prev_time;
                            }
                            format!("{:.2}s", (sum_time as f64) / ((block_count - 1) as f64))
                        } else {
                            "N/A".to_string()
                        };

                        format!(
                            "Blocks: {}\nTransactions: {}\nDifficulty: {}\nAverage Block Time: {}\nPending Transactions: {}",
                            block_count, transaction_count, difficulty, avg_block_time, pending_tx_count
                        )
                    },
                    "GET_BLOCK" => {
                        if request_parts.len() > 1 {
                            if let Ok(block_index) = request_parts[1].parse::<usize>() {
                                let chain = blockchain.lock().unwrap();
                                if block_index < chain.chain.len() {
                                    serde_json::to_string(&chain.chain[block_index]).unwrap()
                                } else {
                                    format!("Block index out of range. Max index: {}", chain.chain.len() - 1)
                                }
                            } else {
                                "Invalid block index".to_string()
                            }
                        } else {
                            "Missing block index".to_string()
                        }
                    },
                    "GET_PENDING" => {
                        let chain = blockchain.lock().unwrap();
                        serde_json::to_string(&chain.pending_transactions).unwrap()
                    },
                    _ => "Unknown command".to_string(),
                };

                // Send the response back
                match stream.write(response.as_bytes()) {
                    Ok(_) => {
                        if let Err(e) = stream.flush() {
                            eprintln!("Failed to flush response: {}", e);
                        }
                    },
                    Err(e) => eprintln!("Failed to send response: {}", e),
                }
            },
            Err(e) => eprintln!("Failed to read from stream: {}", e),
        }
    }
}