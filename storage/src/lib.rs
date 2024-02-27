mod db;
#[macro_use]
mod schema;
mod schema_block;
mod schema_txn;
mod block_store;
mod tranction_store;

extern crate anyhow;
extern crate rocksdb;
extern crate types;
extern crate crypto;

use std::sync::Arc;
use block_store::BlockStore;
use crypto::hash::HashValue;
use db::DB;
use tranction_store::TransactionStore;
use types::{block::{BlockNumber, Block, BlockBody, BlockHeader}, transaction::SignedTransaction};

use anyhow::Result;

pub const LEDGER_DB_NAME: &str = "ledger_db";
pub const STATE_MERKLE_DB_NAME: &str = "state_merkle_db";
pub type ColumnFamilyName = &'static str;

mod db_options; 

#[derive(Debug)]
pub struct Storage {
    ledger_db: Arc<DB>,
    //state_merkle_db: Arc<DB>,
    //event_store: Arc<EventStore>,
    block_store: Arc<BlockStore>,
    //state_store: Arc<StateStore>,
    transaction_store: Arc<TransactionStore>,
    //ledger_pruner: LedgerPrunerManager,
    //_rocksdb_property_reporter: RocksdbPropertyReporter,
    //ledger_commit_lock: std::sync::Mutex<()>,
    //indexer: Option<Indexer>,
}

impl Storage{
    pub fn new(ledger_db:DB) -> Self {
        let arc_ledger_db = Arc::new(ledger_db);
        Self {
            ledger_db: Arc::clone(&arc_ledger_db),
            block_store: Arc::new(BlockStore::new(Arc::clone(&arc_ledger_db))),
            transaction_store: Arc::new(TransactionStore::new(Arc::clone(&arc_ledger_db))),
        }
    }

    pub fn get_block_by_number(&self, number: BlockNumber) -> Result<Option<Block>> {
        let header = match self.block_store.get_header_by_number(number){
            Ok(Some(header)) => header,
            Ok(None) => return Ok(None),
            Err(e) => return Err(e),
        };

        let body = match self.get_body_by_number(number){
            Ok(Some(txns)) =>txns,
            Ok(None) => return Ok(None),
            Err(e) => return Err(e),
        };

        Ok(Some(Block{
            header,
            body
        }))
    }

    pub fn get_header_by_number(&self, number: BlockNumber) -> Result<Option<BlockHeader>> {
        self.block_store.get_header_by_number(number)
    }

    pub fn get_body_by_number(&self, number: BlockNumber) -> Result<Option<BlockBody>> {
        match self.block_store.get_txns_by_number(number){
            Ok(Some(txns_hash)) => {
                let txns =match txns_hash.iter().map(|txn_hash|{
                    let txn:Result<SignedTransaction> = match self.get_txn_by_hash(txn_hash){
                        Ok(Some(txn)) => Ok(txn),
                        _=>Err(anyhow::anyhow!("txn not found")),
                    };
                    txn
                }).collect::<Result<Vec<SignedTransaction>>>(){
                    Ok(txns)=>txns,
                    Err(e)=>return Err(e),
                };

                Ok(Some(BlockBody{
                    transactions: txns
                }))
            }
            Ok(None) => return Ok(None),
            Err(e) => return Err(e),
        }
    }

    pub fn get_txns_hash_by_number(&self, number: BlockNumber) -> Result<Option<Vec<HashValue>>> {
        self.block_store.get_txns_by_number(number)
    }

    pub fn put_block(&self, number: BlockNumber, block: &Block) -> Result<()> {
        self.put_header (number, &block.header)?;
        self.put_body (number, &block.body)
    }

    pub fn put_header(&self, number: BlockNumber, header: &BlockHeader) -> Result<()> {
        self.block_store.put_header(number, header)
    }

    pub fn put_txns_hash(&self, number: BlockNumber, header: &Vec<HashValue>) -> Result<()> {
        self.block_store.put_txns_hash(number, header)
    }

    pub fn put_body(&self, number: BlockNumber, body: &BlockBody) -> Result<()> {
        let txns_hash = match body.transactions.iter().map(|txn|{
            match self.transaction_store.put_txn(&txn.hash(), txn){
                Ok(_) => Ok(txn.hash()),
                Err(e) => Err(e),
            }
        }).collect::<Result<Vec<HashValue>>>(){
            Ok(txns_hash)=>txns_hash,
            Err(e)=>return Err(e),
        };
        self.put_txns_hash(number, &txns_hash)
    }

    pub fn get_txn_by_hash(&self, hash: &HashValue) -> Result<Option<SignedTransaction>> {
        self.transaction_store.get_txn_by_hash(hash)
    }

    pub fn put_txn(&self, hash: &HashValue, txn: &SignedTransaction) -> Result<()> {
        self.transaction_store.put_txn(hash, txn)
    }
}

