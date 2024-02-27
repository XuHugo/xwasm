use crate::ColumnFamilyName;
use anyhow::Result;
use crypto::hash::HashValue;
use types::block::BlockNumber;
use std::fmt::Debug;


pub const TRANSACTION_BY_HASH_CF_NAME: ColumnFamilyName = "transaction_by_hash";

pub const BLOCK_BY_HASH_CF_NAME: ColumnFamilyName = "block_by_hash";
pub const BLOCK_HEADER_BY_HASH_CF_NAME: ColumnFamilyName = "block_header_by_hash";
pub const BLOCK_BODY_BY_HASH_CF_NAME: ColumnFamilyName = "block_body_by_hash";
pub const BLOCK_HASH_BY_NUMBER_CF_NAME: ColumnFamilyName = "block_hash_by_number";
pub const BLOCK_NUMBER_BY_HASH_CF_NAME: ColumnFamilyName = "block_number_by_hash";
pub const BLOCK_BY_NUMBER_CF_NAME: ColumnFamilyName = "block_by_number";
pub const BLOCK_HEADER_BY_NUMBER_CF_NAME: ColumnFamilyName = "block_header_by_number";
pub const BLOCK_BODY_BY_NUMBER_CF_NAME: ColumnFamilyName = "block_body_by_number";
pub const BLOCK_TXNS_BY_NUMBER_CF_NAME: ColumnFamilyName = "block_txns_by_number";


pub const DB_METADATA_CF_NAME: ColumnFamilyName = "db_metadata";
pub const EPOCH_BY_VERSION_CF_NAME: ColumnFamilyName = "epoch_by_version";
pub const EVENT_ACCUMULATOR_CF_NAME: ColumnFamilyName = "event_accumulator";
pub const EVENT_BY_KEY_CF_NAME: ColumnFamilyName = "event_by_key";
pub const EVENT_BY_VERSION_CF_NAME: ColumnFamilyName = "event_by_version";
pub const EVENT_CF_NAME: ColumnFamilyName = "event";
pub const JELLYFISH_MERKLE_NODE_CF_NAME: ColumnFamilyName = "jellyfish_merkle_node";
pub const LEDGER_INFO_CF_NAME: ColumnFamilyName = "ledger_info";
pub const STALE_NODE_INDEX_CF_NAME: ColumnFamilyName = "stale_node_index";
pub const STALE_NODE_INDEX_CROSS_EPOCH_CF_NAME: ColumnFamilyName = "stale_node_index_cross_epoch";
pub const STALE_STATE_VALUE_INDEX_CF_NAME: ColumnFamilyName = "stale_state_value_index";
pub const STATE_VALUE_CF_NAME: ColumnFamilyName = "state_value";
pub const TRANSACTION_CF_NAME: ColumnFamilyName = "transaction";
pub const TRANSACTION_ACCUMULATOR_CF_NAME: ColumnFamilyName = "transaction_accumulator";
pub const TRANSACTION_BY_ACCOUNT_CF_NAME: ColumnFamilyName = "transaction_by_account";
pub const TRANSACTION_INFO_CF_NAME: ColumnFamilyName = "transaction_info";
pub const VERSION_DATA_CF_NAME: ColumnFamilyName = "version_data";
pub const WRITE_SET_CF_NAME: ColumnFamilyName = "write_set";


#[macro_export]
macro_rules! define_schema {
    ($schema_type: ident, $key_type: ty, $value_type: ty, $cf_name: expr) => {
        #[derive(Debug)]
        pub(crate) struct $schema_type;

        impl $crate::schema::Schema for $schema_type {
            const COLUMN_FAMILY_NAME: $crate::ColumnFamilyName = $cf_name;
            type Key = $key_type;
            type Value = $value_type;
        }
    };
}

pub trait KeyCodec<S: Schema + ?Sized>: Sized + PartialEq + Debug {
    fn encode_key(&self) -> Result<Vec<u8>>;
    fn decode_key(data: &[u8]) -> Result<Self>;
}

pub trait ValueCodec<S: Schema + ?Sized>: Sized + PartialEq + Debug {
    fn encode_value(&self) -> Result<Vec<u8>>;
    fn decode_value(data: &[u8]) -> Result<Self>;
}

pub trait Schema: Debug + Send + Sync + 'static {
    const COLUMN_FAMILY_NAME: ColumnFamilyName;

    type Key: KeyCodec<Self>;
    type Value: ValueCodec<Self>;
}

impl<T:Schema> KeyCodec<T> for BlockNumber {
    fn encode_key(&self) -> Result<Vec<u8>> {
        todo!()
    }

    fn decode_key(data: &[u8]) -> Result<Self> {
        todo!()
    }
}

impl<T:Schema> ValueCodec<T> for BlockNumber {
    fn encode_value(&self) -> Result<Vec<u8>> {
        todo!()
    }

    fn decode_value(data: &[u8]) -> Result<Self> {
        todo!()
    }
}


impl<T:Schema> KeyCodec<T> for HashValue {
    fn encode_key(&self) -> Result<Vec<u8>> {
        todo!()
    }

    fn decode_key(data: &[u8]) -> Result<Self> {
        todo!()
    }
}

impl<T:Schema> ValueCodec<T> for HashValue {
    fn encode_value(&self) -> Result<Vec<u8>> {
        todo!()
    }

    fn decode_value(data: &[u8]) -> Result<Self> {
        todo!()
    }
}

impl<T:Schema> ValueCodec<T> for Vec<HashValue> {
    fn encode_value(&self) -> Result<Vec<u8>> {
        todo!()
    }

    fn decode_value(data: &[u8]) -> Result<Self> {
        todo!()
    }
}

