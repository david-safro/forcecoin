mod block;
mod blockchain;
mod transaction;
mod p2p;
mod node;
mod coin;
mod wallet;

use blockchain::Blockchain;
use transaction::Transaction;
use node::Node;
use wallet::{WalletManager, WalletError};
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::thread;  // Added explicit import for thread
use std::time::Duration;
use std::sync::{Arc, Mutex};

const DB_PATH: &str = "blockchain.db";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let command = &args[1];

    // Initialize wallet manager
    let wallet_manager = match WalletManager::new(DB_PATH) {
        Ok(manager) => manager,
        Err(e) => {
            println!("Failed to initialize wallet manager: {:?}", e);
            return;
        }
    };

    match command.as_str() {
        "start-node" => {
            if args.len() < 3 {
                println!("Usage: {} start-node <listen_address>", args[0]);
                return;
            }

            let listen_address = &args[2];
            let node = Node::new(2, 50.0);
            node.start(listen_address);

            // If peers are provided, connect to them
            if args.len() >= 4 {
                for i in 3..args.len() {
                    let peer_address = &args[i];
                    node.connect_to_peer(peer_address);
                }
            }

            println!("Node started on {}. Press Ctrl+C to exit.", listen_address);

            // Keep the main thread alive
            loop {
                thread::sleep(Duration::from_secs(1));
            }
        },
        "create-wallet" => {
            if args.len() < 3 {
                println!("Usage: {} create-wallet <label>", args[0]);
                return;
            }

            let label = &args[2];

            match wallet_manager.create_wallet(label) {
                Ok(wallet) => {
                    println!("Wallet created successfully:");
                    println!("  Label: {}", wallet.label);
                    println!("  Address: {}", wallet.address);
                    println!("  Created at: {}", wallet.created_at);
                },
                Err(e) => println!("Failed to create wallet: {:?}", e),
            }
        },
        "list-wallets" => {
            match wallet_manager.list_wallets() {
                Ok(wallets) => {
                    if wallets.is_empty() {
                        println!("No wallets found.");
                        return;
                    }

                    println!("Wallets:");
                    for wallet in wallets {
                        println!("  Label: {}", wallet.label);
                        println!("  Address: {}", wallet.address);
                        println!("  Created at: {}", wallet.created_at);
                        println!("  Last used: {}", wallet.last_used);
                        println!("----------------------------------------");
                    }
                },
                Err(e) => println!("Failed to list wallets: {:?}", e),
            }
        },
        "send" => {
            if args.len() < 6 {
                println!("Usage: {} send <wallet_label> <recipient_address> <amount> <node_address>", args[0]);
                return;
            }

            let wallet_label = &args[2];
            let recipient = &args[3];
            let amount = match args[4].parse::<f64>() {
                Ok(value) => value,
                Err(_) => {
                    println!("Invalid amount: must be a valid number");
                    return;
                }
            };
            let node_address = &args[5];

            if amount <= 0.0 {
                println!("Amount must be greater than 0");
                return;
            }

            // Get the wallet by label
            let wallet = match wallet_manager.get_wallet_by_label(wallet_label) {
                Ok(wallet) => wallet,
                Err(e) => {
                    println!("Failed to load wallet with label '{}': {:?}", wallet_label, e);
                    return;
                }
            };

            let key_pair = match wallet.get_keypair() {
                Ok(kp) => kp,
                Err(e) => {
                    println!("Failed to get key pair from wallet: {:?}", e);
                    return;
                }
            };

            // Create a transaction
            let transaction = Transaction::new(&wallet.address, recipient, amount, &key_pair);

            // Connect to node and send transaction
            if let Ok(mut stream) = std::net::TcpStream::connect(node_address) {
                use std::io::{Read, Write};

                let tx_json = match serde_json::to_string(&transaction) {
                    Ok(json) => json,
                    Err(e) => {
                        println!("Failed to serialize transaction: {:?}", e);
                        return;
                    }
                };
                let message = format!("NEW_TRANSACTION {}", tx_json);

                if let Err(e) = stream.write(message.as_bytes()) {
                    println!("Failed to send transaction: {:?}", e);
                    return;
                }

                if let Err(e) = stream.flush() {
                    println!("Failed to flush stream: {:?}", e);
                    return;
                }

                // Read response
                let mut buffer = [0; 1024];
                match stream.read(&mut buffer) {
                    Ok(bytes_read) => {
                        let response = String::from_utf8_lossy(&buffer[..bytes_read]);
                        println!("Response: {}", response);
                    },
                    Err(e) => println!("Failed to read response: {:?}", e),
                }
            } else {
                println!("Failed to connect to node at {}", node_address);
            }
        },
        "mine" => {
            if args.len() < 4 {
                println!("Usage: {} mine <wallet_label> <node_address>", args[0]);
                return;
            }

            let wallet_label = &args[2];
            let node_address = &args[3];

            // Get the wallet by label
            let wallet = match wallet_manager.get_wallet_by_label(wallet_label) {
                Ok(wallet) => wallet,
                Err(e) => {
                    println!("Failed to load wallet with label '{}': {:?}", wallet_label, e);
                    return;
                }
            };

            // Connect to node and request mining
            if let Ok(mut stream) = std::net::TcpStream::connect(node_address) {
                use std::io::{Read, Write};

                let message = format!("MINE {}", wallet.address);

                if let Err(e) = stream.write(message.as_bytes()) {
                    println!("Failed to send mining request: {:?}", e);
                    return;
                }

                if let Err(e) = stream.flush() {
                    println!("Failed to flush stream: {:?}", e);
                    return;
                }

                // Read response
                let mut buffer = [0; 1024];
                match stream.read(&mut buffer) {
                    Ok(bytes_read) => {
                        let response = String::from_utf8_lossy(&buffer[..bytes_read]);
                        println!("Response: {}", response);
                    },
                    Err(e) => println!("Failed to read response: {:?}", e),
                }
            } else {
                println!("Failed to connect to node at {}", node_address);
            }
        },
        "get-balance" => {
            if args.len() < 4 {
                println!("Usage: {} get-balance <wallet_label> <node_address>", args[0]);
                return;
            }

            let wallet_label = &args[2];
            let node_address = &args[3];

            // Get the wallet by label
            let wallet = match wallet_manager.get_wallet_by_label(wallet_label) {
                Ok(wallet) => wallet,
                Err(e) => {
                    println!("Failed to load wallet with label '{}': {:?}", wallet_label, e);
                    return;
                }
            };

            // Connect to node and request balance
            if let Ok(mut stream) = std::net::TcpStream::connect(node_address) {
                use std::io::{Read, Write};

                let message = format!("GET_BALANCE {}", wallet.address);

                if let Err(e) = stream.write(message.as_bytes()) {
                    println!("Failed to send balance request: {:?}", e);
                    return;
                }

                if let Err(e) = stream.flush() {
                    println!("Failed to flush stream: {:?}", e);
                    return;
                }

                // Read response
                let mut buffer = [0; 1024];
                match stream.read(&mut buffer) {
                    Ok(bytes_read) => {
                        let response = String::from_utf8_lossy(&buffer[..bytes_read]);
                        println!("Balance for wallet '{}': {} FCN", wallet_label, response);
                    },
                    Err(e) => println!("Failed to read response: {:?}", e),
                }
            } else {
                println!("Failed to connect to node at {}", node_address);
            }
        },
        "blockchain-stats" => {
            if args.len() < 3 {
                println!("Usage: {} blockchain-stats <node_address>", args[0]);
                return;
            }

            let node_address = &args[2];

            // Connect to node and request blockchain stats
            if let Ok(mut stream) = std::net::TcpStream::connect(node_address) {
                use std::io::{Read, Write};

                let message = "GET_STATS";

                if let Err(e) = stream.write(message.as_bytes()) {
                    println!("Failed to send stats request: {:?}", e);
                    return;
                }

                if let Err(e) = stream.flush() {
                    println!("Failed to flush stream: {:?}", e);
                    return;
                }

                // Read response
                let mut buffer = [0; 2048];
                match stream.read(&mut buffer) {
                    Ok(bytes_read) => {
                        let response = String::from_utf8_lossy(&buffer[..bytes_read]);
                        println!("Blockchain Statistics:\n{}", response);
                    },
                    Err(e) => println!("Failed to read response: {:?}", e),
                }
            } else {
                println!("Failed to connect to node at {}", node_address);
            }
        },
        "get-block" => {
            if args.len() < 4 {
                println!("Usage: {} get-block <block_index> <node_address>", args[0]);
                return;
            }

            let block_index = match args[2].parse::<usize>() {
                Ok(idx) => idx,
                Err(_) => {
                    println!("Invalid block index: must be a number");
                    return;
                }
            };
            let node_address = &args[3];

            // Connect to node and request block info
            if let Ok(mut stream) = std::net::TcpStream::connect(node_address) {
                use std::io::{Read, Write};

                let message = format!("GET_BLOCK {}", block_index);

                if let Err(e) = stream.write(message.as_bytes()) {
                    println!("Failed to send block request: {:?}", e);
                    return;
                }

                if let Err(e) = stream.flush() {
                    println!("Failed to flush stream: {:?}", e);
                    return;
                }

                // Read response
                let mut buffer = [0; 8192];
                match stream.read(&mut buffer) {
                    Ok(bytes_read) => {
                        let response = String::from_utf8_lossy(&buffer[..bytes_read]);
                        println!("Block #{}:\n{}", block_index, response);
                    },
                    Err(e) => println!("Failed to read response: {:?}", e),
                }
            } else {
                println!("Failed to connect to node at {}", node_address);
            }
        },
        "get-pending" => {
            if args.len() < 3 {
                println!("Usage: {} get-pending <node_address>", args[0]);
                return;
            }

            let node_address = &args[2];

            // Connect to node and request pending transactions
            if let Ok(mut stream) = std::net::TcpStream::connect(node_address) {
                use std::io::{Read, Write};

                let message = "GET_PENDING";

                if let Err(e) = stream.write(message.as_bytes()) {
                    println!("Failed to send pending transactions request: {:?}", e);
                    return;
                }

                if let Err(e) = stream.flush() {
                    println!("Failed to flush stream: {:?}", e);
                    return;
                }

                // Read response
                let mut buffer = [0; 8192];
                match stream.read(&mut buffer) {
                    Ok(bytes_read) => {
                        let response = String::from_utf8_lossy(&buffer[..bytes_read]);
                        println!("Pending Transactions:\n{}", response);
                    },
                    Err(e) => println!("Failed to read response: {:?}", e),
                }
            } else {
                println!("Failed to connect to node at {}", node_address);
            }
        },
        _ => {
            print_usage();
        }
    }
}

fn print_usage() {
    println!("ForceCoin - A Simple Blockchain Cryptocurrency");
    println!("----------------------------------------------");
    println!("Usage:");
    println!("  Node Management:");
    println!("    start-node <listen_address> [peer1] [peer2] ...  - Start a blockchain node");
    println!("  Wallet Management:");
    println!("    create-wallet <label>                           - Create a new wallet");
    println!("    list-wallets                                    - List all wallets");
    println!("    get-balance <wallet_label> <node_address>       - Get wallet balance");
    println!("  Transaction Operations:");
    println!("    send <wallet_label> <recipient> <amount> <node> - Send coins to an address");
    println!("    mine <wallet_label> <node_address>              - Mine pending transactions");
    println!("  Blockchain Information:");
    println!("    blockchain-stats <node_address>                 - Show blockchain statistics");
    println!("    get-block <block_index> <node_address>          - Show details of a block");
    println!("    get-pending <node_address>                      - Show pending transactions");
}