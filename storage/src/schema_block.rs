
use crate::{define_schema, schema::{ValueCodec, BLOCK_NUMBER_BY_HASH_CF_NAME, BLOCK_BY_HASH_CF_NAME,BLOCK_HEADER_BY_HASH_CF_NAME, BLOCK_BODY_BY_HASH_CF_NAME, BLOCK_BY_NUMBER_CF_NAME, BLOCK_HEADER_BY_NUMBER_CF_NAME, BLOCK_BODY_BY_NUMBER_CF_NAME, BLOCK_TXNS_BY_NUMBER_CF_NAME}};
use types::block::{BlockHeader, BlockBody, Block, BlockNumber};
use crypto::hash::HashValue;

use anyhow::Result;

define_schema!(
    BlockNumberSchema, 
    HashValue,
    BlockNumber, 
    BLOCK_NUMBER_BY_HASH_CF_NAME
);

define_schema!(
    BlockSchema, 
    BlockNumber, 
    Block, 
    BLOCK_BY_NUMBER_CF_NAME
);

define_schema!(
    BlockHeaderSchema, 
    BlockNumber, 
    BlockHeader, 
    BLOCK_HEADER_BY_NUMBER_CF_NAME
);

define_schema!(
    BlockBodySchema, 
    BlockNumber, 
    BlockBody, 
    BLOCK_BODY_BY_NUMBER_CF_NAME
);

define_schema!(
    BlockTxnsSchema, 
    BlockNumber, 
    Vec<HashValue>, 
    BLOCK_TXNS_BY_NUMBER_CF_NAME
);

// define_schema!(
//     BlockSchema, 
//     HashValue, 
//     Block, 
//     BLOCK_BY_HASH_CF_NAME
// );

// define_schema!(
//     BlockHeaderSchema, 
//     HashValue, 
//     BlockHeader, 
//     BLOCK_HEADER_BY_HASH_CF_NAME
// );

// define_schema!(
//     BlockBodySchema, 
//     HashValue, 
//     BlockBody, 
//     BLOCK_BODY_BY_HASH_CF_NAME
// );




impl ValueCodec<BlockSchema> for Block {
    fn encode_value(&self) -> Result<Vec<u8>> {
        todo!()
    }

    fn decode_value(data: &[u8]) -> Result<Self> {
        todo!()
    }
}

impl ValueCodec<BlockHeaderSchema> for BlockHeader {
    fn encode_value(&self) -> Result<Vec<u8>> {
        todo!()
    }

    fn decode_value(data: &[u8]) -> Result<Self> {
        todo!()
    }
}

impl ValueCodec<BlockBodySchema> for BlockBody {
    fn encode_value(&self) -> Result<Vec<u8>> {
        todo!()
    }

    fn decode_value(data: &[u8]) -> Result<Self> {
        todo!()
    }
}


