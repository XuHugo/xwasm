use anyhow::{bail, Result};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use rusty_leveldb::{LdbIterator, Options, DB};
use std::{
    io::{self, Write},
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

#[allow(dead_code)]
impl State {
    fn new_mem() -> Result<Self> {
        let db = match DB::open("statedb", rusty_leveldb::in_memory()) {
            Ok(o) => o,
            Err(e) => bail!(e.to_string()),
        };
        Ok(State { db })
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
    use crate::State;

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
        let mut state = State::new_db().unwrap();
        state.put(&k, &v).unwrap();
        let result = state.get(&k).unwrap();
        assert_eq!(result, v);
    }
}
