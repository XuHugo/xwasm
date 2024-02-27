
use crate::account::AccountAddress;
use crypto::hash::HashValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SignedTransaction {
    #[serde(skip)]
    hash: Option<HashValue>,
    txn: Transaction,
    signature: Signature,
}

impl SignedTransaction{
    pub fn hash(&self) -> HashValue {
        self.hash
            .expect("SignedUserTransaction's id should be Some after init.")
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    chain_id: u64,
    sender: AccountAddress,
    to: AccountAddress,
    nonce: u64,
    payload: Vec<u8>,
    max_gas: u64,
    gas_price: u64,
    expiration_timestamp_secs: u64,
    value: u128,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Signature{
    Ed25519 {
        public_key: u8,//Ed25519PublicKey,
        signature: u8,//Ed25519Signature,
    },
}