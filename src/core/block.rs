extern crate serde;
extern crate sha2;

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use crate::runtime::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub data: String,
}

impl Block {
    pub fn new(index: u64, timestamp: u128, data: String, previous_hash: String) -> Block {
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
            transactions: vec![],
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let headers = format!("{}{}{}{}", &self.index, &self.timestamp, &self.data, &self.previous_hash);
        let mut hasher = Sha256::new();
        hasher.update(headers);
        let hash = hasher.finalize();
        format!("{:x}", hash)
    }
}