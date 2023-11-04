extern crate serde;

use std::fmt::{Debug};
use std::clone::Clone;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TransactionData {
    MintNFT { metadata: String },
    TransferNFT { nft_id: String, recipient_address: String },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f32,
    pub data: Option<TransactionData>,
}
