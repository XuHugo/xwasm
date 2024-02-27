

use crate::{define_schema, schema::{ValueCodec, TRANSACTION_BY_HASH_CF_NAME}};
use types::transaction::SignedTransaction;
use crypto::hash::HashValue;

use anyhow::Result;

define_schema!(
    TransactionSchema, 
    HashValue, 
    SignedTransaction, 
    TRANSACTION_BY_HASH_CF_NAME
);


impl ValueCodec<TransactionSchema> for SignedTransaction {
    fn encode_value(&self) -> Result<Vec<u8>> {
        todo!()
    }

    fn decode_value(data: &[u8]) -> Result<Self> {
        todo!()
    }
}
