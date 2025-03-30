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

    fn handle_client(mut stream: TcpStream, blockchain: Arc<Mutex<Blockchain>>, peers: Arc<Mutex<HashSet<String>>>) {
        let mut buffer = [0; 8192];
        stream.read(&mut buffer).unwrap();

        let request = String::from_utf8_lossy(&buffer);
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
                            if transaction.verify() {
                                let mut chain = blockchain.lock().unwrap();
                                chain.pending_transactions.push(transaction);
                                "Transaction accepted".to_string()
                            } else {
                                "Invalid transaction".to_string()
                            }
                        },
                        Err(_) => "Invalid transaction format".to_string(),
                    }
                } else {
                    "Missing transaction data".to_string()
                }
            },
            "MINE" => {
                if request_parts.len() > 1 {
                    let miner_address = request_parts[1].to_string();
                    let mut chain = blockchain.lock().unwrap();
                    chain.mine_pending_transactions(miner_address);

                    // Don't broadcast here as it would require self reference
                    // In a complete implementation, we'd restructure the code
                    "Block mined successfully".to_string()
                } else {
                    "Missing miner address".to_string()
                }
            },
            _ => "Unknown command".to_string(),
        };

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}