mod block;
mod blockchain;
mod transaction;
mod p2p;
mod node;

use blockchain::Blockchain;
use transaction::Transaction;
use ring::signature::{Ed25519KeyPair, KeyPair};
use ring::rand::SystemRandom;
use std::sync::{Arc, Mutex};

fn generate_keys() -> (String, Ed25519KeyPair) {
    let rng = SystemRandom::new();
    let private_key = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    let key_pair = Ed25519KeyPair::from_pkcs8(private_key.as_ref()).unwrap();

    let public_key = hex::encode(key_pair.public_key().as_ref());
    (public_key, key_pair)
}

fn main() {
    let mut blockchain = Blockchain::new(4, 50.0);
    let (test1_pub, test1_keys) = generate_keys();
    let (test2_pub,test2_keys) = generate_keys();

    println!("test 1 key: {}", test1_pub);
    println!("test 2: {}", test2_pub);

    let transaction = Transaction::new(&test1_pub, &test2_pub, 10.0, &test1_keys);

    blockchain.create_transaction(transaction);
    blockchain.mine_pending_transactions(test1_pub.clone());

    println!("Blockchain: {:?}", blockchain.chain);
}
//NOT FULLY DEBUGGED
struct Wallet {
    private_key : String,
    public_key: String,
}
fn wallet() -> Wallet{
    let rng = SystemRandom::new();
    let mut private_key = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    let key_pair = Ed25519KeyPair::from_pkcs8(private_key.as_ref()).unwrap();
    let public_key = hex::encode(key_pair.public_key().as_ref());

    private_key = hex::encode(private_key.as_ref());
    Wallet{private_key, public_key}
}
