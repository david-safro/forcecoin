use std::collections::HashMap;
use crate::transaction::Transaction;
use ring::signature::{Ed25519KeyPair, KeyPair};
use ring::rand::SystemRandom;

pub struct Coin {
    pub name: String,
    pub ticker: String,
    pub amount: u64,
    pub division: f64, //one unit = amount * division
    pub balances: HashMap<User, f64>,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct User {
    pub name: String,
    pub wallet: Wallet,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Wallet {
    pub private_key: String,
    pub public_key: String,
}

impl Coin {
    pub fn new() -> Self {
        let total = 1000000 * 10u64.pow(8);
        Coin {
            name: "ForceCoin".to_string(),
            ticker: "FCN".to_string(),
            amount: total,
            division: 8.0,
            balances: HashMap::new(),
        }
    }

    pub fn get_balance(&self, address: &User) -> f64 {
        *self.balances.get(address).unwrap_or(&0.0)
    }

    pub fn transfer(&mut self, from: &User, to: &User, amount: f64) -> bool {
        let from_balance = self.get_balance(from);

        if from_balance >= amount {
            let keypair = Ed25519KeyPair::from_pkcs8(hex::decode(&from.wallet.private_key).unwrap().as_ref())
                .expect("Failed to parse private key");

            let _transaction = Transaction::new(&from.name, &to.name, amount, &keypair);

            // Update balances
            self.balances.insert(to.clone(), self.get_balance(to) + amount);
            self.balances.insert(from.clone(), from_balance - amount);
            return true;
        } else {
            println!("Insufficient funds");
            return false;
        }
    }

    pub fn new_user(&mut self, name: &str) -> User {
        let wallet = self.generate_wallet();
        let user = User { name: name.to_string(), wallet };
        self.balances.insert(user.clone(), 100.0);
        user
    }

    fn generate_wallet(&self) -> Wallet {
        let rng = SystemRandom::new();
        let private_key_encoded = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let key_pair = Ed25519KeyPair::from_pkcs8(private_key_encoded.as_ref()).unwrap();
        let public_key = hex::encode(key_pair.public_key().as_ref());

        let private_key = hex::encode(private_key_encoded.as_ref());
        Wallet { private_key, public_key }
    }
}