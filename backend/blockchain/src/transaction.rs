use serde::{Serialize, Deserialize};
use ring::signature::{Ed25519KeyPair, UnparsedPublicKey};
use ring::digest::{Context, SHA256};
use thiserror::Error;
use hex;
use chrono::Utc;

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum TransactionError {
    #[error("Invalid signature")]
    InvalidSignature,

    #[error("Invalid address format")]
    InvalidAddressFormat,

    #[error("Invalid amount: {0}")]
    InvalidAmount(String),

    #[error("Crypto error: {0}")]
    CryptoError(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub signature: String,
    #[serde(default = "default_timestamp")]
    pub timestamp: u64,
    #[serde(default)]
    pub fee: f64,
    #[serde(default)]
    pub nonce: u64,
}

fn default_timestamp() -> u64 {
    Utc::now().timestamp() as u64
}

impl Transaction {
    pub fn new(sender: &str, receiver: &str, amount: f64, private_key: &Ed25519KeyPair) -> Self {
        let timestamp = Utc::now().timestamp() as u64;
        // Use a simple nonce for demonstration
        let nonce = rand::random::<u64>();

        // Calculate a reasonable fee (simplified)
        let fee = (amount * 0.001).max(0.001);

        let data = format!("{}{}{}{}{}{}", sender, receiver, amount, timestamp, fee, nonce);

        let signature = hex::encode(private_key.sign(data.as_bytes()).as_ref());

        Self {
            sender: sender.to_string(),
            receiver: receiver.to_string(),
            amount,
            signature,
            timestamp,
            fee,
            nonce,
        }
    }

    pub fn verify(&self) -> bool {
        // Skip verification for mining rewards
        if self.sender == "0" {
            return true;
        }

        let data = format!("{}{}{}{}{}{}",
                           self.sender, self.receiver, self.amount, self.timestamp, self.fee, self.nonce);

        // Try to decode the sender as public key
        match hex::decode(&self.sender) {
            Ok(public_key_bytes) => {
                let public_key = UnparsedPublicKey::new(&ring::signature::ED25519, &public_key_bytes);
                match hex::decode(&self.signature) {
                    Ok(signature_bytes) => {
                        public_key.verify(data.as_bytes(), &signature_bytes).is_ok()
                    },
                    Err(_) => false
                }
            },
            Err(_) => false
        }
    }

    pub fn calculate_id(&self) -> String {
        let mut context = Context::new(&SHA256);

        context.update(self.sender.as_bytes());
        context.update(self.receiver.as_bytes());
        context.update(self.amount.to_string().as_bytes());
        context.update(self.timestamp.to_string().as_bytes());
        context.update(self.fee.to_string().as_bytes());
        context.update(self.nonce.to_string().as_bytes());
        context.update(self.signature.as_bytes());

        let digest = context.finish();
        hex::encode(digest.as_ref())
    }
}