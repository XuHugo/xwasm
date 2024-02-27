
use std::sync::Arc;

use crypto::hash::HashValue;
use types::{
    block::{BlockNumber, BlockHeader, Block, BlockBody, GENESIS_BLOCK_NUMBER},
    transaction::SignedTransaction,
};
use db::DB;
use anyhow::Result;

use crate::schema_block::{BlockSchema, BlockHeaderSchema, BlockBodySchema, BlockNumberSchema, BlockTxnsSchema};


#[derive(Debug)]
pub struct BlockStore {
    db: Arc<DB>,
    //latest_ledger_info: ArcSwap<Option<LedgerInfoWithSignatures>>,
}


impl BlockStore {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            //latest_ledger_info: ArcSwap::from(Arc::new(ledger_info)),
        }
    }

    fn get_genesis(&self) -> Result<Option<Block>>{
        self.db.get::<BlockSchema>(&GENESIS_BLOCK_NUMBER)
    }

    pub fn get_block_by_number(&self, number: BlockNumber) -> Result<Option<Block>> {
        //self.db.get::<BlockSchema>(&number)
        let header = match self.get_header_by_number(number){
            Ok(Some(header)) => header,
            Ok(None) => return Ok(None),
            Err(e) => return Err(e),
        };
        let body = match self.get_body_by_number(number){
            Ok(Some(header)) => header,
            Ok(None) => return Ok(None),
            Err(e) => return Err(e),
        };
        Ok(Some(Block {
            header,
            body,
        }))
    }

    pub fn get_header_by_number(&self, number: BlockNumber) -> Result<Option<BlockHeader>> {
        self.db.get::<BlockHeaderSchema>(&number)
    }

    pub fn get_body_by_number(&self, number: BlockNumber) -> Result<Option<BlockBody>> {
        self.db.get::<BlockBodySchema>(&number)
    }

    pub fn get_txns_by_number(&self, number: BlockNumber) -> Result<Option<Vec<HashValue>>> {
        self.db.get::<BlockTxnsSchema>(&number)
    }

    pub fn get_block_by_hash(&self, hash: &HashValue) -> Result<Option<Block>> {
        match self.db.get::<BlockNumberSchema>(hash){
            Ok(Some(number)) => self.get_block_by_number(number),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn get_header_by_hash(&self, hash: &HashValue) -> Result<Option<BlockHeader>> {
        match self.db.get::<BlockNumberSchema>(hash){
            Ok(Some(number)) => self.get_header_by_number(number),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn get_body_by_hash(&self, hash: &HashValue) -> Result<Option<BlockBody>> {
        match self.db.get::<BlockNumberSchema>(hash){
            Ok(Some(number)) => self.get_body_by_number(number),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn get_txns_by_hash(&self, hash: &HashValue) -> Result<Option<Vec<HashValue>>> {
        match self.db.get::<BlockNumberSchema>(hash){
            Ok(Some(number)) => self.get_txns_by_number(number),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn put_header(&self, number: BlockNumber, header: &BlockHeader) -> Result<()> {
        self.db.put::<BlockHeaderSchema>(&number, header)
    }

    pub fn put_txns_hash(&self, number: BlockNumber, txns: &Vec<HashValue>) -> Result<()> {
        self.db.put::<BlockTxnsSchema>(&number, txns)
    }

}