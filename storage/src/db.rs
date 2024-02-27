
use anyhow::{format_err, Result};
pub use rocksdb::{
    ColumnFamilyDescriptor, DBCompressionType, Options,
};
use crate::schema::{KeyCodec, Schema, ValueCodec};
use crate::ColumnFamilyName;
use std::{collections::HashMap, iter::Iterator, path::Path};
use std::sync::Mutex;



#[derive(Debug)]
enum WriteOp {
    Value { key: Vec<u8>, value: Vec<u8> },
    Deletion { key: Vec<u8> },
}

#[derive(Debug)]
pub struct SchemaBatch {
    rows: Mutex<HashMap<ColumnFamilyName, Vec<WriteOp>>>,
}

impl Default for SchemaBatch {
    fn default() -> Self {
        Self {
            rows: Mutex::new(HashMap::new()),
        }
    }
}

impl SchemaBatch {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn put<S: Schema>(&self, key: &S::Key, value: &S::Value) -> Result<()> {
        let key = <S::Key as KeyCodec<S>>::encode_key(key)?;
        let value = <S::Value as ValueCodec<S>>::encode_value(value)?;
        self.rows
            .lock()
            .expect("Cannot currently handle a poisoned lock")
            .entry(S::COLUMN_FAMILY_NAME)
            .or_insert_with(Vec::new)
            .push(WriteOp::Value { key, value });

        Ok(())
    }

    /// Adds a delete operation to the batch.
    pub fn delete<S: Schema>(&self, key: &S::Key) -> Result<()> {
        let key = <S::Key as KeyCodec<S>>::encode_key(key)?;
        self.rows
            .lock()
            .expect("Cannot currently handle a poisoned lock")
            .entry(S::COLUMN_FAMILY_NAME)
            .or_insert_with(Vec::new)
            .push(WriteOp::Deletion { key });

        Ok(())
    }
}


#[derive(Debug)]
pub struct DB {
    name: &'static str, 
    inner: rocksdb::DB,
}

impl DB {
    pub fn open(
        path: impl AsRef<Path>,
        name: &'static str,
        column_families: Vec<ColumnFamilyName>,
        db_opts: &rocksdb::Options,
    ) -> Result<Self> {
        let db = DB::open_cf(
            db_opts,
            path,
            name,
            column_families
                .iter()
                .map(|cf_name| {
                    let mut cf_opts = rocksdb::Options::default();
                    cf_opts.set_compression_type(rocksdb::DBCompressionType::Lz4);
                    rocksdb::ColumnFamilyDescriptor::new((*cf_name).to_string(), cf_opts)
                })
                .collect(),
        )?;
        Ok(db)
    }

    pub fn open_cf(
        db_opts: &rocksdb::Options,
        path: impl AsRef<Path>,
        name: &'static str,
        cfds: Vec<rocksdb::ColumnFamilyDescriptor>,
    ) -> Result<DB> {
        let inner = rocksdb::DB::open_cf_descriptors(db_opts, path, cfds)?;
        Ok(DB { name, inner })
    }

    pub fn get<S: Schema>(&self, schema_key: &S::Key) -> Result<Option<S::Value>> {

        let k = <S::Key as KeyCodec<S>>::encode_key(schema_key)?;
        let cf_handle = self.get_cf_handle(S::COLUMN_FAMILY_NAME)?;

        let result = self.inner.get_cf(cf_handle, &k)?;

        result
            .map(|raw_value| <S::Value as ValueCodec<S>>::decode_value(&raw_value))
            .transpose()
    }

    pub fn put<S: Schema>(&self, key: &S::Key, value: &S::Value) -> Result<()> {
        // Not necessary to use a batch, but we'd like a central place to bump counters.
        // Used in tests only anyway.
        //pub_cf
        let batch = SchemaBatch::new();
        batch.put::<S>(key, value)?;
        self.write_schemas(batch)
    }

    pub fn write_schemas(&self, batch: SchemaBatch) -> Result<()> {
        let rows_locked = batch.rows.lock().expect("Cannot currently handle a poisoned lock");
        let mut db_batch = rocksdb::WriteBatch::default();
        for (cf_name, rows) in rows_locked.iter() {
            let cf_handle = self.get_cf_handle(cf_name)?;
            for write_op in rows {
                match write_op {
                    WriteOp::Value { key, value } => db_batch.put_cf(cf_handle, key, value),
                    WriteOp::Deletion { key } => db_batch.delete_cf(cf_handle, key),
                }
            }
        }

        self.inner.write_opt(db_batch, &default_write_options())?;

        Ok(())
    }

    fn get_cf_handle(&self, cf_name: &str) -> Result<&rocksdb::ColumnFamily> {
        self.inner.cf_handle(cf_name).ok_or_else(|| {
            format_err!(
                "DB::cf_handle not found for column family name: {}",
                cf_name
            )
        })
    }

}

fn default_write_options() -> rocksdb::WriteOptions {
    let mut opts = rocksdb::WriteOptions::default();
    opts.set_sync(true);
    opts
}
