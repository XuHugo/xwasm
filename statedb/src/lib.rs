use anyhow::{bail, Result};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use rusty_leveldb::{LdbIterator, Options, WriteBatch, DB};
use std::{
    io::{self, Write},
    path::Path,
    sync::Arc,
};

const STATE_PATH: &str = "db/statedb.db";

lazy_static! {
    pub static ref STATE_MEM: Arc<Mutex<State>> = Arc::new(Mutex::new(State {
        db: DB::open("statedb", rusty_leveldb::in_memory()).unwrap()
    }));
}

lazy_static! {
    pub static ref STATE_DB: Arc<Mutex<State>> = Arc::new(Mutex::new(State {
        db: DB::open(STATE_PATH, State::opts()).unwrap()
    }));
}

pub struct State {
    pub db: DB,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BatchOperation {
    Put(Vec<u8>, Vec<u8>),
    Delete(Vec<u8>),
}

#[allow(dead_code)]
impl State {
    pub fn new_mem() -> Result<Self> {
        let db = match DB::open("statedb", rusty_leveldb::in_memory()) {
            Ok(o) => o,
            Err(e) => bail!(e.to_string()),
        };
        Ok(State { db })
    }

    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let db =
            DB::open(path, Self::opts()).map_err(|error| anyhow::anyhow!(error.to_string()))?;
        Ok(Self { db })
    }
    fn new_db() -> Result<Self> {
        let db = match DB::open(STATE_PATH, State::opts()) {
            Ok(o) => o,
            Err(e) => bail!(e.to_string()),
        };
        Ok(State { db })
    }
    fn opts() -> Options {
        let opts = Options::default();
        opts
    }
    pub fn get(&mut self, k: &[u8]) -> Result<Vec<u8>> {
        match self.db.get(k) {
            Some(v) => return Ok(v),
            None => bail!("{:?}:<not found>", k),
        }
    }

    pub fn get_optional(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        self.db.get(key)
    }

    pub fn write_batch(
        &mut self,
        operations: impl IntoIterator<Item = BatchOperation>,
    ) -> Result<()> {
        let mut batch = WriteBatch::new();
        for operation in operations {
            match operation {
                BatchOperation::Put(key, value) => batch.put(&key, &value),
                BatchOperation::Delete(key) => batch.delete(&key),
            }
        }

        if batch.count() == 0 {
            return Ok(());
        }

        self.db.write(batch, true)?;
        Ok(())
    }

    pub fn put(&mut self, k: &[u8], v: &[u8]) -> Result<()> {
        match self.db.put(k, v) {
            Ok(_) => {
                match self.db.flush() {
                    Ok(_) => return Ok(()),
                    Err(e) => bail!(e.to_string()),
                };
            }
            Err(e) => bail!(e.to_string()),
        };
    }
    fn delete(&mut self, k: &[u8]) -> Result<()> {
        match self.db.delete(k) {
            Ok(_) => {
                match self.db.flush() {
                    Ok(_) => return Ok(()),
                    Err(e) => bail!(e.to_string()),
                };
            }
            Err(e) => bail!(e.to_string()),
        };
    }

    fn get_str(&mut self, k: &str) -> Result<String> {
        match self.db.get(k.as_bytes()) {
            Some(v) => match String::from_utf8(v.clone()) {
                Ok(v) => return Ok(v),
                Err(e) => bail!(e.to_string()),
            },
            None => bail!("{}:<not found>", k),
        }
    }

    fn put_str(&mut self, k: &str, v: &str) -> Result<()> {
        match self.db.put(k.as_bytes(), v.as_bytes()) {
            Ok(_) => {
                match self.db.flush() {
                    Ok(_) => return Ok(()),
                    Err(e) => bail!(e.to_string()),
                };
            }
            Err(e) => bail!(e.to_string()),
        };
    }

    fn delete_str(&mut self, k: &str) -> Result<()> {
        match self.db.delete(k.as_bytes()) {
            Ok(_) => {
                match self.db.flush() {
                    Ok(_) => return Ok(()),
                    Err(e) => bail!(e.to_string()),
                };
            }
            Err(e) => bail!(e.to_string()),
        };
    }

    fn iter(&mut self) {
        let mut it = self.db.new_iter().unwrap();
        let (mut k, mut v) = (vec![], vec![]);
        let mut out = io::BufWriter::new(io::stdout());
        while it.advance() {
            it.current(&mut k, &mut v);
            out.write_all(&k).unwrap();
            out.write_all(b" => ").unwrap();
            out.write_all(&v).unwrap();
            out.write_all(b"\n").unwrap();
        }
    }

    fn compact(&mut self, from: &str, to: &str) {
        self.db
            .compact_range(from.as_bytes(), to.as_bytes())
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    use crate::{BatchOperation, State};

    fn temp_state_path(label: &str) -> std::path::PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "xwasm-statedb-{label}-{}-{unique}",
            std::process::id()
        ))
    }
    #[test]
    fn state_mem() {
        let k = vec![1, 2, 3];
        let v = vec![4, 5, 6];
        let mut state = State::new_mem().unwrap();
        state.put(&k, &v).unwrap();
        let result = state.get(&k).unwrap();
        assert_eq!(result, v);
    }

    #[test]
    fn state_db() {
        let k = vec![1, 2, 3];
        let v = vec![4, 5, 6];
        let path = temp_state_path("basic");
        let mut state = State::open(&path).unwrap();
        state.put(&k, &v).unwrap();
        let result = state.get(&k).unwrap();
        assert_eq!(result, v);
        drop(state);
        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn state_batch_applies_puts_and_deletes() {
        let mut state = State::new_mem().unwrap();
        let first_key = b"first".to_vec();
        let second_key = b"second".to_vec();

        state
            .write_batch([
                BatchOperation::Put(first_key.clone(), vec![1]),
                BatchOperation::Put(second_key.clone(), vec![2]),
            ])
            .unwrap();
        assert_eq!(state.get_optional(&first_key), Some(vec![1]));
        assert_eq!(state.get_optional(&second_key), Some(vec![2]));

        state
            .write_batch([
                BatchOperation::Delete(first_key.clone()),
                BatchOperation::Put(second_key.clone(), vec![3]),
            ])
            .unwrap();
        assert_eq!(state.get_optional(&first_key), None);
        assert_eq!(state.get_optional(&second_key), Some(vec![3]));
    }

    #[test]
    fn state_batch_survives_reopen() {
        let path = temp_state_path("durable");

        {
            let mut state = State::open(&path).unwrap();
            state
                .write_batch([BatchOperation::Put(b"durable".to_vec(), vec![7])])
                .unwrap();
        }
        {
            let mut state = State::open(&path).unwrap();
            assert_eq!(state.get_optional(b"durable"), Some(vec![7]));
        }

        fs::remove_dir_all(path).unwrap();
    }
}
