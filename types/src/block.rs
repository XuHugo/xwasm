
use crate::transaction::SignedTransaction;
use crate::account::AccountAddress;

use crypto::hash::HashValue;
use serde::{Deserialize, Serialize};


pub type BlockNumber = u64;
pub const GENESIS_BLOCK_NUMBER: BlockNumber = 0;

#[derive(Debug, PartialEq, Serialize, Deserialize,)]
pub struct Block {
    pub header: BlockHeader,
    pub body: BlockBody,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockHeader {

    id: Option<HashValue>,
    parent_hash: HashValue,
    timestamp: u64,
    number: BlockNumber,
    author: AccountAddress,
    txn_accumulator_root: HashValue,
    block_accumulator_root: HashValue,
    state_root: HashValue,
    gas_used: u64,
    body_hash: HashValue,
    chain_id: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockBody {
    pub transactions: Vec<SignedTransaction>,
}


