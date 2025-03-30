use clap::{Parser, Subcommand, Args};
use colored::Colorize;
use chrono::{DateTime, Local};
use std::io::{self, Write};
use rusqlite::Connection;
use serde_json;
use std::net::TcpStream;
use std::io::{Read, BufReader, BufRead};

use crate::wallet::{WalletManager, WalletError};
use crate::blockchain::{Blockchain, BlockchainStats};
use crate::transaction::Transaction;
use crate::node::Node;

#[derive(Parser)]
#[command(name = "Blockchain CLI")]
#[command(author = "Your Name")]
#[command(version = "1.0")]
#[command(about = "A simple blockchain cryptocurrency CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Wallet management commands
    Wallet {
        #[command(subcommand)]
        command: WalletCommands,
    },

    /// Node management commands
    Node {
        #[command(subcommand)]
        command: NodeCommands,
    },

    /// Transaction commands
    Transaction {
        #[command(subcommand)]
        command: TransactionCommands,
    },

    /// Mining commands
    Mine {
        /// Label of mining reward wallet
        wallet: String,

        /// Node address (e.g., 127.0.0.1:7001)
        node: String,
    },

    /// Blockchain commands
    Blockchain {
        #[command(subcommand)]
        command: BlockchainCommands,
    },
}

#[derive(Subcommand)]
enum WalletCommands {
    /// Create a new wallet
    Create {
        /// Label for the wallet
        label: String,
    },

    /// List all wallets
    List,

    /// Show detailed wallet information
    Info {
        /// Label of the wallet
        label: String,
    },

    /// Check wallet balance
    Balance {
        /// Label of the wallet
        label: String,

        /// Node address (e.g., 127.0.0.1:7001)
        node: String,
    },
}

#[derive(Subcommand)]
enum NodeCommands {
    /// Start a blockchain node
    Start {
        /// Listen address (e.g., 127.0.0.1:7001)
        address: String,

        /// Peer nodes to connect to
        #[arg(num_args = 0..)]
        peers: Vec<String>,
    },

    /// Check node status
    Status {
        /// Node address (e.g., 127.0.0.1:7001)
        address: String,
    },
}

#[derive(Subcommand)]
enum TransactionCommands {
    /// Send tokens to another address
    Send {
        /// Label of sending wallet
        from_wallet: String,

        /// Recipient address
        to_address: String,

        /// Amount to send
        amount: f64,

        /// Node address (e.g., 127.0.0.1:7001)
        node: String,
    },

    /// Show transaction history for a wallet
    History {
        /// Label of wallet
        wallet: String,

        /// Node address (e.g., 127.0.0.1:7001)
        node: String,
    },
}

#[derive(Subcommand)]
enum BlockchainCommands {
    /// Show blockchain information
    Info {
        /// Node address (e.g., 127.0.0.1:7001)
        node: String,
    },

    /// Show block information
    Block {
        /// Block index/height
        block_index: u64,

        /// Node address (e.g., 127.0.0.1:7001)
        node: String,
    },
}

pub struct CLI {
    wallet_manager: WalletManager,
}

impl CLI {
    pub fn new(db_path: &str) -> Result<Self, WalletError> {
        let wallet_manager = WalletManager::new(db_path)?;
        Ok(CLI { wallet_manager })
    }

    pub fn run(&self) {
        let cli = Cli::parse();

        match &cli.command {
            Commands::Wallet { command } => match command {
                WalletCommands::Create { label } => {
                    self.create_wallet(label);
                }
                WalletCommands::List => {
                    self.list_wallets();
                }
                WalletCommands::Info { label } => {
                    self.show_wallet_info(label);
                }
                WalletCommands::Balance { label, node } => {
                    self.check_wallet_balance(label, node);
                }
            },
            Commands::Node { command } => match command {
                NodeCommands::Start { address, peers } => {
                    self.start_node(address, peers);
                }
                NodeCommands::Status { address } => {
                    self.check_node_status(address);
                }
            },
            Commands::Transaction { command } => match command {
                TransactionCommands::Send { from_wallet, to_address, amount, node } => {
                    self.send_transaction(from_wallet, to_address, amount, node);
                }
                TransactionCommands::History { wallet, node } => {
                    self.show_transaction_history(wallet, node);
                }
            },
            Commands::Mine { wallet, node } => {
                self.mine_block(wallet, node);
            }
            Commands::Blockchain { command } => match command {
                BlockchainCommands::Info { node } => {
                    self.show_blockchain_info(node);
                }
                BlockchainCommands::Block { block_index, node } => {
                    self.show_block_info(*block_index, node);
                }
            },
        }
    }

    // Implementation of the various commands:

    fn create_wallet(&self, label: &str) {
        match self.wallet_manager.create_wallet(label) {
            Ok(wallet) => {
                println!("{}", "Wallet created successfully:".green());
                println!("  Label: {}", wallet.label);
                println!("  Address: {}", wallet.address);
                println!("  Created at: {}", wallet.created_at);
            },
            Err(e) => println!("{} {}", "Error:".red(), e),
        }
    }

    fn list_wallets(&self) {
        match self.wallet_manager.list_wallets() {
            Ok(wallets) => {
                if wallets.is_empty() {
                    println!("No wallets found.");
                    return;
                }

                println!("{}", "Wallets:".green());
                for wallet in wallets {
                    println!("  Label: {}", wallet.label);
                    println!("  Address: {}", wallet.address);
                    println!("  Created at: {}", wallet.created_at);
                    println!("  Last used: {}", wallet.last_used);
                    println!("{}", "-".repeat(40));
                }
            },
            Err(e) => println!("{} {}", "Error:".red(), e),
        }
    }

    fn show_wallet_info(&self, label: &str) {
        match self.wallet_manager.get_wallet_by_label(label) {
            Ok(wallet) => {
                println!("{}", "Wallet Information:".green());
                println!("  Label: {}", wallet.label);
                println!("  Address: {}", wallet.address);
                println!("  Created at: {}", wallet.created_at);
                println!("  Last used: {}", wallet.last_used);
            },
            Err(e) => println!("{} {}", "Error:".red(), e),
        }
    }

    fn check_wallet_balance(&self, label: &str, node: &str) {
        // Implementation left as an exercise
        println!("Checking balance for wallet '{}' from node {}", label, node);
    }

    fn start_node(&self, address: &str, peers: &Vec<String>) {
        println!("Starting node on {} with peers: {:?}", address, peers);
        // Implementation left as an exercise
    }

    fn check_node_status(&self, address: &str) {
        println!("Checking status of node at {}", address);
        // Implementation left as an exercise
    }

    fn send_transaction(&self, from: &str, to: &str, amount: &f64, node: &str) {
        println!("Sending {} from '{}' to '{}' via node {}", amount, from, to, node);
        // Implementation left as an exercise
    }

    fn show_transaction_history(&self, wallet: &str, node: &str) {
        println!("Showing transaction history for wallet '{}' from node {}", wallet, node);
        // Implementation left as an exercise
    }

    fn mine_block(&self, wallet: &str, node: &str) {
        println!("Mining block with rewards to wallet '{}' on node {}", wallet, node);
        // Implementation left as an exercise
    }

    fn show_blockchain_info(&self, node: &str) {
        println!("Showing blockchain info from node {}", node);
        // Implementation left as an exercise
    }

    fn show_block_info(&self, block_index: u64, node: &str) {
        println!("Showing info for block {} from node {}", block_index, node);
        // Implementation left as an exercise
    }
}