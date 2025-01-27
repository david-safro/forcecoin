use chrono::Utc;
use ring::digest::{Context, SHA256, Digest};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: String,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        let hash = Block::calculate_hash(index, &timestamp, &data, &previous_hash);

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    pub fn calculate_hash(index: u64, timestamp: &str, data: &str, previous_hash: &str) -> String {
        let mut context = Context::new(&SHA256);
        context.update(index.to_string().as_bytes());
        context.update(timestamp.as_bytes());
        context.update(data.as_bytes());
        context.update(previous_hash.as_bytes());
        let digest = context.finish();
        hex::encode(digest.as_ref())
    }
}
