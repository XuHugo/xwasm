


use std::sync::Arc;
use types::transaction::SignedTransaction;
use crypto::hash::HashValue;
use db::DB;

use anyhow::Result;

use crate::schema_txn::TransactionSchema;

#[derive(Clone, Debug)]
pub struct TransactionStore {
    db: Arc<DB>,
}

impl TransactionStore {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub fn get_txn_by_hash(&self, hash: &HashValue) -> Result<Option<SignedTransaction>> {
        self.db.get::<TransactionSchema>(hash)
    }

    pub fn put_txn(&self, hash: &HashValue, txn: &SignedTransaction) -> Result<()> {
        self.db.put::<TransactionSchema>(hash, txn)
    }
    
}


