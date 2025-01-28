use serde::{Serialize, Deserialize};
use ring::signature::{KeyPair, Ed25519KeyPair, Signature, UnparsedPublicKey};
use hex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub signature: String,
}

impl Transaction {
    pub fn new(sender: &str, receiver: &str, amount: f64, private_key: &Ed25519KeyPair) -> Self {
        let data = format!("{}{}{}", sender, receiver, amount);
        let signature = private_key.sign(data.as_bytes());
        Self {
            sender: sender.to_string(),
            receiver: receiver.to_string(),
            amount,
            signature: hex::encode(signature.as_ref()),
        }
    }

    pub fn verify(&self) -> bool {
        let data = format!("{}{}{}", self.sender, self.receiver, self.amount);
        let public_key_bytes = hex::decode(&self.sender).unwrap();
        let public_key = UnparsedPublicKey::new(&ring::signature::ED25519, &public_key_bytes);

        public_key.verify(data.as_bytes(), &hex::decode(&self.signature).unwrap()).is_ok()
    }
}
