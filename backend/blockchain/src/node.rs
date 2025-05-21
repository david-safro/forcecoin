use crate::blockchain::Blockchain;
use crate::p2p::P2P;
use std::sync::{Arc, Mutex};
use crate::coin::Coin;

pub struct Node {
    pub blockchain: Arc<Mutex<Blockchain>>,
    pub p2p: P2P,
}

impl Node {
    pub fn new(difficulty: usize, mining_reward: f64) -> Self {
        let blockchain = Arc::new(Mutex::new(Blockchain::new(difficulty, mining_reward, Coin::new())));
        let p2p = P2P::new(Arc::clone(&blockchain));

        Node {
            blockchain,
            p2p
        }
    }

    pub fn start(&self, address: &str) {
        self.p2p.start(address);
        println!("Node started on {}", address);
    }

    pub fn connect_to_peer(&self, peer_address: &str) {
        self.p2p.connect_to_peer(peer_address);
    }
}