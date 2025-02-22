use std::collections::HashMap;
use crate::Transaction;
use ring::signature::{Ed25519KeyPair, KeyPair};
use ring::rand::SystemRandom;
struct Coin {
    name: String,
    ticker : String,
    amount: u64,
    division: f64, //one unit = amount * division
    balances : HashMap<User, f64>,
}
#[derive(Hash,Eq,PartialEq,Debug, Clone)]
struct User{
    name : String,
    wallet : Wallet,
}
impl Coin {
    fn new() -> Self {
        let total = 1000000 * 10u64.pow(8);
        Coin {
            name: "ForceCoin".to_string(),
            ticker: "FCN".to_string(),
            amount: total,
            division: 8.0,
            balances: HashMap::new(),
        }
    }

    fn get_balance(&self, address: User) -> f64 {
        *self.balances.get(&address).unwrap()
    }
    fn transfer(&mut self, from: &User, to: &User, amount: &f64) -> bool {
        if self.balances.get(from) >= Option::from(amount) {
            Transaction::new(&from.name, &to.name, *amount, &Ed25519KeyPair::from_pkcs8(hex::decode(&from.wallet.private_key).unwrap().as_ref()).unwrap());
            self.balances.insert(to.clone(), self.get_balance(to.clone()) + amount);
            self.balances.insert(from.clone(), self.get_balance(from.clone()) - amount);
            return true;
        }
        else{
            println!("Insufficient funds");
            return false;
        }
    }
    fn new_user(&mut self, name: &str){
        let wallet = wallet();
        let user = User{name: name.to_string(), wallet};
        self.balances.insert(user.clone(), 100.0);
    }
}
#[derive(Hash,Eq,PartialEq,Debug, Clone)]
struct Wallet {
    private_key : String,
    public_key: String,
}
fn wallet() -> Wallet{
    let rng = SystemRandom::new();
    let private_key_encoded = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    let key_pair = Ed25519KeyPair::from_pkcs8(private_key_encoded.as_ref()).unwrap();
    let public_key = hex::encode(key_pair.public_key().as_ref());

    let private_key = hex::encode(private_key_encoded.as_ref());
    return Wallet{private_key, public_key}
}